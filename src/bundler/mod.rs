use std::collections::HashMap;
use std::path::Path;

mod bundler_programs;

// use bundler_programs::swc_bundler::bundler;
// use bundler_programs::mako_bundler::bundler;
// use bundler_programs::esbuildrs_bundler::bundler;
use bundler_programs::esbuild_bundler::ESBuildBundler;
use crate::bundler::bundler_programs::bundler;

pub struct Bundler;

impl Bundler {
    pub fn new() -> Self {
        Self
    }

    pub fn bundle(&self, extension_dir: &Path) -> bool {
        // TODO: Verifying correct extension

        let mut entry_points = HashMap::new();
        if !extension_dir.join("src").exists() {
            println!("No src folder found in extension directory.");
            return false;
        }
        let src_dir = extension_dir.join("src");
        for entry in std::fs::read_dir(&src_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let folder = path.clone();
                // check if main.ts exists in the folder
                if path.join("main.ts").exists() {
                    entry_points.insert(folder, path.join("main.ts"));
                } else {
                    println!("No main.ts found in folder: {}, skipping.", folder.display());
                }
            }
        }
        // TODO: Bundle extension
        // let bundler = SwcBundler::new();
        // swc_bundler.bundle(&entry_points);
        let bundler = ESBuildBundler::new();
        bundler.bundle(entry_points)
    }
}
