use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "c1")]
#[command(about = "A modern C project scaffolding and package management tool, like cargo for C")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new C project in a new directory
    New {
        /// Project name (will create a directory with this name)
        name: String,
    },
    /// Initialize a new C project in the current directory
    Init,
    /// Create a new module (generates .c and .h files)
    Create {
        /// Module name
        name: String,
    },
    /// Build and run the project
    Run,
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Add a git dependency to the project
    Add {
        /// Git repository URL
        url: String,
        /// Optional: specify a tag
        #[arg(long)]
        tag: Option<String>,
        /// Optional: specify a branch
        #[arg(long)]
        branch: Option<String>,
    },
    /// Clean the build directory
    Clean,
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
        Commands::New { name } => cmd_new(name),
        Commands::Init => cmd_init(),
        Commands::Create { name } => cmd_create(name),
        Commands::Run => cmd_run(),
        Commands::Build { release } => cmd_build(release),
        Commands::Add { url, tag, branch } => cmd_add(url, tag, branch),
        Commands::Clean => cmd_clean(),
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

/// Create a new project in a new directory
fn cmd_new(name: String) {
    let target_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(&name);

    // Check if target directory already exists
    if target_dir.exists() {
        eprintln!("Error: Directory '{}' already exists.", name);
        std::process::exit(1);
    }

    // Create project directory
    fs::create_dir_all(&target_dir).expect("Failed to create project directory");
    println!("Creating project '{}' in '{}'...", name, name);

    // Initialize project in the new directory
    init_project_in_dir(&target_dir, &name);

    println!("✓ Project '{}' created successfully!", name);
}

/// Initialize a project in the current directory
fn cmd_init() {
    let target_dir = std::env::current_dir().expect("Failed to get current directory");
    let project_name = get_current_dir_name();

    // Safety check: directory must be empty (except hidden files)
    if !is_dir_empty(&target_dir).expect("Failed to check directory") {
        eprintln!("Error: Directory is not empty. c1 init must be run in an empty directory.");
        std::process::exit(1);
    }

    println!("Initializing project '{}'...", project_name);

    // Initialize project in current directory
    init_project_in_dir(&target_dir, &project_name);

    println!("✓ Project '{}' initialized successfully!", project_name);
}

/// Common function to initialize project files in a directory
fn init_project_in_dir(target_dir: &Path, project_name: &str) {
    // Switch to target directory for subsequent operations
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    if target_dir != original_dir {
        std::env::set_current_dir(target_dir).expect("Failed to change to project directory");
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
c1 build
c1 build --release
```

## Running

```bash
c1 run
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

/// Get project name from project.toml
fn get_project_name_from_config() -> Option<String> {
    let config_path = "project.toml";
    if !Path::new(config_path).exists() {
        return None;
    }
    let content = fs::read_to_string(config_path).ok()?;
    let config: ProjectConfig = toml::from_str(&content).ok()?;
    Some(config.project.name)
}

/// Build the project with cmake
fn cmd_build(release: bool) {
    let build_type = if release { "Release" } else { "Debug" };
    let build_dir = "build";

    println!("Building project ({} mode)...", build_type);

    // Step 1: cmake -B build -DCMAKE_BUILD_TYPE=...
    let cmake_config = Command::new("cmake")
        .args(["-B", build_dir, &format!("-DCMAKE_BUILD_TYPE={}", build_type)])
        .output();

    match cmake_config {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("CMake configuration failed:");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to run cmake: {}", e);
            eprintln!("Make sure cmake is installed.");
            std::process::exit(1);
        }
    }

    // Step 2: cmake --build build
    let cmake_build = Command::new("cmake")
        .args(["--build", build_dir])
        .output();

    match cmake_build {
        Ok(output) => {
            if output.status.success() {
                println!("✓ Build completed successfully!");
            } else {
                eprintln!("Build failed:");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to run cmake --build: {}", e);
            std::process::exit(1);
        }
    }
}

/// Build and run the project
fn cmd_run() {
    // First build the project (debug mode)
    cmd_build(false);

    // Get project name for executable
    let project_name = get_project_name_from_config()
        .unwrap_or_else(|| get_current_dir_name());

    let executable = format!("build/{}", project_name);

    if !Path::new(&executable).exists() {
        eprintln!("Error: Executable '{}' not found.", executable);
        std::process::exit(1);
    }

    println!("\nRunning {}...\n", executable);

    // Run the executable
    let status = Command::new(&executable)
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                if let Some(code) = s.code() {
                    std::process::exit(code);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to run executable: {}", e);
            std::process::exit(1);
        }
    }
}

/// Add a git dependency to the project
fn cmd_add(url: String, tag: Option<String>, branch: Option<String>) {
    let config_path = "project.toml";

    if !Path::new(config_path).exists() {
        eprintln!("Error: project.toml not found. Are you in a c1 project?");
        std::process::exit(1);
    }

    // Extract package name from git URL
    let pkg_name = extract_package_name(&url);

    println!("Adding dependency: {}...", pkg_name);

    // Ensure external directory exists
    fs::create_dir_all("external").expect("Failed to create external directory");

    let target_dir = format!("external/{}", pkg_name);

    // Remove existing directory if it exists
    if Path::new(&target_dir).exists() {
        println!("  Removing existing {}...", target_dir);
        fs::remove_dir_all(&target_dir).expect("Failed to remove existing directory");
    }

    // Build git clone command
    let mut cmd = Command::new("git");
    cmd.args(["clone", &url, &target_dir]);

    // Add branch or tag if specified
    if let Some(ref b) = branch {
        cmd.args(["--branch", b, "--single-branch"]);
    } else if let Some(ref t) = tag {
        cmd.args(["--branch", t, "--single-branch"]);
    }

    // Execute git clone
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("  ✓ Cloned {} to {}", pkg_name, target_dir);
            } else {
                eprintln!("  ✗ Failed to clone {}", pkg_name);
                eprintln!("    {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("  ✗ Error cloning {}: {}", pkg_name, e);
            std::process::exit(1);
        }
    }

    // Update project.toml
    update_project_toml(&pkg_name, &url, tag, branch);

    println!("✓ Added {} to project.toml", pkg_name);
}

