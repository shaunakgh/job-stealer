mod env;

use colored::*;
use regex::Regex;
use std::error::Error;
use std::process::Command;

type ModelPrompt = (String, String, String, String);

// Get arguments for ai prompt
fn get_args() -> Result<ModelPrompt, Box<dyn Error>> {
    // model name (e.g deepseek-r1)
    let model = loop {
        let mut line = String::new();
        println!("{} Enter the model to be used. Click enter for default (mistral): ", "?".blue());
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(model) => {
                if model == "" { println!("{}", "Defaulting to Mistral!".blue().italic()); break "mistral".to_string(); }

                let status = Command::new("ollama").args(&["run", &model]).output()?;

                if status.status.success() {
                    println!("{{ Model: {} }}", model.to_string().blue().bold());
                    break model;
                } else { eprintln!("{}", "Invalid model — please try again.".red()); }
            }
            Err(_) => { eprintln!("{}", "Invalid model — please try again.".red()); }
        }
    };
    // Language of choice to be coded in
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
    // Project prompt (e.g Create a django webapp)
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
    // Path to project
    let path = loop {
        let mut line = String::new();
        println!("{} Enter the path to the project. Click enter for default (current working directory): ", "?".blue());
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(path) => {
                if path == "" { println!("{}", "Defaulting to current working directory!".blue().italic()); break ".".to_string(); }

                let status = Command::new(&format!("cd {}", path)).output()?;
                if status.status.success() {
                    println!("{{ Path: {} }}", path.to_string().blue().bold());
                    break path;
                } else { eprintln!("{}", "Invalid model — please try again.".red()); }

                break path;
            }
            Err(_) => {
                eprintln!("{}", "Invalid path — please try again.".red());
            }
        }
    };
    Ok((model, prompt, language.to_string(), path.to_string()))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Init AI
    let (model, prompt, language, path) = get_args()?;
    let output = Command::new("ollama")
        .args(&["run", &model, &format!("Code this with a concise and multidisciplinary intent and form without adding anything extra - just the code: {} in {}", prompt, language)])
        .output()?;

    // Parse to get code
    let re = Regex::new(&format!(r"(?s)```{}\s+(.*?)\s+```", language)).unwrap();
    #[allow(unused_assignments)]
    let mut code = "";

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);

        if let Some(captures) = re.captures(&text) {
            code = captures.get(1).unwrap().as_str();
            println!(
                "{} {}\n{}",
                "+".green(),
                "Output: ".bright_yellow().bold(),
                code
            );
        } else {
            println!("{}", "No code block found. Maybe retry with diff model".red().italic());
            std::process::exit(0)
        }
    } else {
        eprintln!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr).red().bold()
        );
    }

    Ok(())
}
