mod template;

use colored::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Función para validar la entrada del usuario
fn get_valid_input(prompt: &str, options: &[&str]) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= options.len() {
                return options[index - 1].to_string();
            }
        }
        println!("Invalid input. Please enter a valid number between 1 and {}.", options.len());
    }
}

fn main() {
    let languages = vec![
        ("Rust", "\u{e7a8}", "bright_red"),
        ("Python", "\u{e606}", "bright_yellow"),
        ("JavaScript", "\u{e781}", "yellow"),
        ("TypeScript", "\u{e628}", "cyan"),
        ("Go", "\u{e724}", "bright_cyan"),
        ("C++", "\u{e61d}", "blue"),
        ("C#", "\u{f81a}", "bright_blue"),
        ("Nixlang", "\u{f313}", "bright_white"),
    ];

    let licenses = vec![
        "MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "LGPL-2.1", "EPL-2.0", "None",
    ];

    fn display_in_rows_with_nerdfonts(items: &[(&str, &str, &str)], max_rows: usize) {
        let num_columns = (items.len() + max_rows - 1) / max_rows;
        for row in 0..max_rows {
            for col in 0..num_columns {
                if let Some((name, icon, color)) = items.get(col * max_rows + row) {
                    let formatted = format!("{} {}", icon, name).color(Color::from(*color));
                    print!("{:<25}", formatted);
                }
            }
            println!();
        }
    }

    fn display_in_rows(items: &[&str], max_rows: usize) {
        let num_columns = (items.len() + max_rows - 1) / max_rows;
        for row in 0..max_rows {
            for col in 0..num_columns {
                if let Some(item) = items.get(col * max_rows + row) {
                    print!("{:<20}", item);
                }
            }
            println!();
        }
    }

    println!("Available languages:");
    display_in_rows_with_nerdfonts(&languages, 5);

    let language_options: Vec<&str> = languages.iter().map(|(name, _, _)| *name).collect();
    let selected_language = get_valid_input(
        "\nSelect the language for your documentation (enter the number):",
        &language_options,
    );

    println!("\nAvailable licenses:");
    display_in_rows(&licenses, 5);

    let selected_license = get_valid_input(
        "\nSelect a license (enter the number):",
        &licenses,
    );

    let title = loop {
        println!("\nEnter the title for your README:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let title = input.trim();
        if !title.is_empty() {
            break title.to_string();
        }
        println!("Title cannot be empty. Please enter a valid title.");
    };

    let num_sections = loop {
        println!("\nHow many sections do you want?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 {
                break num;
            }
        }
        println!("Please enter a valid number greater than 0.");
    };

    let mut sections = Vec::new();
    for i in 1..=num_sections {
        loop {
            println!("Enter the title for section {}:", i);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let section_title = input.trim();
            if !section_title.is_empty() {
                sections.push(section_title.to_string());
                break;
            }
            println!("Section title cannot be empty. Please enter a valid title.");
        }
    }

    let dir_path = Path::new("generated");
    let file_path = dir_path.join("README.md");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create 'generated' directory");
    }

    let mut file = File::create(&file_path).expect("File couldn't be created");
    let content = template::generate_readme_content(&title, &selected_language, &selected_license, &sections);

    file.write_all(content.as_bytes())
        .expect("File couldn't be written");

    println!("README.md properly generated in 'generated' directory!");
}
