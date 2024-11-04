pub fn generate_readme_content() -> String {
	let content = r#"
<h1 align="center">ReethMe - ドキュメント</h1>


<!-- Badges section -->
<div align="center">

![https://www.rust-lang.org/es](https://img.shields.io/badge/Rust-24273A.svg?style=flat&logo=rust&logoColor=fc9d03) 
[![license](https://img.shields.io/pypi/l/supervision)](https://github.com/roboflow/supervision/blob/main/LICENSE.md)

</div>
<!-- Badges section end -->


## Table of Contents
- [About](#-about)
- [Installation](#-installation)


📖 About
> [!NOTE]  
>  While the templates include a default setup for general projects, they can be customized to fit different visual styles and layouts based on your preferences.

This program uses Rust’s efficient concurrency model and powerful text templating capabilities to auto-generate personalized README files with rich formatting. It includes:

- **serde**: for serializing and deserializing JSON configurations for customizing README content.
- **tera**: a flexible templating engine for defining the README structure with user-specific variables.
- **tokio**: an asynchronous runtime to efficiently handle multiple I/O tasks, enabling quick processing for larger projects with multiple README files.

With this setup, the program allows you to produce highly customized and visually engaging README files that can incorporate the latest updates and templates effortlessly. 🚨 However, be aware that this advanced templating setup may require careful handling of configuration files to avoid potential mismatches.

## 🛠️ Installation

This repository depends purely on Rust, so you must have the [Rust](https://www.rust-lang.org/tools/install) enviroment installed.

### Windows Installation
If you use Windows as your OS, simply access the link of the oficial page and choose your installation methods.

### Linux Installation
First of all, you will need a C compiler for executing Rust programs. 
vLinux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the `build-essential` package by simply running.
```bash
sudo apt update
sudo apt install build-essential
```

Then you can execute the follwing command:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```bash
Rust is installed now. Great!
```

### MacOS installation

For Mac users, it's pretty similar to Linux, just run the script installation:
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then install the compiler with the following command:

```bash
xcode-select --install
```
"#;
	content.to_string()
}