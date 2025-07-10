# 安装指南

本指南介绍在不同平台上安装 `diffx` 的各种方法。

## 快速安装

### 从 Crates.io 安装（推荐）

安装 `diffx` 最简单的方法是使用 Cargo：

```bash
cargo install diffx
```

这将从 [crates.io](https://crates.io/crates/diffx) 下载、编译并安装最新版本的 `diffx`。

## 平台特定安装

### Linux

#### Ubuntu/Debian

```bash
# 如果尚未安装 Rust，请先安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装 diffx
cargo install diffx
```

#### Arch Linux

```bash
# 安装 Rust
sudo pacman -S rust

# 安装 diffx
cargo install diffx
```

#### Alpine Linux

```bash
# 安装 Rust
apk add rust cargo

# 安装 diffx
cargo install diffx
```

### macOS

#### 使用 Homebrew（即将推出）

```bash
# 即将推出
brew install diffx
```

#### 使用 Cargo

```bash
# 通过 Homebrew 安装 Rust
brew install rust

# 或通过 rustup 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 diffx
cargo install diffx
```

### Windows

#### 使用 Cargo

```powershell
# 从 https://rustup.rs/ 安装 Rust
# 然后安装 diffx
cargo install diffx
```

#### 使用 Scoop（即将推出）

```powershell
# 即将推出
scoop install diffx
```

#### 使用 Chocolatey（即将推出）

```powershell
# 即将推出
choco install diffx
```

## 从源代码构建

### 前提条件

- Rust 1.70.0 或更高版本
- Git

### 克隆和构建

```bash
# 克隆仓库
git clone https://github.com/kako-jun/diffx.git
cd diffx

# 构建并安装
cargo install --path diffx-cli

# 或仅用于开发的构建
cargo build --release
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试套件
cargo test --package diffx-core
cargo test --package diffx-cli

# 运行集成测试
cargo test --test integration
```

## Docker 安装

### 使用预构建镜像（即将推出）

```bash
# 即将推出
docker pull ghcr.io/kako-jun/diffx:latest
docker run --rm -v $(pwd):/workspace ghcr.io/kako-jun/diffx file1.json file2.json
```

### 构建自己的镜像

```dockerfile
FROM rust:1.70-alpine AS builder

WORKDIR /app
COPY . .
RUN cargo install --path diffx-cli

FROM alpine:latest
RUN apk add --no-cache libc6-compat
COPY --from=builder /usr/local/cargo/bin/diffx /usr/local/bin/diffx
ENTRYPOINT ["/usr/local/bin/diffx"]
```

```bash
# 构建镜像
docker build -t diffx .

# 使用镜像
docker run --rm -v $(pwd):/workspace diffx /workspace/file1.json /workspace/file2.json
```

## 包管理器（未来计划）

### Node.js 生态系统

```bash
# 即将推出
npm install diffx-js
npx diffx-js file1.json file2.json
```

### Python 生态系统

```bash
# 即将推出
pip install diffx-python
diffx file1.json file2.json
```

## 验证

安装后，验证 `diffx` 是否正常工作：

```bash
# 检查版本
diffx --version

# 运行简单测试
echo '{"a": 1}' > test1.json
echo '{"a": 2}' > test2.json
diffx test1.json test2.json

# 预期输出：
# ~ a: 1 -> 2

# 清理
rm test1.json test2.json
```

## 更新

### 通过 Cargo 更新

```bash
cargo install diffx --force
```

### 检查更新

```bash
# 检查当前版本
diffx --version

# 在 crates.io 上检查最新版本
cargo search diffx
```

## 故障排除

### 常见问题

#### 未找到 Rust

如果您收到找不到 `cargo` 或 `rust` 的错误：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 权限被拒绝

如果在安装过程中遇到权限错误：

```bash
# 在 Linux/macOS 上，确保 ~/.cargo/bin 在您的 PATH 中
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### 编译错误

如果编译失败：

```bash
# 将 Rust 更新到最新版本
rustup update

# 清理 cargo 缓存并重试
cargo clean
cargo install diffx
```

#### 编译时内存不足

对于内存有限的系统：

```bash
# 使用较少的并行任务
cargo install diffx --jobs 1
```

### 获取帮助

如果您遇到问题：

1. 查看 [FAQ](faq_zh.md)
2. 搜索现有的 [GitHub issues](https://github.com/kako-jun/diffx/issues)
3. 创建一个新的 issue，并包含：
   - 您的操作系统和版本
   - Rust 版本（`rustc --version`）
   - 完整的错误信息
   - 重现步骤

## 卸载

要删除 `diffx`：

```bash
# 卸载 diffx
cargo uninstall diffx

# 删除任何配置文件（可选）
rm -rf ~/.config/diffx
```

## 系统要求

### 最低要求

- **内存**: 256 MB 可用内存
- **磁盘**: 50 MB 可用空间用于二进制文件
- **CPU**: 任何现代 x86_64 或 ARM64 处理器

### 推荐要求

- **内存**: 1 GB 或更多用于处理大文件
- **磁盘**: 500 MB 用于源代码构建
- **CPU**: 多核处理器用于并行处理

### 支持的平台

- **Linux**: x86_64, ARM64
- **macOS**: x86_64, ARM64 (Apple Silicon)
- **Windows**: x86_64
- **FreeBSD**: x86_64 (社区支持)

## 性能考虑

### 大文件

对于非常大的文件（>100MB），请考虑：

```bash
# 使用流模式（如果可用）
diffx --stream large1.json large2.json

# 增加内存限制
diffx --help
diffx large1.json large2.json
```

### 多个文件

对于批处理：

```bash
# 使用并行处理
find . -name "*.json" -print0 | xargs -0 -P $(nproc) -I {} diffx {} {}.backup
```