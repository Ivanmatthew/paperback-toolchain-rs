use std::collections::HashMap;
use std::path::PathBuf;

pub trait bundler {
    fn new(entry_points: HashMap<PathBuf, PathBuf>) -> Self;
    async fn bundle(&mut self) -> bool;
}

// pub mod swc_bundler;
// pub mod mako_bundler;
// pub mod rspack_bundler;
// pub mod esbuildrs_bundler;
// pub mod esbuild_bundler;
pub mod rolldown_bundler;