use console::{style, Color};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", style("Welcome to Fedbox CLI!").bold().fg(Color::Green));

    loop {
        let items = vec!["Run Script", "Option 2", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option:")
            .items(&items)
            .interact()?;

        match selection {
            0 => run_script()?,
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

fn run_script() -> Result<(), Box<dyn Error>> {
    println!("Running script...");
    let output = Command::new("bash")
        .arg("scripts/example_script.sh")
        .output()?;

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}