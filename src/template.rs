pub fn generate_readme_content(
    title: &str,
    language: &str,
    license: &str,
    sections: &[String],
) -> String {
    let badges = match language {
        "Rust" => r#"![Rust](https://img.shields.io/badge/Rust-24273A.svg?style=flat&logo=rust&logoColor=fc9d03)"#,
        "Python" => r#"![Python](https://img.shields.io/badge/Python-3776AB.svg?style=flat&logo=python&logoColor=white)"#,
        "JavaScript" => r#"![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E.svg?style=flat&logo=javascript&logoColor=black)"#,
        "TypeScript" => r#"![TypeScript](https://img.shields.io/badge/TypeScript-3178C6.svg?style=flat&logo=typescript&logoColor=white)"#,
        "Go" => r#"![Go](https://img.shields.io/badge/Go-00ADD8.svg?style=flat&logo=go&logoColor=white)"#,
        "C++" => r#"![C++](https://img.shields.io/badge/C++-00599C.svg?style=flat&logo=c%2B%2B&logoColor=white)"#,
        "C#" => r#"![C#](https://img.shields.io/badge/C%23-68217A.svg?style=flat&logo=c-sharp&logoColor=white)"#,
        "Nixlang" => r#"![Nix](https://img.shields.io/badge/Nix-5277C3.svg?style=flat&logo=nixos&logoColor=white)"#,
        _ => "",
    };

    let license_badge = match license {
        "MIT" => r#"[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)"#,
        "Apache-2.0" => r#"[![Apache 2.0 License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)"#,
        "GPL-3.0" => r#"[![GPLv3 License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)"#,
        "BSD-3-Clause" => r#"[![BSD 3-Clause License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)"#,
        "LGPL-2.1" => r#"[![LGPL 2.1 License](https://img.shields.io/badge/License-LGPL%202.1-blue.svg)](https://opensource.org/licenses/LGPL-2.1)"#,
        "EPL-2.0" => r#"[![EPL 2.0 License](https://img.shields.io/badge/License-EPL%202.0-blue.svg)](https://opensource.org/licenses/EPL-2.0)"#,
        "None" => "",
        _ => "",
    };

    let mut table_of_contents = String::new();
    for section in sections {
        table_of_contents.push_str(&format!("- [{}](#-{})\n", section, section.to_lowercase().replace(" ", "-")));
    }

    let mut sections_content = String::new();
    for section in sections {
        sections_content.push_str(&format!(
            r#"
## {}
Content for the {} section goes here.

"#,
            section, section
        ));
    }

    format!(
        r#"
<h1 align="center">{}</h1>

<!-- Badges section -->
<div align="center">

{}
{}

</div>
<!-- Badges section end -->

## Table of Contents
{}

{}

"#,
        title, badges, license_badge, table_of_contents, sections_content
    )
}
