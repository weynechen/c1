use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "c1")]
#[command(about = "A modern C project scaffolding and package management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new C project
    Init {
        /// Project name (defaults to current directory name)
        name: Option<String>,
    },
    /// Create a new module (generates .c and .h files)
    Create {
        /// Module name
        name: String,
    },
    /// Sync dependencies from project.toml
    Sync,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    project: Project,
    #[serde(default)]
    dependencies: toml::Table,
    #[serde(default)]
    build: BuildConfig,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default = "default_edition")]
    edition: String,
    #[serde(default)]
    description: String,
}

#[derive(Serialize, Deserialize, Default)]
struct BuildConfig {
    #[serde(default = "default_compiler")]
    compiler: String,
    #[serde(default)]
    flags: Vec<String>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_edition() -> String {
    "99".to_string()
}

fn default_compiler() -> String {
    "gcc".to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => cmd_init(name),
        Commands::Create { name } => cmd_create(name),
        Commands::Sync => cmd_sync(),
    }
}

/// Check if directory is empty (only hidden files allowed)
fn is_dir_empty(path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(true);
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // Allow hidden files (starting with .)
        if !name_str.starts_with('.') {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Get project name from current directory
fn get_current_dir_name() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "my_project".to_string())
}

fn cmd_init(name: Option<String>) {
    let project_name: String;
    let target_dir: std::path::PathBuf;
    
    if let Some(n) = name {
        // If project name is provided, create a directory with the same name
        project_name = n;
        target_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(&project_name);
        
        // Check if target directory already exists
        if target_dir.exists() {
            eprintln!("Error: Directory '{}' already exists.", project_name);
            std::process::exit(1);
        }
        
        // Create project directory
        fs::create_dir_all(&target_dir).expect("Failed to create project directory");
        println!("Creating project '{}' in '{}'...", project_name, project_name);
    } else {
        // If no project name provided, use current directory name and generate in current directory
        project_name = get_current_dir_name();
        target_dir = std::env::current_dir().expect("Failed to get current directory");
        
        // Safety check: directory must be empty (except hidden files)
        if !is_dir_empty(&target_dir).expect("Failed to check directory") {
            eprintln!("Error: Directory is not empty. c1 init must be run in an empty directory.");
            std::process::exit(1);
        }
        
        println!("Initializing project '{}'...", project_name);
    }
    
    // Switch to target directory for subsequent operations
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    if target_dir != original_dir {
        std::env::set_current_dir(&target_dir).expect("Failed to change to project directory");
    }

    // Create directory structure
    fs::create_dir_all("src").expect("Failed to create src directory");
    fs::create_dir_all("include").expect("Failed to create include directory");
    fs::create_dir_all("external").expect("Failed to create external directory");
    fs::create_dir_all("build").expect("Failed to create build directory");

    // Create main.c (placed in project root directory)
    let main_c = format!(
        r#"#include <stdio.h>

int main(void) {{
    printf("Hello {}\n");
    return 0;
}}
"#,
        project_name
    );
    fs::write("main.c", main_c).expect("Failed to create main.c");

    // Create CMakeLists.txt
    let cmake_content = format!(
        r#"cmake_minimum_required(VERSION 3.16)
project({} C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Source files list
set(SOURCES
    main.c
    # @c1_sources
)

# Header files list
set(HEADERS
    # @c1_headers
)

add_executable(${{PROJECT_NAME}} ${{SOURCES}} ${{HEADERS}})

# Include directories: following Pitchfork convention, headers in include/
target_include_directories(${{PROJECT_NAME}} PRIVATE 
    ${{CMAKE_CURRENT_SOURCE_DIR}}/include
)

# Default linked libraries (reserved example)
# target_link_libraries(${{PROJECT_NAME}} PRIVATE m)
"#,
        project_name
    );
    fs::write("CMakeLists.txt", cmake_content).expect("Failed to create CMakeLists.txt");

    // Create project.toml
    let project_toml = format!(
        r#"[project]
name = "{}"
version = "0.1.0"
edition = "c99"
description = "A C project created with c1"

[dependencies]
# Add your dependencies here
# Example:
# arc-c = {{ git = "https://github.com/weynechen/arc-c.git", tag = "v0.5.0" }}

[build]
compiler = "gcc"
flags = ["-O3", "-Wall"]
"#,
        project_name
    );
    fs::write("project.toml", project_toml).expect("Failed to create project.toml");

    // Create README.md
    let readme = format!(
        r#"# {}

A C project created with c1.

## Building

```bash
cmake -B build
cmake --build build
```

## Project Structure

```txt
.
├── main.c         # Main entry point
├── CMakeLists.txt
├── include/       # Header files
├── src/           # Source files
├── external/      # External dependencies
├── build/         # Build output directory
└── project.toml   # Project configuration
```
"#,
        project_name
    );
    fs::write("README.md", readme).expect("Failed to create README.md");

    // Create .gitignore
    let gitignore = r#"# Build directory 
/build

# IDE
/.idea
/.vscode

# Compiled files
*.o
*.a
*.so
*.exe
/cmake-build-*
"#;
    fs::write(".gitignore", gitignore).expect("Failed to create .gitignore");

    // Run git init
    match Command::new("git").args(["init"]).output() {
        Ok(output) => {
            if output.status.success() {
                println!("Initialized git repository");
            } else {
                eprintln!("Warning: git init failed");
            }
        }
        Err(_) => {
            eprintln!("Warning: git not found, skipping git init");
        }
    }

    println!("✓ Project '{}' initialized successfully!", project_name);
    
    // Restore original working directory
    if target_dir != original_dir {
        std::env::set_current_dir(&original_dir).expect("Failed to restore original directory");
    }
}

fn cmd_create(name: String) {
    // Validate module name (only alphanumeric and underscore)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        eprintln!("Error: Module name must contain only letters, numbers, and underscores");
        std::process::exit(1);
    }

    let src_file = format!("src/{}.c", name);
    let header_file = format!("include/{}.h", name);

    // Check if files already exist
    if Path::new(&src_file).exists() {
        eprintln!("Error: {} already exists", src_file);
        std::process::exit(1);
    }
    if Path::new(&header_file).exists() {
        eprintln!("Error: {} already exists", header_file);
        std::process::exit(1);
    }

    // Create header guard macro name
    let guard_name = format!("_{}_H", name.to_uppercase());

    // Generate header file content
    let header_content = format!(
        r#"#ifndef {}
#define {}

// TODO: Add your declarations here

#endif // {}
"#,
        guard_name, guard_name, guard_name
    );

    // Generate source file content
    let src_content = format!(
        r#"#include "{}.h"

// TODO: Add your implementation here
"#,
        name
    );

    // Write files
    fs::write(&src_file, src_content).expect("Failed to create source file");
    fs::write(&header_file, header_content).expect("Failed to create header file");

    println!("✓ Created {} and {}", src_file, header_file);

    // Update CMakeLists.txt
    update_cmake_lists(&name);
}

