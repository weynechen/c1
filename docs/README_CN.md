# c1

一个现代化的 C 语言项目脚手架工具，灵感来自 `uv` (Python)。

`c1` 通过解决"初始化繁琐"的痛点，简化 C 语言开发流程。

## 特性

- **项目初始化**：使用标准目录结构快速搭建新 C 项目
- **模块管理**：原子化创建源文件/头文件对，自动更新 CMakeLists.txt
- **Pitchfork 布局**：强制执行标准 C 项目目录结构
- **现代 CMake**：自动化构建配置管理
- **简化依赖拉取**：轻量级的依赖 git clone 助手（试验性功能）

## 安装

### 从源码安装

```bash
git clone https://github.com/yourusername/c1.git
cd c1
cargo build --release
```

二进制文件将位于 `target/release/c1`。

## 快速开始

### 初始化新项目

```bash
# 创建一个带独立目录的新项目
c1 init my_project
cd my_project

# 或在当前目录初始化（必须为空目录）
mkdir my_project && cd my_project
c1 init
```

这将创建以下目录结构：

```
my_project/
├── main.c              # 入口点，包含 "Hello my_project"
├── CMakeLists.txt      # 预配置源文件/头文件锚点
├── project.toml        # 项目配置
├── README.md           # 项目文档
├── .gitignore          # Git 忽略规则
├── build/              # 构建输出目录
├── include/            # 头文件
├── src/                # 额外的源文件
└── external/           # 外部依赖（可选）
```

### 创建新模块

```bash
c1 create utils
```

这将生成：
- `src/utils.c` - 源文件，包含 `#include "utils.h"`
- `include/utils.h` - 头文件，包含头文件保护宏

并自动更新 `CMakeLists.txt` 添加新文件。

## 命令

| 命令 | 描述 |
|---------|-------------|
| `c1 init [name]` | 初始化新项目。如果提供名称则创建对应目录。 |
| `c1 create <name>` | 创建新模块（生成 .c 和 .h 文件）。 |
| `c1 sync` | 简化依赖拉取（git clone 助手）。 |

## 配置 (project.toml)

```toml
[project]
name = "my_project"
version = "0.1.0"
edition = "99"          # C 标准：99, 11, 17
description = "My awesome C project"

[dependencies]
# 简化依赖拉取（试验性功能）
# cjson = { git = "https://github.com/DaveGamble/cJSON.git", tag = "v1.7.18" }

[build]
compiler = "gcc"
flags = ["-O3", "-Wall", "-Wextra"]
```

## 构建项目

标准 CMake 工作流：

```bash
cmake -B build
cmake --build build
```

或使用预创建的 build 目录：

```bash
cd build
cmake ..
make
```

## 项目结构

`c1` 强制执行 [Pitchfork 布局](https://api.csswg.org/bikeshed/?force=1&url=https://raw.githubusercontent.com/vector-of-bool/pitchfork/develop/data/spec.bs) 规范：

```
project_root/
├── main.c              # 应用程序入口点
├── CMakeLists.txt      # 构建配置
├── project.toml        # 项目元数据
├── README.md
├── .gitignore
├── build/              # 构建输出（内容 gitignored，保留目录）
├── include/            # 公共头文件
├── src/                # 实现文件
└── external/           # 第三方依赖
```

## 许可证

MIT 许可证 - 详情请参见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 issue 或 pull request。
