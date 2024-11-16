// Module declarations
mod migrator;
mod bundler;
mod util;

// External crate imports
use std::{fs, time};
use std::io::Write;
use std::path::PathBuf;
use clap::{arg, Command};

// Internal crate imports (modules)

const APP_NAME: &str = if cfg!(feature = "drop-in") {
    "paperback-cli"
} else {
    "pbtc"
};

fn cli() -> Command {
    Command::new(APP_NAME)
        .version("0.1.0")
        .author("Ivanmatthew")
        .about("A CLI for creating and managing paperback extensions")
        .subcommands(
            [
                Command::new("migrate")
                    .about("Migrate your 0.8 extension to 0.9")
                    .arg(arg!(-r --recursive "Recursively migrate all subdirectories."))
                    .arg(arg!(<dir> "Target directory containing your extension."))
                    .arg(arg!(<output> "Output directory for the migrated extension.")),
                Command::new("new")
                    .about("Create a new extension.")
                    .arg(arg!(<name> "Name of the extension.")),
                Command::new("bundle")
                    .about("Build your extension.")
                    // TODO: Set to optional and use current directory
                    .arg(arg!(<dir> "Target directory containing your extension. Uses current directory if not provided.")),
                Command::new("serve")
                    .about("Serve your extension.")
                    .arg(arg!([dir] "Target directory containing your extension. Uses current directory if not provided.")),
                Command::new("logcat")
                    .about("Use the grpc protocol to retrieve realtime logs from the paperback app.")
                    .arg(arg!([dir] "Target directory containing your extension. Uses current directory if not provided.")),
                Command::new("debug")
                    .about("Rebuild and use the grpc protocol to automatically load in your extension on your paperback app device.")
                    .arg(arg!(-w --watch "Automatically rebuild and upload extension on detected code changes."))
                    .arg(arg!(<extension> "Name of the extension."))
                    .arg(arg!(<host> "Host to use for the grpc server."))
                    .arg(arg!([dir] "Target directory containing your extension. Uses current directory if not provided.")),
            ]
        )
}

fn ask(question: &str) -> String {
    print!("{} ", question);
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("migrate", sub_matches)) => {
            todo!("Logic is half-baked, need to finish this by creating a javascript with d.ts file acting as a compatibility layer, where direct AST manipulation is happening on some parts of the code.");
            let migrator = migrator::Migrator::new();
            if (!migrator.is_migratable(&PathBuf::from(sub_matches.get_one::<String>("dir").unwrap()))) {
                println!("Unable to migrate.");
                return;
            }

            let iter_now = time::Instant::now();
            for entry in fs::read_dir(&PathBuf::from(sub_matches.get_one::<String>("dir").unwrap()).join("src")).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    if sub_matches.contains_id("recursive") {
                        migrator.migrate(&path, sub_matches.get_one::<String>("output").unwrap());
                    }
                }
            }
            println!("Recursive analysis completed in {}ms", iter_now.elapsed().as_millis());
        }
        Some(("new", _)) => {
            println!("Creating new extension...");
            let name = ask("Enter the name of the extension:");
            let author = ask("Enter the author of the extension:");
            let description = ask("Enter the description of the extension:");

            println!("Creating extension with name: {}, author: {}, description: {}", name, author, description);
            let is_correct = ask("Is this correct? (y/N)");
            if is_correct.to_lowercase() == "y" {
                println!("Creating extension...");
            } else {
                println!("Aborting...");
            }
        }
        Some(("bundle", sub_matches)) => {
            println!("Bundling extension...");
            let bundler = bundler::Bundler::new();
            let start = time::Instant::now();
            let bundled_successfully = bundler.bundle(&PathBuf::from(sub_matches.get_one::<String>("dir").unwrap())).await;
            if bundled_successfully {
                println!("Bundled extension in {}ms", start.elapsed().as_millis());
                println!("Extension bundled successfully.");
            } else {
                println!("Failed to bundle extension.");
            }
        }
        Some(("serve", _)) => {
            todo!("Implement http server for serving extension.");
            println!("Serving extension...");
        }
        Some(("logcat", _)) => {
            todo!("Implement grpc client for retrieving logs.");
            println!("Retrieving logs...");
        }
        Some(("debug", _)) => {
            todo!("Implement grpc client for uploading extension and retrieving logs (ratatui).");
            println!("Debugging extension...");
        }
        _ => {
            cli().print_help().unwrap();
        }
    }
}
