use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=static/");
    println!("cargo::rerun-if-changed=tailwind.config.js");

    if Path::new("./node_modules").exists() {
        let status = Command::new("npm")
            .args(&["install"])
            .status()
            .expect("Failed to install npm dependencies");
        assert!(status.success(), "Failed to install npm dependencies.")
    }

    let status = Command::new("npm")
        .args(&["run", "build:css"])
        .status()
        .expect("Failed to run Tailwind build.");
    assert!(status.success(), "Tailwind build failed.");
}
