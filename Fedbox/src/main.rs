// filepath: /fedbox/fedbox/src/main.rs
use std::process::Command;

fn main() {
    println!("Welcome to the Fedbox CLI!");

    // Example of running a script
    let output = Command::new("bash")
        .arg("scripts/example_script.sh")
        .output()
        .expect("Failed to execute script");

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
}