fn update_cmake_lists(module_name: &str) {
    let cmake_path = "CMakeLists.txt";
    
    if !Path::new(cmake_path).exists() {
        eprintln!("Warning: CMakeLists.txt not found, skipping automatic registration");
        return;
    }

    let content = fs::read_to_string(cmake_path).expect("Failed to read CMakeLists.txt");

    // Update SOURCES section
    let src_placeholder = "# @c1_sources";
    let src_entry = format!("    src/{}.c\n    {}", module_name, src_placeholder);
    let new_content = content.replace(src_placeholder, &src_entry);

    // Update HEADERS section
    let header_placeholder = "# @c1_headers";
    let header_entry = format!("    include/{}.h\n    {}", module_name, header_placeholder);
    let new_content = new_content.replace(header_placeholder, &header_entry);

    fs::write(cmake_path, new_content).expect("Failed to update CMakeLists.txt");

    println!("✓ Updated CMakeLists.txt");
}

fn cmd_sync() {
    let config_path = "project.toml";
    
    if !Path::new(config_path).exists() {
        eprintln!("Error: project.toml not found");
        std::process::exit(1);
    }

    let content = fs::read_to_string(config_path).expect("Failed to read project.toml");
    let config: ProjectConfig = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing project.toml: {}", e);
            std::process::exit(1);
        }
    };

    if config.dependencies.is_empty() {
        println!("No dependencies to sync");
        return;
    }

    // Ensure external directory exists
    fs::create_dir_all("external").expect("Failed to create external directory");

    for (name, value) in config.dependencies {
        println!("Syncing dependency: {}...", name);
        
        if let Some(table) = value.as_table() {
            if let Some(git_url) = table.get("git").and_then(|v| v.as_str()) {
                let target_dir = format!("external/{}", name);
                
                // Remove existing directory if it exists
                if Path::new(&target_dir).exists() {
                    println!("  Removing existing {}...", target_dir);
                    fs::remove_dir_all(&target_dir).expect("Failed to remove existing directory");
                }

                // Build git clone command
                let mut cmd = Command::new("git");
                cmd.args(["clone", git_url, &target_dir]);

                // Add branch or tag if specified
                if let Some(branch) = table.get("branch").and_then(|v| v.as_str()) {
                    cmd.args(["--branch", branch, "--single-branch"]);
                } else if let Some(tag) = table.get("tag").and_then(|v| v.as_str()) {
                    cmd.args(["--branch", tag, "--single-branch"]);
                }

                // Execute git clone
                match cmd.output() {
                    Ok(output) => {
                        if output.status.success() {
                            println!("  ✓ Cloned {} to {}", name, target_dir);
                        } else {
                            eprintln!("  ✗ Failed to clone {}", name);
                            eprintln!("    {}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        eprintln!("  ✗ Error cloning {}: {}", name, e);
                    }
                }
            } else {
                eprintln!("  ✗ No 'git' URL specified for {}", name);
            }
        } else {
            eprintln!("  ✗ Invalid dependency format for {}", name);
        }
    }

    println!("\n✓ Dependency sync complete");
}
