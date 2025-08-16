# fd_rust

## 项目简介
fd_rust 是一个基于 Rust 开发的 Windows 平台快速命令行搜索工具。它类似于 Unix/Linux 系统中的 `find` 命令，但针对 Windows 平台进行了优化，支持文件名和文件内容搜索，具有高性能和丰富的命令行选项。

## 功能特点
- 支持正则表达式文件名搜索
- 支持文件内容搜索
- 支持大小写敏感/不敏感搜索
- 支持递归/非递归目录搜索
- 支持按文件扩展名过滤
- 支持显示上下文内容（-C 选项）
- 支持跳过隐藏/系统文件
- 彩色输出，提高可读性
- 针对Windows平台优化，支持处理隐藏和系统文件

## 环境要求
- Rust 1.70+
- Cargo（Rust 包管理器）

## 安装步骤
1. 克隆项目到本地：
```bash
git clone [项目地址]
```
2. 进入项目目录：
```bash
cd fd_rust
```
3. 构建项目：
```bash
cargo build --release
```
4. 将 `target/release` 目录下的可执行文件添加到系统 PATH 中

## 使用说明
### 基本用法
```bash
# 搜索当前目录下包含"test"的文件
fd_rust test

# 搜索指定目录
fd_rust pattern C:\path\to\directory

# 搜索文件内容
fd_rust -C pattern

# 大小写不敏感搜索
fd_rust -i pattern

# 限制搜索深度（不递归）d_rust --no-recurse pattern

# 按扩展名过滤
fd_rust --ext rs,md pattern

# 包含隐藏/系统文件
fd_rust --include-hidden pattern
```

### 命令行选项
- `pattern`: 搜索模式（正则表达式）
- `-d, --directory <DIR>`: 搜索目录（默认为当前目录）
- `-C, --content`: 搜索文件内容而非文件名
- `-i, --case-insensitive`: 大小写不敏感搜索
- `-R, --no-recurse`: 不递归搜索子目录
- `-x, --context <NUM>`: 显示匹配内容的上下文行数
n- `-e, --ext <EXTS>`: 按扩展名过滤（多个扩展名用逗号分隔）
- `-H, --include-hidden`: 包含隐藏/系统文件
## 项目结构
```
fd_rust/
├── src/               # 源代码目录
│   └── main.rs       # 主程序入口
├── Cargo.toml        # 项目配置文件
└── README.md         # 项目说明文档
```

## 依赖项
- `ansi-term`: 提供彩色终端输出
- `clap`: 命令行参数解析
- `ignore`: 高效文件系统遍历
- `regex`: 正则表达式支持
- `winapi`: Windows API 访问（用于处理隐藏/系统文件）

# 开发指南
## 代码规范
遵循 Rust 官方代码风格
使用 cargo fmt 格式化代码
使用 cargo clippy 进行代码检查

# 贡献指南
Fork 本仓库
创建特性分支 (git checkout -b feature/AmazingFeature)
提交更改 (git commit -m 'Add some AmazingFeature')
推送到分支 (git push origin feature/AmazingFeature)
创建 Pull Request