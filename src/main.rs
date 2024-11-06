use serde_json::Value;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run `cargo metadata` and capture the output
    let output = Command::new("cargo")
        .args(&["metadata", "--no-deps", "--format-version", "1"])
        .output()?
        .stdout;
    let metadata: Value = serde_json::from_slice(&output)?;

    // Collect root paths for packages in the workspace
    let mut paths_to_include = vec![];

    if let Some(packages) = metadata["packages"].as_array() {
        for package in packages {
            if let Some(path) = package["manifest_path"].as_str() {
                // Add Cargo.toml and source directory
                let manifest_path = PathBuf::from(path);
                let package_dir = manifest_path.parent().unwrap();
                paths_to_include.push(package_dir.join("Cargo.toml"));

                // Add src/ directory if it exists
                let src_path = package_dir.join("src");
                if src_path.exists() {
                    paths_to_include.push(src_path);
                }

                // Check for other directories like assets/, tests/, etc.
                let extra_dirs = ["assets", "tests", "benches", "examples"];
                for dir in extra_dirs.iter() {
                    let extra_path = package_dir.join(dir);
                    if extra_path.exists() {
                        paths_to_include.push(extra_path);
                    }
                }
            }
        }
    }

    // List out the paths to be included
    println!("Files and directories to include:");
    for path in paths_to_include {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                println!("{}", entry?.path().display());
            }
        } else {
            println!("{}", path.display());
        }
    }

    Ok(())
}