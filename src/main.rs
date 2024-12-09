mod template;
mod dependencies;
mod contributors;

use contributors::fetch_contributors;
use dependencies::get_dependencies_by_language;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn get_valid_named_input(prompt: &str, options: &[(&str, &str)]) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if let Some(&(name, _)) = options.iter().find(|&&(name, _)| name.eq_ignore_ascii_case(input)) {
            return name.to_string();
        }
        println!("Invalid input. Please enter a valid name from the options.");
    }
}

fn get_confirmation(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Invalid input. Please answer 'yes' or 'no'."),
        }
    }
}

fn select_dependencies(language: &str) -> Vec<(String, String)> {
    let mut selected_dependencies = Vec::new();
    if get_confirmation("Do you want to include dependencies for the selected language? (y/n)") {
        let available_dependencies = get_dependencies_by_language(language);

        if available_dependencies.is_empty() {
            println!("No predefined dependencies available for {}.", language);
        } else {
            println!("Available dependencies for {}:", language);
            for (index, (name, _)) in available_dependencies.iter().enumerate() {
                println!("{}. {}", index + 1, name);
            }

            loop {
                println!("Enter the number of the dependency to include or type 'done' to finish:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let input = input.trim();

                if input.eq_ignore_ascii_case("done") {
                    break;
                }

                match input.parse::<usize>() {
                    Ok(num) if num > 0 && num <= available_dependencies.len() => {
                        selected_dependencies.push(available_dependencies[num - 1].clone());
                        println!("Added: {}", available_dependencies[num - 1].0);
                    }
                    _ => println!("Invalid input. Please enter a valid number or 'done'."),
                }
            }
        }
    }
    selected_dependencies
}

fn display_items_with_icons(items: &[(&str, &str)], label: &str) {
    println!("\n{}", label);
    for (name, icon) in items {
        println!("{} {}", icon, name);
    }
    println!();
}

#[tokio::main]
async fn main() {
    let languages = vec![
        ("Rust", "\u{e7a8}"),
        ("Python", "\u{e606}"),
        ("JavaScript", "\u{e781}"),
        ("TypeScript", "\u{e628}"),
        ("Go", "\u{e724}"),
        ("C++", "\u{e61d}"),
        ("C#", "\u{f81a}"),
        ("Nixlang", "\u{f313}"),
    ];

    let frameworks_by_language = vec![
        ("Rust", vec![("Rocket", "\u{e7aa}"), ("Actix-web", "\u{e7ab}")]),
        ("Python", vec![("Django", "\u{e60d}"), ("Flask", "\u{e610}"), ("FastAPI", "\u{e70c}")]),
        ("JavaScript", vec![("React", "\u{e7ba}"), ("Vue", "\u{e7b4}"), ("Angular", "\u{e753}")]),
        ("TypeScript", vec![("ExpressJS", "\u{e70e}"), ("NestJS", "\u{e7b7}")]),
        ("C#", vec![("EntityFramework", "\u{f84d}")]),
        ("Nixlang", vec![("home-manager", "\u{f313}"), ("nixpkgs", "\u{f313}")]),
    ];

    let licenses = vec![
        ("MIT", "\u{f3e0}"),
        ("Apache-2.0", "\u{f3c9}"),
        ("GPL-3.0", "\u{f3d8}"),
        ("BSD-3-Clause", "\u{f3c7}"),
        ("LGPL-2.1", "\u{f3da}"),
        ("EPL-2.0", "\u{f3cf}"),
        ("None", "\u{f0fe}"),
    ];

    display_items_with_icons(&languages, "Available languages:");
    let selected_language = get_valid_named_input("Enter the language for your documentation:", &languages);

    let dependencies = select_dependencies(&selected_language);

    let mut selected_frameworks = Vec::new();
    if let Some((_, frameworks)) = frameworks_by_language
        .iter()
        .find(|&&(lang, _)| lang.eq_ignore_ascii_case(&selected_language))
    {
        display_items_with_icons(&frameworks, &format!("Available frameworks for {}:", selected_language));
        loop {
            println!("Enter a framework name to include or type 'done' to finish:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let input = input.trim();
            if input.eq_ignore_ascii_case("done") {
                break;
            }
            if let Some((name, _)) = frameworks.iter().find(|&&(name, _)| name.eq_ignore_ascii_case(input)) {
                selected_frameworks.push(name.to_string());
            } else {
                println!("Invalid framework name. Please try again.");
            }
        }
    } else {
        println!("No frameworks available for the selected language.");
    }

    display_items_with_icons(&licenses, "Available licenses:");
    let selected_license = get_valid_named_input("Enter the license for your documentation:", &licenses);

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

    let mut sections = Vec::new();
    loop {
        println!("Enter a section name or type 'done' to finish:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        if input.eq_ignore_ascii_case("done") {
            break;
        }
        if !input.is_empty() {
            sections.push(input.to_string());
        } else {
            println!("Section name cannot be empty.");
        }
    }

    let mut contributors_section = String::new();
    let mut include_contributors_in_toc = false;

    if get_confirmation("\nDo you want to include a 'Contributors' section? (y/n)") {
        println!("\nEnter the GitHub username:");
        let mut github_user = String::new();
        io::stdin()
            .read_line(&mut github_user)
            .expect("Failed to read input");
        let github_user = github_user.trim();

        println!("\nEnter the repository name:");
        let mut repo_name = String::new();
        io::stdin()
            .read_line(&mut repo_name)
            .expect("Failed to read input");
        let repo_name = repo_name.trim();

        match fetch_contributors(github_user, repo_name).await {
            Ok(contributors) => {
                if contributors.is_empty() {
                    contributors_section = "##Contributors\nNo contributors found.".to_string();
                } else {
                    let contributors_html: Vec<String> = contributors
                        .iter()
                        .map(|contributor| {
                            format!(
                                r#"<a href="https://github.com/{login}" target="_blank" title="{login}">
<img src="{avatar_url}" width="75" height="75" style="border-radius: 50%; margin: 5px;">
</a>"#,
                                login = contributor.login,
                                avatar_url = contributor.avatar_url
                            )
                        })
                        .collect();
                    contributors_section = format!(
                        r#"
<div style="display: flex; flex-wrap: wrap;">
{}
</div>
"#,
                        contributors_html.join("\n")
                    );
                    include_contributors_in_toc = true;
                }
            }
            Err(_) => {
                println!("Failed to fetch contributors. Skipping the 'Contributors' section.");
            }
        }
    }

    if include_contributors_in_toc {
        sections.push("Contributors".to_string());
    }

    let dir_path = Path::new("generated");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create 'generated' directory");
    }

    let file_path = dir_path.join("README.md");
    let mut file = File::create(&file_path).expect("File couldn't be created");

    let content = template::generate_readme_content(
        &title,
        &selected_language,
        &selected_license,
        &sections,
        &selected_frameworks,
        &contributors_section,
        &dependencies,
    );

    file.write_all(content.as_bytes())
        .expect("File couldn't be written");

    println!("README.md properly generated in 'generated' directory!");
}
