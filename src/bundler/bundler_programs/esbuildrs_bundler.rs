// use std::collections::HashMap;
// use std::path::{Path, PathBuf};
// use async_std::task;
// use esbuild_rs::{BuildOptionsBuilder, build, BuildResult, Target, Platform, Engine, Format, EntryPoint, TreeShaking};
//
// pub struct bundler;
//
// impl bundler {
//     pub fn new() -> Self {
//         Self
//     }
//
//     pub fn bundle(&self, entry_points: HashMap<PathBuf, PathBuf>) -> bool {
//         for (dir, entry_point) in entry_points {
//             println!("Bundling extension: {} with entrypoint: {}", dir.display(), entry_point.display());
//             let outdir = dir.join("dist");
//             if !outdir.exists() {
//                 std::fs::create_dir(&outdir).unwrap();
//             }
//
//             let mut options_builder = BuildOptionsBuilder::new();
//             options_builder.abs_working_dir = dir.to_str().unwrap().to_string();
//             options_builder.entry_points_advanced.push(EntryPoint {
//                 // input_path: entry_point.to_str().unwrap().to_string(),
//                 // output_path: outdir.join("index").to_str().unwrap().to_string(),
//                 input_path: "main.ts".to_string(),
//                 output_path: "dist/index".to_string()
//             });
//             options_builder.tsconfig = dir.join("tsconfig.json").to_str().unwrap().to_string();
//             options_builder.tree_shaking = TreeShaking::True;
//             options_builder.bundle = true;
//             options_builder.main_fields.push("module".to_string());
//             options_builder.platform = Platform::Browser;
//             options_builder.target = Target::ES2020;
//             options_builder.format = Format::IIFE;
//             options_builder.outdir = outdir.to_str().unwrap().to_string();
//             let options = options_builder.build();
//             let res = task::block_on(build(options));
//             if res.errors.as_slice().len() > 0 {
//                 let res_error_messages = res.errors.as_slice().iter().map(|e| e.text.as_str().to_string()).collect::<Vec<String>>();
//                 println!("Failed to bundle extension: {}", dir.display());
//                 println!("{:?}", res_error_messages);
//                 return false;
//             }
//             if res.warnings.as_slice().len() > 0 {
//                 let res_warning_messages = res.warnings.as_slice().iter().map(|e| e.text.as_str().to_string()).collect::<Vec<String>>();
//                 println!("Warnings while bundling extension: {}", dir.display());
//                 println!("{:?}", res_warning_messages);
//             }
//             for f in res.output_files.as_slice() {
//                 println!("Out: {} ({} bytes)", f.path.as_str(), f.data.as_str().len() / 1024);
//             }
//             println!("Bundled extension: {}", dir.display());
//         }
//
//         true
//     }
// }