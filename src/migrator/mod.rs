// basic module for adding, subtracting, and multiplying two numbers

// use std::path::{PathBuf};

// pub data struct
// pub struct Extension {
//     pub name: String,
//     pub version: String,
//     pub path: PathBuf,
// }
// pub trait Migrator {
//     fn migrate(&self, dir: &str, output: &str, recursive: &bool);
//     fn get_extension_version(&self, version: &str);
//     fn get_extensions(&self, dir: &str);
// }

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use oxc::allocator::Allocator;
use oxc::ast::{ast::{Class, Function, TSImportType}, visit::walk_mut, visit::walk, Visit, VisitMut};
use oxc::ast::ast::{ImportDefaultSpecifier, ImportSpecifier, MethodDefinition, PropertyDefinition};
use oxc::codegen::{CodeGenerator};
use sonic_rs::{get, JsonValueTrait, Serialize};
use oxc::parser::Parser;
use oxc::span::{SourceType, Span};
use oxc::syntax::scope::ScopeFlags;

#[derive(Debug, Clone)]
struct InvalidExtensionError;
impl std::fmt::Display for InvalidExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid extension")
    }
}

#[derive(Debug)]
struct NodeMeta {
    r#type: String,
    name: Option<String>,
    span: Span
}
#[derive(Debug, Default)]
struct ExtensionASTNodes {
    functions: Vec<NodeMeta>,
    properties: Vec<NodeMeta>
}
impl<'a> VisitMut<'a> for ExtensionASTNodes{
    fn visit_method_definition(&mut self, it: &mut MethodDefinition<'a>) {
        let function_name: String = it.key.name().unwrap().to_string();
        self.functions.push(NodeMeta {
            r#type: "function".to_string(),
            name: Some(function_name),
            span: it.span
        });
        walk_mut::walk_method_definition(self, it);
    }

    fn visit_property_definition(&mut self, it: &mut PropertyDefinition<'a>) {
        let property_name: String = it.key.name().unwrap().to_string();
        self.properties.push(NodeMeta {
            r#type: "property".to_string(),
            name: Some(property_name),
            span: it.span
        });
        walk_mut::walk_property_definition(self, it);
    }
}

pub struct Migrator;

impl Migrator {
    pub fn new() -> Self {
        Migrator
    }
    pub fn migrate(&self, dir: &PathBuf, output: &str) {
        let allocator = Allocator::default();
        let entrypoint_name = dir.file_name().unwrap().to_str().unwrap().to_owned() + ".ts";
        let dummy_actual_path = dir.join(entrypoint_name);
        // display only the last five parts of the path
        let components: Vec<&str> = dummy_actual_path
            .components()
            .map(|comp| comp.as_os_str().to_str().unwrap_or(""))
            .collect();
        let shortened_path = components.iter().rev().take(5).rev().collect::<PathBuf>();
        println!("Parsing file: {}", shortened_path.display());
        let parser_now = std::time::Instant::now();
        let dummy_actual_path_str = &fs::read_to_string(&dummy_actual_path).unwrap();
        let mut parser = Parser::new(&allocator, dummy_actual_path_str, SourceType::from_path(&dummy_actual_path).unwrap()).parse();
        let parse_elapsed = parser_now.elapsed();
        println!("Parsed in {:.3}ms", parse_elapsed.as_micros() as f32/1000f32);

        // walk the AST tree to collect
        let mut extension_ast_nodes = ExtensionASTNodes::default();
        let mut collect_now = std::time::Instant::now();
        walk_mut::walk_program(&mut extension_ast_nodes, &mut parser.program);
        let collect_elapsed = collect_now.elapsed();
        println!("Collected in {:.3}ms", collect_elapsed.as_micros() as f32/1000f32);
        println!(
            "Counted Functions: {}",
            extension_ast_nodes.functions.len(),
        );

        // for each function, get the line number and the function name.
        let mut concat = vec![];
        concat.append(&mut extension_ast_nodes.functions);
        concat.append(&mut extension_ast_nodes.properties);
        for node_meta in concat {
            let start = node_meta.span.start as usize; // character number
            let end = node_meta.span.end; // character number
            let node_meta_name = node_meta.name.unwrap();
            let line_number = dummy_actual_path_str[..start].lines().count();
            println!("{}: {:<35} at line {}", node_meta.r#type[0..1].to_uppercase() + &node_meta.r#type[1..], node_meta_name, line_number);
        }
    }

    pub fn is_migratable(&self, extension_dir: &Path) -> bool {
        let extension_versions = self._get_extension_versions(extension_dir);

        if extension_versions.is_err() {
            println!("Invalid extension");
            false;
        }

        let (ext_version, app_version) = extension_versions.unwrap();
        if app_version == "0.9" {
            println!("Extension is already ready for app version 0.9");
            false;
        }

        true
    }

    /// Returns the extension version and the app version in order.
    /// TODO: Change ambiguous naming of function (we are getting the toolchain version for the app version)
    fn _get_extension_versions(&self, extension_dir: &Path) -> Result<(String, String), InvalidExtensionError> {
        let version_translation_vec = HashMap::from([
            // toolchain version => app version
            ("0.8", "8.0"),
            ("1.0", "0.9")
        ]);

        let package_json_str = fs::read_to_string(extension_dir.join("package.json")).unwrap();
        let dev_dependencies = get(&package_json_str, ["devDependencies"]).unwrap();
        let dependencies = get(&package_json_str, ["dependencies"]).unwrap();

        let found_dev_dependency = dev_dependencies.get("@paperback/toolchain");
        let found_dependency = dependencies.get("@paperback/toolchain");

        if found_dev_dependency.is_none() && found_dependency.is_none() {
            false;
        }

        let version = if found_dev_dependency.is_some() {
            found_dev_dependency.unwrap()
        } else {
            found_dependency.unwrap()
        };
        let ext_version = version.as_str().unwrap();

        // match version on partial string
        for (toolchain_version, app_version) in version_translation_vec {
            if ext_version.contains(toolchain_version) {
                println!("Detected used Toolchain version: {}. App version: {}", ext_version, app_version);

                return Ok((ext_version.to_string(), app_version.to_string()));
            }
        }

        Err(InvalidExtensionError)
    }
    pub fn is_root_dir(&self, dir: &Path) -> bool {
        if dir.join("package.json").exists() && dir.join("src").is_dir() {
            true;
        }
        false
    }
    pub fn get_extensions(&self, dir: &Path) {
        println!("Getting extensions...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}