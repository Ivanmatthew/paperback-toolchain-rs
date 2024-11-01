// use std::collections::HashMap;
// use std::default::Default;
// use anyhow::Error;
// use std::path::Path;
// use std::time;
// use swc_bundler::{Config, Load, ModuleData, ModuleRecord, Resolve};
// use swc_common::{sync::Lrc, FileName, FilePathMapping, Globals, SourceMap, Span, GLOBALS};
// use swc_common::errors::{ColorConfig, Handler};
// use swc_ecma_ast::{Bool, EsVersion, Expr, IdentName, KeyValueProp, Lit, MemberExpr, MemberProp, MetaPropExpr, MetaPropKind, PropName, Str};
// use swc_ecma_loader::resolvers::lru::CachingResolver;
// use swc_ecma_loader::resolvers::node::NodeModulesResolver;
// use swc_ecma_loader::TargetEnv;
// use swc_ecma_parser::{parse_file_as_module, parse_file_as_program, Syntax, TsConfig, TsSyntax};
//
// pub struct bundler;
//
// impl bundler {
//     pub fn new() -> Self {
//         Self
//     }
//
//     pub fn bundle(&self, entry_points: HashMap<String, &Path>) -> bool {
//         // TODO: Bundle extension
//         let globals = Box::leak(Box::new(Globals::default()));
//         let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
//         // let external_modules = Vec::new();
//         let mut bundler = swc_bundler::Bundler::new(
//             &globals,
//             cm.clone(),
//             PathLoader { cm: cm.clone() },
//             // CachingResolver::new(
//             //     4096,
//             NodeModulesResolver::new(
//                 TargetEnv::Browser,
//                 Default::default(),
//                 true
//             ),
//             // ),
//             Config {
//                 require: false,
//                 disable_inliner: true,
//                 external_modules: Default::default(),
//                 disable_fixer: true,
//                 disable_hygiene: true,
//                 disable_dce: true,
//                 module: Default::default(),
//                 // require: true,
//                 // external_modules,
//                 // ..Default::default()
//             },
//             Box::new(Hook),
//         );
//
//         let start = time::Instant::now();
//         let modules = bundler
//             .bundle(entry_points)
//             .map_err(|err| {
//                 println!("{:?}", err);
//             });
//         // .unwrap_or_else(|err| {
//         //     println!("{:?}", err);
//         //     panic!("failed to bundle")
//         // });
//
//         {
//             let elapsed = start.elapsed();
//             println!("Bundled {} modules in {}ms", modules.unwrap().len(), elapsed.as_millis());
//         }
//
//         true
//     }
// }
//
// struct PathLoader {
//     pub cm: Lrc<SourceMap>,
// }
// impl Load for PathLoader {
//     fn load(&self, f: &FileName) -> Result<ModuleData, Error> {
//         let fm = match f {
//             FileName::Real(path) => self.cm.load_file(path)?,
//             e => Err(anyhow::anyhow!("unimplemented: load({:?})", e))?,
//             // _ => unreachable!(),
//         };
//
//         println!("Loading: {:?}", fm);
//
//         let module = parse_file_as_module(
//             &fm,
//             //Syntax::Es(Default::default()),
//             Syntax::Typescript(TsSyntax {
//                 tsx: false,
//                 decorators: true,
//                 dts: false,
//                 no_early_errors: false,
//                 disallow_ambiguous_jsx_like: true,
//             }),
//             EsVersion::Es2015,
//             None,
//             &mut vec![],
//         )
//             .unwrap_or_else(|err| {
//                 let handler =
//                     Handler::with_tty_emitter(ColorConfig::Always, false, false, Some(self.cm.clone()));
//                 err.into_diagnostic(&handler).emit();
//                 panic!("failed to parse")
//             });
//
//         println!("Done!");
//
//         // Javascript:
//         // const configPath = join(process.cwd(), "bench.config.js");
//         // //const config = (await import(configPath)).default;
//         // above commented out because it will not work on windows
//         // const config = require(configPath).default;
//
//         Ok(ModuleData {
//             fm,
//             module,
//             helpers: Default::default(),
//         })
//     }
// }
//
// struct Hook;
//
// // what this does is it adds the import.meta.url and import.meta.main properties to the module
// impl swc_bundler::Hook for Hook {
//     fn get_import_meta_props(
//         &self,
//         span: Span,
//         module_record: &ModuleRecord,
//     ) -> Result<Vec<KeyValueProp>, Error> {
//         let file_name = module_record.file_name.to_string();
//
//         Ok(vec![
//             KeyValueProp {
//                 key: PropName::Ident(IdentName::new("url".into(), span)),
//                 value: Box::new(Expr::Lit(Lit::Str(Str {
//                     span,
//                     raw: None,
//                     value: file_name.into(),
//                 }))),
//             },
//             KeyValueProp {
//                 key: PropName::Ident(IdentName::new("main".into(), span)),
//                 value: Box::new(if module_record.is_entry {
//                     Expr::Member(MemberExpr {
//                         span,
//                         obj: Box::new(Expr::MetaProp(MetaPropExpr {
//                             span,
//                             kind: MetaPropKind::ImportMeta,
//                         })),
//                         prop: MemberProp::Ident(IdentName::new("main".into(), span)),
//                     })
//                 } else {
//                     Expr::Lit(Lit::Bool(Bool { span, value: false }))
//                 }),
//             },
//         ])
//     }
// }