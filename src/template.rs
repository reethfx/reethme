pub fn generate_readme_content(title: &str, sections: &[String]) -> String {
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

![https://www.rust-lang.org/es](https://img.shields.io/badge/Rust-24273A.svg?style=flat&logo=rust&logoColor=fc9d03) 
[![license](https://img.shields.io/pypi/l/supervision)](https://github.com/roboflow/supervision/blob/main/LICENSE.md)

</div>
<!-- Badges section end -->

## Table of Contents
{}

{}

"#,
        title, table_of_contents, sections_content
    )
}
