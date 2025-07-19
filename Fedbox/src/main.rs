use console::{style, Color};
use dialoguer::{Select, theme::ColorfulTheme};
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    println!("{}", style("Welcome to Fedbox CLI!").bold().fg(Color::Green));

    loop {
        let items = vec!["Run Script", "Option 2", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option:")
            .items(&items)
            .interact()?;

        match selection {
            0 => run_script(),
            1 => println!("Option 2 selected"), // Replace with actual functionality
            2 => {
                println!("Exiting Fedbox CLI.");
                break;
            }
            _ => println!("Invalid selection"),
        }
    }

    Ok(())
}

fn run_script() {
    println!("Running script...");
    let output = Command::new("bash")
        .arg("scripts/example_script.sh")
        .output()
        .expect("Failed to execute script");

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
}