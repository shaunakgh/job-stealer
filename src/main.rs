use colored::*;
use regex::Regex;
use std::error::Error;
use std::process::Command;

type ModelPromptLang = (String, String, String);

fn get_args() -> Result<ModelPromptLang, Box<dyn Error>> {
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
                } else { eprintln!("{}", "Invalid model — please try again.".red()); }
            }
            Err(_) => { eprintln!("{}", "Invalid model — please try again.".red()); }
        }
    };
    let language = loop {
        let mut line = String::new();
        println!(
            "{} Python\n{} Ruby\n{} Rust\n{} Enter the language/option: ",
            "[1]".bold(),
            "[2]".bold(),
            "[3]".bold(),
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
                    _ => { eprintln!("{}", "Invalid language — please try again.".red()); }
                }
            }
            Err(_) => {
                eprintln!("{}", "Invalid language — please try again.".red());
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
            Ok(prompt) => { break prompt; }
            Err(_) => {
                eprintln!("{}", "Invalid prompt — please try again.".red());
            }
        }
    };
    Ok((model, prompt, language.to_string()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (model, prompt, language) = get_args()?;
    let output = Command::new("ollama")
        .args(&["run", &model, &format!("Code this with a concise and multidisciplinary intent and form without adding anything extra - just the code: {} in {}", prompt, language)])
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
