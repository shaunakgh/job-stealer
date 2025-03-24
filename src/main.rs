use colored::*;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let model = loop {
        let mut line = String::new();
        println!("{} Enter the model to be used: ", "?".blue());
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(model) => {
                let status = Command::new("ollama").args(&["run", &model]).output()?;

                if status.status.success() {
                    println!("{{ Model: {} }}", model.to_string().blue().bold());
                    break model;
                } else {
                    eprintln!("{}", "Invalid model — please try again.".red());
                }
            }
            Err(_) => {
                eprintln!("{}", "Invalid model — please try again.".red());
            }
        }
    };
    let prompt = loop {
        let mut line = String::new();
        println!("{} Enter the prompt: ", "?".blue());
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(prompt) => {
                println!("{{ Prompt: {} }}", prompt.to_string().blue().bold());
                break prompt;
            }
            Err(_) => {
                eprintln!("{}", "Invalid prompt — please try again.".red());
            }
        }
    };
    let language = loop {
        let mut line = String::new();
        println!(
            "{} Python\n{} Ruby\n{} Rust\n{} Webapp with Svelte\n{} Enter the language/option: ",
            "[1]".bold(),
            "[2]".bold(),
            "[3]".bold(),
            "[4]".bold(),
            "?".blue()
        );
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match line.trim().parse::<i32>() {
            Ok(lang) => {
                match lang {
                    1 => { println!("{{ Language: {} }}", "Python".blue().bold()); break "python"; }
                    2 => { println!("{{ Language: {} }}", "Ruby".blue().bold()); break "ruby"; }
                    3 => { println!("{{ Language: {} }}", "Rust".blue().bold()); break "rust"; }
                    4 => { println!("{{ Language: {} }}", "Webapp with Svelte".blue().bold()); break "webapp"; }
                    _ => { eprintln!("{}", "Invalid language — please try again.".red()); }
                }
            }
            Err(_) => {
                eprintln!("{}", "Invalid language — please try again.".red());
            }
        }
    };
    let output = Command::new("ollama")
        .args(&["run", &model, &prompt])
        .output()?;

    if output.status.success() {
        println!(
            "{} {}\n{}",
            "+".green(),
            "Output: ".bright_yellow().bold(),
            String::from_utf8_lossy(&output.stdout).blue().italic()
        );
    } else {
        eprintln!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr).red().bold()
        );
    }

    Ok(())
}
