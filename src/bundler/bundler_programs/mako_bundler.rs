// use std::collections::HashMap;
// use std::path::{Path, PathBuf};
// use mako::compiler::{Compiler, Args};
// use mako::config::Config;
//
// pub struct bundler;
//
// impl bundler {
//     pub fn new() -> Self {
//         Self
//     }
//
//     pub fn bundle(&self, entry_points: HashMap<String, &Path>) -> bool {
//         for (dir_name, entry_point) in entry_points {
//             let config = Config::new(entry_point, None, None);
//             let compiler_result = Compiler::new(
//                 config.unwrap(),
//                 PathBuf::from(entry_point),
//                 Args {
//                     watch: false,
//                 },
//                 None,
//             );
//
//             let compiler = compiler_result.unwrap();
//             let compile_result = compiler.compile();
//
//             if compile_result.is_err() {
//                 println!("Failed to compile extension: {}", dir_name);
//                 return false;
//             }
//             println!("Compiled extension: {}", dir_name);
//         }
//
//         true
//     }
// }