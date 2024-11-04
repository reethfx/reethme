mod template;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let path = Path::new("README.md");
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("File couldn't be created: {}", e),
    };
    let content = template::generate_readme_content();


    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("README.md properly generated!"),
        Err(e) => panic!("File coulnd't be written: {}", e),
    };
}
