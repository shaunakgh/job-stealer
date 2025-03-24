use std::process::Command;
use std::error::Error;
use colored::*;

fn main() -> Result<(), Box<dyn Error>> {
    let model = loop {
        let mut line = String::new();
        println!("{} Enter the model to be used: ", "?".blue());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(model) => {
                let status = Command::new("ollama")
                    .args(&["run", &model])
                    .output()?;

                if status.status.success() {
                    println!("{{ Model: {} }}", model.to_string().blue().bold());
                    break model;
                } else {
                    eprintln!("{}", "Invalid model — please try again.".red());
                }
            }
            Err(_) => { eprintln!("{}", "Invalid model — please try again.".red()); }
        }
    };
    let output = Command::new("ollama")
        .args(&["run", &model, "tell me a new joke"])
        .output()?;

    if output.status.success() {
        println!("{} {}\n {}", "+".green(), "Output: ".bright_yellow().bold(), String::from_utf8_lossy(&output.stdout).blue().italic());
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr).red().bold());
    }

    Ok(())
}