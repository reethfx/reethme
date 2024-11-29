mod template;

use colored::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let languages = vec![
        ("Rust", "\u{e7a8}", "bright_red"),       // nf-dev-rust
        ("Python", "\u{e606}", "bright_yellow"),  // nf-dev-python
        ("JavaScript", "\u{e781}", "yellow"),     // nf-dev-javascript
        ("TypeScript", "\u{e628}", "cyan"),      // nf-dev-typescript
        ("Go", "\u{e724}", "bright_cyan"),        // nf-dev-go
        ("C++", "\u{e61d}", "blue"),             // nf-dev-cplusplus
        ("C#", "\u{f81a}", "bright_blue"),        // nf-md-language_csharp
        ("Nixlang", "\u{f313}", "bright_white"),  // nf-dev-nix
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
    println!("\nSelect the language for your documentation (enter the number):");

    let mut lang_choice = String::new();
    io::stdin()
        .read_line(&mut lang_choice)
        .expect("Failed to read input");
    let lang_choice: usize = lang_choice
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if lang_choice == 0 || lang_choice > languages.len() {
        panic!("Invalid language selection");
    }
    let selected_language = &languages[lang_choice - 1].0;

    println!("\nAvailable licenses:");
    display_in_rows(&licenses, 5);
    println!("\nSelect a license (enter the number):");

    let mut license_choice = String::new();
    io::stdin()
        .read_line(&mut license_choice)
        .expect("Failed to read input");
    let license_choice: usize = license_choice
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if license_choice == 0 || license_choice > licenses.len() {
        panic!("Invalid license selection");
    }
    let selected_license = &licenses[license_choice - 1];

    println!("\nEnter the title for your README:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");
    let title = title.trim();

    println!("\nHow many sections do you want?");
    let mut num_sections = String::new();
    io::stdin()
        .read_line(&mut num_sections)
        .expect("Failed to read input");
    let num_sections: usize = num_sections
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut sections = Vec::new();
    for i in 1..=num_sections {
        println!("Enter the title for section {}:", i);
        let mut section_title = String::new();
        io::stdin()
            .read_line(&mut section_title)
            .expect("Failed to read input");
        sections.push(section_title.trim().to_string());
    }

    let dir_path = Path::new("generated");
    let file_path = dir_path.join("README.md");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create 'generated' directory");
    }

    let mut file = File::create(&file_path).expect("File couldn't be created");
    let content = template::generate_readme_content(&title, selected_language, selected_license, &sections);

    file.write_all(content.as_bytes())
        .expect("File couldn't be written");

    println!("README.md properly generated in 'generated' directory!");
}
