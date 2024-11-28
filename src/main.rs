mod template;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("Enter the title for your README:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");
    let title = title.trim();

    println!("How many sections do you want?");
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
    let content = template::generate_readme_content(&title, &sections);

    file.write_all(content.as_bytes())
        .expect("File couldn't be written");

    println!("README.md properly generated in 'generated' directory!");
}
