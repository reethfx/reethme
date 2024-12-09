pub fn get_dependencies_by_language(language: &str) -> Vec<(String, String)> {
    match language {
        "Rust" => vec![
            ("serde".to_string(), "https://github.com/serde-rs/serde".to_string()),
            ("tokio".to_string(), "https://github.com/tokio-rs/tokio".to_string()),
        ],
        "Python" => vec![
            ("requests".to_string(), "https://github.com/psf/requests".to_string()),
            ("numpy".to_string(), "https://github.com/numpy/numpy".to_string()),
        ],
        "JavaScript" => vec![
            ("react".to_string(), "https://github.com/facebook/react".to_string()),
            ("axios".to_string(), "https://github.com/axios/axios".to_string()),
        ],
        "TypeScript" => vec![
            ("typescript".to_string(), "https://github.com/microsoft/TypeScript".to_string()),
            ("nestjs".to_string(), "https://github.com/nestjs/nest".to_string()),
        ],
        "Go" => vec![
            ("gin".to_string(), "https://github.com/gin-gonic/gin".to_string()),
            ("gorm".to_string(), "https://github.com/go-gorm/gorm".to_string()),
        ],
        "C++" => vec![
            ("boost".to_string(), "https://github.com/boostorg/boost".to_string()),
        ],
        "C#" => vec![
            ("Newtonsoft.Json".to_string(), "https://github.com/JamesNK/Newtonsoft.Json".to_string()),
        ],
        "Nixlang" => vec![
            ("nixpkgs".to_string(), "https://github.com/NixOS/nixpkgs".to_string()),
        ],
        _ => vec![],
    }
}
