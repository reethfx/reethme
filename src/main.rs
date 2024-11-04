use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let path = Path::new("README.md");
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("File couln't be generated: {}", e),
    };

    let content = "# Hello wolrd!\n\nThis is a test.";
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("README.md successfully generated!"),
        Err(e) => panic!("File couldn't be written: {}", e),
    };
}