/// Extract package name from git URL
fn extract_package_name(url: &str) -> String {
    // Handle URLs like:
    // - https://github.com/user/repo.git
    // - git@github.com:user/repo.git
    // - https://github.com/user/repo
    let url = url.trim_end_matches(".git");
    let name = url
        .rsplit('/')
        .next()
        .or_else(|| url.rsplit(':').next())
        .unwrap_or("unknown");
    name.to_string()
}

/// Update project.toml with new dependency
fn update_project_toml(name: &str, url: &str, tag: Option<String>, branch: Option<String>) {
    let config_path = "project.toml";
    let content = fs::read_to_string(config_path).expect("Failed to read project.toml");

    // Build dependency entry
    let dep_entry = if let Some(t) = tag {
        format!("{} = {{ git = \"{}\", tag = \"{}\" }}", name, url, t)
    } else if let Some(b) = branch {
        format!("{} = {{ git = \"{}\", branch = \"{}\" }}", name, url, b)
    } else {
        format!("{} = {{ git = \"{}\" }}", name, url)
    };

    // Find [dependencies] section and add entry
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut dep_section_idx = None;
    let mut insert_idx = None;

    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "[dependencies]" {
            dep_section_idx = Some(i);
        } else if dep_section_idx.is_some() {
            // Check if we hit another section
            if line.trim().starts_with('[') && !line.trim().starts_with("[dependencies") {
                insert_idx = Some(i);
                break;
            }
            // Check if this dependency already exists
            if line.trim().starts_with(&format!("{} ", name)) || line.trim().starts_with(&format!("{}=", name)) {
                // Replace existing entry
                lines[i] = dep_entry.clone();
                fs::write(config_path, lines.join("\n")).expect("Failed to write project.toml");
                return;
            }
        }
    }

    // Insert after [dependencies] section
    if let Some(idx) = dep_section_idx {
        let insert_pos = insert_idx.unwrap_or(lines.len());
        // Find the first non-comment, non-empty line after [dependencies]
        let mut pos = idx + 1;
        while pos < insert_pos {
            let line = lines[pos].trim();
            if !line.is_empty() && !line.starts_with('#') {
                break;
            }
            pos += 1;
        }
        lines.insert(pos, dep_entry);
    } else {
        // No [dependencies] section, add it
        lines.push(String::new());
        lines.push("[dependencies]".to_string());
        lines.push(dep_entry);
    }

    fs::write(config_path, lines.join("\n")).expect("Failed to write project.toml");
}

/// Clean the build directory
fn cmd_clean() {
    let build_dir = "build";

    if !Path::new(build_dir).exists() {
        println!("Build directory does not exist, nothing to clean.");
        return;
    }

    println!("Cleaning build directory...");

    // Remove all contents in build directory
    match fs::remove_dir_all(build_dir) {
        Ok(_) => {
            // Recreate empty build directory
            fs::create_dir_all(build_dir).expect("Failed to recreate build directory");
            println!("✓ Build directory cleaned successfully!");
        }
        Err(e) => {
            eprintln!("Failed to clean build directory: {}", e);
            std::process::exit(1);
        }
    }
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
