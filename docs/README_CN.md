# c1

一个现代化的 C 语言项目脚手架与包管理工具，像 cargo 一样管理 C 项目。

## 特性

- **项目脚手架**：`c1 new` / `c1 init` - 使用标准目录结构快速创建 C 项目
- **构建系统**：`c1 build` / `c1 run` - 一条命令编译运行项目
- **模块管理**：`c1 create` - 原子化创建源文件/头文件对
- **依赖管理**：`c1 add` / `c1 sync` - 基于 Git 的依赖管理
- **Pitchfork 布局**：强制执行标准 C 项目目录结构
- **现代 CMake**：自动化构建配置管理

## 安装

### 从源码安装

```bash
git clone https://github.com/yourusername/c1.git
cd c1
cargo build --release
```

二进制文件将位于 `target/release/c1`。

## 快速开始

### 创建新项目

```bash
# 创建一个新目录并初始化项目
c1 new my_project
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
└── external/           # 外部依赖
```

### 编译运行

```bash
# 编译并运行（debug 模式）
c1 run

# 仅编译
c1 build

# release 模式编译
c1 build --release
```

### 创建新模块

```bash
c1 create utils
```

这将生成：
- `src/utils.c` - 源文件，包含 `#include "utils.h"`
- `include/utils.h` - 头文件，包含头文件保护宏

并自动更新 `CMakeLists.txt` 添加新文件。

### 添加依赖

```bash
# 添加 git 依赖
c1 add https://github.com/DaveGamble/cJSON.git

# 指定 tag
c1 add https://github.com/weynechen/arc-c.git --tag v0.5.0

# 指定分支
c1 add https://github.com/example/lib.git --branch develop

# 从 project.toml 同步所有依赖
c1 sync
```

## 命令

| 命令 | 描述 |
|------|------|
| `c1 new <name>` | 创建新目录并初始化项目 |
| `c1 init` | 在当前目录初始化项目 |
| `c1 create <name>` | 创建新模块（生成 .c 和 .h 文件） |
| `c1 run` | 编译并运行项目 |
| `c1 build [--release]` | 编译项目（默认 debug 模式） |
| `c1 add <url> [--tag/--branch]` | 添加 git 依赖 |
| `c1 sync` | 从 project.toml 同步依赖 |
| `c1 clean` | 清除 build 目录 |

## 配置文件 (project.toml)

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

## 项目结构

`c1` 强制执行 [Pitchfork 布局](https://api.csswg.org/bikeshed/?force=1&url=https://raw.githubusercontent.com/vector-of-bool/pitchfork/develop/data/spec.bs) 规范：

```
project_root/
├── main.c              # 应用程序入口点
├── CMakeLists.txt      # 构建配置
├── project.toml        # 项目元数据
├── README.md
├── .gitignore
├── build/              # 构建输出
├── include/            # 公共头文件
├── src/                # 实现文件
└── external/           # 第三方依赖
```

## 许可证

MIT 许可证 - 详情请参见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 issue 或 pull request。
