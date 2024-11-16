// use std::collections::HashMap;
// use std::io::Read;
// use std::path::{Path, PathBuf};
// use std::process::Command;
// use crate::bundler::bundler_programs::bundler;
// use crate::util::caching::{DiskCache, DiskCacheTrait};
//
// pub struct ESBuildBundler {
//     esbuild_bin: String,
// }
//
// fn pacman_detect() -> Vec<String> {
//     let mut pacmans = Vec::new();
//     if Command::new("bun").arg("--version").output().is_ok() {
//         pacmans.push(String::from("bun"));
//     }
//     // if Command::new("pnpm").arg("--version").spawn().is_ok() {
//     //     pacmans.push(String::from("pnpm"));
//     // }
//     // if Command::new("yarn").arg("--version").spawn().is_ok() {
//     //     pacmans.push(String::from("yarn"));
//     // }
//     #[cfg(windows)]
//     let mut binding = Command::new("npm.cmd");
//     #[cfg(not(windows))]
//     let mut binding = Command::new("npm");
//     let mut npm_cmd = binding.arg("--version");
//     let result = npm_cmd.output();
//     if result.is_ok() {
//         pacmans.push(String::from("npm"));
//     } else {
//         // list error
//         let error = result.unwrap_err();
//         println!("Error: {}", error);
//     }
//
//     pacmans
// }
//
// fn esbuild_bin_detect() -> Vec<String> {
//     let cache_path = Path::new(file!()).parent().unwrap().join("esbuild_cache");
//     let mut cache = DiskCache::new(&cache_path).unwrap();
//
//     let cached_esbuild_bin = cache.load();
//     match cached_esbuild_bin {
//         Ok(cached_esbuild_bin) => {
//             println!("Loaded cached esbuild binary: {}", cached_esbuild_bin);
//             if cached_esbuild_bin.len() > 0 {
//                 return vec![cached_esbuild_bin];
//             }
//         }
//         Err(err) => {
//             // do nothing
//         }
//     }
//
//
//     let mut pacmans = pacman_detect();
//     let mut esbuild_bins = Vec::new();
//     let translation_table = [
//         ("bun", "bunx"),
//         ("npm", "npx"),
//     ];
//     for pacman in pacmans {
//         let mut command = match pacman.as_str() {
//             "bun" => Command::new("bunx"),
//             #[cfg(windows)]
//             "npm" => Command::new("npx.cmd"),
//             #[cfg(not(windows))]
//             "npm" => "npx",
//             _ => Command::new("npx")
//         };
//         let pacman = translation_table.iter().find(|(pacman_name, _)| *pacman_name == pacman).unwrap();
//         let pacman = pacman.1;
//         if pacman == "bunx" {
//             command.arg("esbuild").arg("--version");
//         } else if pacman == "npx" {
//             command.arg("--no").arg("esbuild").arg("--").arg("--version");
//         }
//         let command_result = command.output();
//         if command_result.is_ok() {
//             esbuild_bins.push(pacman.to_string());
//         } else {
//             println!("esbuild not found in {pacman}");
//             let err = command_result.unwrap_err();
//             println!("Error: {}", err);
//         }
//     }
//
//     if esbuild_bins.len() > 0 {
//         let cache_result = cache.store(&esbuild_bins[0]);
//         if cache_result.is_err() {
//             println!("Warning: Detected esbuild has not been cached due to error: '{}'", cache_result.unwrap_err());
//         }
//     }
//
//     esbuild_bins
// }
//
// impl bundler for ESBuildBundler {
//     fn new() -> Self {
//         let esbuild_bins = esbuild_bin_detect();
//         if esbuild_bins.len() == 0 {
//             println!("Error: No esbuild binaries found.");
//             todo!("Handle this error, implement an esbuild installer helper.");
//         }
//         println!("Using {}", esbuild_bins[0]);
//
//         Self {
//             esbuild_bin: esbuild_bins[0].clone()
//         }
//     }
//
//     fn bundle(&self, entry_points: HashMap<PathBuf, PathBuf>) -> bool {
//         let mut entry_points_cmdline = Vec::new();
//         for (dir_name_full_path, entry_point_full_path) in &entry_points {
//             let dir_name = dir_name_full_path.file_name().unwrap().to_str().unwrap();
//             let entry_point_full_path = entry_point_full_path.to_str().unwrap();
//             // TODO: Find a better way to verify if the entry point is valid path and isn't an unescaping string
//             entry_points_cmdline.push(format!("\"{dir_name}\"=\"{entry_point_full_path}\""));
//         }
//
//         let esbuild_bin_str = self.esbuild_bin.clone();
//         let _ = std::env::set_current_dir(entry_points.keys().next().unwrap());
//         let mut cmd = Command::new(esbuild_bin_str);
//         cmd.args((vec! [
//             "esbuild",
//             "--bundle",
//             "--outdir=dist",
//             "--format=iife",
//             "--global-name=sources",
//             "--metafile=true"
//         ]));
//         for entry_point in entry_points_cmdline {
//             cmd.arg(entry_point);
//         }
//         let cmd_result = cmd.spawn();
//
//         match cmd_result {
//             Err(err) => {
//                 println!("Error: {}", err);
//                 return false;
//             }
//             Ok(mut child) => {
//                 println!("Bundling successful.");
//             }
//         }
//
//         true
//     }
// }