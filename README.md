# c1

A modern C project scaffolding and package management tool, like cargo for C.

## Features

- **Project Scaffolding**: `c1 new` / `c1 init` - Create C projects with standard directory structure
- **Build System**: `c1 build` / `c1 run` - Build and run projects with a single command
- **Module Management**: `c1 create` - Atomic creation of source/header file pairs
- **Dependency Management**: `c1 add` / `c1 sync` - Git-based dependency management
- **Pitchfork Layout**: Enforces standard C project directory structure
- **Modern CMake**: Automated build configuration management

## Installation

### From Source

```bash
git clone https://github.com/yourusername/c1.git
cd c1
cargo build --release
```

The binary will be available at `target/release/c1`.

## Quick Start

### Create a New Project

```bash
# Create a new project with its own directory
c1 new my_project
cd my_project

# Or initialize in the current directory (must be empty)
mkdir my_project && cd my_project
c1 init
```

This creates the following structure:

```
my_project/
├── main.c              # Entry point with "Hello my_project"
├── CMakeLists.txt      # Pre-configured with source/header anchors
├── project.toml        # Project configuration
├── README.md           # Project documentation
├── .gitignore          # Git ignore rules
├── build/              # Build output directory
├── include/            # Header files
├── src/                # Additional source files
└── external/           # External dependencies
```

### Build and Run

```bash
# Build and run (debug mode)
c1 run

# Build only
c1 build

# Build in release mode
c1 build --release
```

### Create a New Module

```bash
c1 create utils
```

This generates:
- `src/utils.c` - Source file with `#include "utils.h"`
- `include/utils.h` - Header file with include guard

And automatically updates `CMakeLists.txt` with the new files.

### Add Dependencies

```bash
# Add a git dependency
c1 add https://github.com/DaveGamble/cJSON.git

# Add with specific tag
c1 add https://github.com/weynechen/arc-c.git --tag v0.5.0

# Add with specific branch
c1 add https://github.com/example/lib.git --branch develop

# Sync all dependencies from project.toml
c1 sync
```

## Commands

| Command | Description |
|---------|-------------|
| `c1 new <name>` | Create a new project in a new directory |
| `c1 init` | Initialize a new project in current directory |
| `c1 create <name>` | Create a new module (.c and .h files) |
| `c1 run` | Build and run the project |
| `c1 build [--release]` | Build the project (debug by default) |
| `c1 add <url> [--tag/--branch]` | Add a git dependency |
| `c1 sync` | Sync dependencies from project.toml |
| `c1 clean` | Clean the build directory |

## Configuration (project.toml)

```toml
[project]
name = "my_project"
version = "0.1.0"
edition = "c99"
description = "My awesome C project"

[dependencies]
arc-c = { git = "https://github.com/weynechen/arc-c.git", tag = "v0.5.0" }
cjson = { git = "https://github.com/DaveGamble/cJSON.git", branch = "master" }

[build]
compiler = "gcc"
flags = ["-O3", "-Wall", "-Wextra"]
```

## Project Structure

`c1` enforces the [Pitchfork Layout](https://api.csswg.org/bikeshed/?force=1&url=https://raw.githubusercontent.com/vector-of-bool/pitchfork/develop/data/spec.bs) convention:

```
project_root/
├── main.c              # Application entry point
├── CMakeLists.txt      # Build configuration
├── project.toml        # Project metadata
├── README.md
├── .gitignore
├── build/              # Build output
├── include/            # Public headers
├── src/                # Implementation files
└── external/           # Third-party dependencies
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
