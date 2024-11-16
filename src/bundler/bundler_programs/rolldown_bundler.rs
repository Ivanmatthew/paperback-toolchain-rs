use std::collections::HashMap;
use std::path::{PathBuf};
use crate::bundler::bundler_programs::bundler;
use rolldown::{Bundler, BundlerOptions, BundlerBuilder, InputItem, SourceMapType, Platform};
use sugar_path::SugarPath;

pub struct RolldownBundler {
    rolldown_bundler: Bundler
}

impl bundler for RolldownBundler {
    fn new(entry_points: HashMap<PathBuf, PathBuf>) -> Self {
        let input_items = entry_points.iter().map(|(dir_name_full_path, entry_point_full_path)| {
            InputItem {
                name: Some(dir_name_full_path.file_name().unwrap().to_str().unwrap().to_string()),
                import: entry_point_full_path.normalize().to_str().unwrap().to_string(),
            }
        }).collect::<Vec<_>>();

        Self {
            rolldown_bundler: BundlerBuilder::default().with_options(BundlerOptions {
                input: Some(input_items),
                cwd: Some(std::env::current_dir().unwrap().normalize()),
                sourcemap: Some(SourceMapType::File),
                platform: Some(Platform::Browser),
                ..Default::default()
            }).build()
        }
    }

    async fn bundle(&mut self) -> bool {
        let start_time = tokio::time::Instant::now();
        let bundle_output = self.rolldown_bundler.write().await;
        let elapsed_time = start_time.elapsed().as_millis();

        match bundle_output {
            Ok(bundle_output) => {
                for warning in bundle_output.warnings {
                    println!("Warning: {}", warning);
                }

                if bundle_output.errors.len() > 0 {
                    println!("Failed to bundle extension.");

                    for error in bundle_output.errors {
                        println!("Error: {}", error);
                    }

                    return false;
                }

                if bundle_output.assets.len() == 0 {
                    println!("No assets generated.");
                } else {
                    for asset in bundle_output.assets {
                        println!("Generated asset: {}", asset.filename());
                    }
                }

                println!("Bundled extension in {}ms", elapsed_time);

                true
            }
            Err(bundle_err) => {
                println!("Failed to bundle extension.\n\tError: {}", bundle_err);
                false
            }
        }
    }
}