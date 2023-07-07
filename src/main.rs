use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[clap(author = "InfiniteCoder", version)]
/// Project generator for raylib with web support
enum Cli {
    /// Creates a new rust project like cargo new does, adds raylib as a dependency, creates Assets folder, adds Makefile, .cargo/config.toml and index.html. Requires cargo to be installed.
    New { name: String },

    /// Integrates Makefile, .cargo/config.toml and index.html into existing project.
    Integrate,

    /// Runs your project on web. Requires python3 and make to be installed.
    Run,
}

pub fn integrate() {
    if !std::path::Path::new("Assets").exists() {
        std::fs::create_dir("Assets").expect("Failed to create Assets folder.");
    }

    if !std::path::Path::new("Makefile").exists() {
        std::fs::write("Makefile", include_str!("template/Makefile"))
            .expect("Failed to create makefile.");
    }

    if !std::path::Path::new("html").exists() {
        std::fs::create_dir("html").expect("Failed to create html folder.");
    }

    if !std::path::Path::new("html/index.html").exists() {
        std::fs::write(
            "html/index.html",
            include_str!("template/index.html").replace(
                "$PROJECT_NAME",
                std::env::current_dir()
                    .expect("Failed to fetch project name.")
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ),
        )
        .expect("Failed to create index.html.");
    }

    if !std::path::Path::new(".cargo").exists() {
        std::fs::create_dir(".cargo").expect("Failed to create .cargo folder.");
    }

    if !std::path::Path::new(".cargo/config.toml").exists() {
        std::fs::write(".cargo/config.toml", include_str!("template/config.toml"))
            .expect("Failed to create config.toml.");
    }
}

fn main() {
    match Cli::parse() {
        Cli::New { name } => {
            assert!(
                Command::new("cargo")
                    .arg("new")
                    .arg(&name)
                    .status()
                    .expect("Failed to create new project.")
                    .success(),
                "Failed to create new project."
            );

            std::env::set_current_dir(std::path::Path::new(&name))
                .expect("Failed to jump to project's directory.");

            assert!(
                Command::new("cargo")
                    .arg("add")
                    .arg("raylib")
                    .status()
                    .expect("Failed to add raylib as a dependency.")
                    .success(),
                "Failed to add raylib as a dependency."
            );

            std::fs::write("src/main.rs", include_str!("template/main.rs"))
                .expect("Failed to create config.toml.");

            integrate();
        }
        Cli::Integrate => integrate(),
        Cli::Run => {
            assert!(
                Command::new("make")
                    .status()
                    .expect("Failed to build project.")
                    .success(),
                "Failed to build project."
            );

            std::env::set_current_dir(std::path::Path::new("html"))
                .expect("Failed to jump to project's html.");

            if Command::new("python3")
                .arg("-m")
                .arg("http.server")
                .arg("8000")
                .status()
                .is_err()
            {
                assert!(
                    Command::new("python")
                        .arg("-m")
                        .arg("http.server")
                        .arg("8000")
                        .status()
                        .expect("Failed to run project.")
                        .success(),
                    "Failed to run project."
                );
            }
        }
    }
}
