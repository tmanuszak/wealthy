use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=static");

    print!("Building tailwind stylesheet.");
    let status = Command::new("npm")
        .args(&["run", "build:css"])
        .status()
        .expect("Failed to run Tailwind build.");

    assert!(status.success(), "Tailwind build failed.")
}
