use std::process::Command;

fn main() {
    println!("Building tailwind stylesheet.");
    let status = Command::new("npm")
        .args(&["run", "build:css"])
        .status()
        .expect("Failed to run Tailwind build.");

    assert!(status.success(), "Tailwind build failed.")
}
