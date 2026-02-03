# c1

A modern C project scaffolding tool, inspired by `uv` (Python).

`c1` simplifies C language development by solving the pain point of "tedious initialization".

## Features

- **Project Initialization**: Quickly scaffold a new C project with standard directory structure
- **Module Management**: Atomic creation of source/header file pairs with automatic CMakeLists.txt updates
- **Pitchfork Layout**: Enforces standard C project directory structure
- **Modern CMake**: Automated build configuration management
- **Simple Dependency Fetching**: Lightweight git clone helper for dependencies (experimental)

## Installation

### From Source

```bash
git clone https://github.com/yourusername/c1.git
cd c1
cargo build --release
```

The binary will be available at `target/release/c1`.

## Quick Start

### Initialize a New Project

```bash
# Create a new project with its own directory
c1 init my_project
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
└── external/           # External dependencies (optional)
```

### Create a New Module

```bash
c1 create utils
```

This generates:
- `src/utils.c` - Source file with `#include "utils.h"`
- `include/utils.h` - Header file with include guard

And automatically updates `CMakeLists.txt` with the new files.

## Commands

| Command | Description |
|---------|-------------|
| `c1 init [name]` | Initialize a new project. Creates a directory if name is provided. |
| `c1 create <name>` | Create a new module (generates .c and .h files). |
| `c1 sync` | Simple dependency fetcher (git clone helper). |

## Configuration (project.toml)

```toml
[project]
name = "my_project"
version = "0.1.0"
edition = "99"          # C standard: 99, 11, 17
description = "My awesome C project"

[dependencies]
# Simple git clone helper (experimental)
# cjson = { git = "https://github.com/DaveGamble/cJSON.git", tag = "v1.7.18" }

[build]
compiler = "gcc"
flags = ["-O3", "-Wall", "-Wextra"]
```

## Building Your Project

Standard CMake workflow:

```bash
cmake -B build
cmake --build build
```

Or using the pre-created build directory:

```bash
cd build
cmake ..
make
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
├── build/              # Build output (gitignored content, directory kept)
├── include/            # Public headers
├── src/                # Implementation files
└── external/           # Third-party dependencies
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
