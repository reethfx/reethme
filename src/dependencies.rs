use std::io;

pub fn get_dependencies() -> Vec<(String, String)> {
    let mut dependencies = Vec::new();
    println!("Do you want to include dependencies? (y/n)");
    let mut include = String::new();
    io::stdin()
        .read_line(&mut include)
        .expect("Failed to read input");
    let include = include.trim().to_lowercase();

    if include == "y" || include == "yes" {
        loop {
            println!("Enter the name of the dependency (or type 'done' to finish):");
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read input");
            let name = name.trim();

            if name.eq_ignore_ascii_case("done") {
                break;
            }

            println!("Enter the GitHub repository URL for '{}':", name);
            let mut url = String::new();
            io::stdin()
                .read_line(&mut url)
                .expect("Failed to read input");
            let url = url.trim();

            if !name.is_empty() && !url.is_empty() {
                dependencies.push((name.to_string(), url.to_string()));
            } else {
                println!("Dependency name and URL cannot be empty. Please try again.");
            }
        }
    }
    dependencies
}
