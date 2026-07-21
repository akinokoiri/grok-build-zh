# 安装汉化版 Grok Build

社区维护 · **非官方**。安装后使用命令 **`grok-zh`** 进入中文界面。  
官方英文版请用：`curl -fsSL https://x.ai/cli/install.sh | bash`（命令为 `grok`）。

---

## 方式一：一键安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash
```

脚本会：

1. 识别 macOS / Linux 与 CPU 架构  
2. 从 [GitHub Releases](https://github.com/ivan6232/grok-build-zh/releases) 下载对应的 `grok-zh-*.tar.gz`  
3. 安装到 `~/.local/bin/grok-zh`  
4. 把 `~/.local/bin` 写入 shell 配置（`~/.zshrc` / `~/.bashrc` 等）  
5. 若尚无预编译包，则**自动回退为源码编译**

安装完成后：

```bash
# 若当前终端找不到命令
export PATH="$HOME/.local/bin:$PATH"

grok-zh --version
grok-zh
```

### 可选：内置人设描述中文

装完 `grok-zh` 后，若希望代理/人设列表的 **description** 也是中文（ID 仍为英文）：

```bash
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh && git checkout zh-CN
bash docs/zh/bundled-zh/apply.sh
```

详见 [bundled-zh/README.md](bundled-zh/README.md)。操作习惯见 [USAGE.md](USAGE.md)。

### 常用选项

| 用法 | 说明 |
|------|------|
| `bash -s -- v0.1.0-zh.4` | 安装指定 Release 标签 |
| `GROK_ZH_BIN_DIR=~/bin bash` | 自定义安装目录 |
| `GROK_ZH_FROM_SOURCE=1 bash` | 强制源码编译 |
| `GROK_ZH_NO_PATH=1 bash` | 不修改 shell 配置文件 |

完整示例：

```bash
# 指定版本
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash -s -- v0.1.0-zh.4

# 强制源码
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | GROK_ZH_FROM_SOURCE=1 bash
```

---

## 方式二：下载 Release 手动安装

1. 打开 [Releases](https://github.com/ivan6232/grok-build-zh/releases)  
2. 下载对应平台包：

| 平台 | 文件名 |
|------|--------|
| macOS Apple Silicon (M 系列) | `grok-zh-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `grok-zh-x86_64-apple-darwin.tar.gz` |
| Linux x64 | `grok-zh-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `grok-zh-aarch64-unknown-linux-gnu.tar.gz` |

3. 解压并安装：

```bash
tar -xzf grok-zh-*.tar.gz
# 压缩包内目录中有 grok-zh
chmod +x */grok-zh
mkdir -p ~/.local/bin
mv */grok-zh ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc   # bash 用户改 ~/.bashrc
source ~/.zshrc
grok-zh
```

---

## 方式三：源码编译

依赖：

- Rust（见仓库 `rust-toolchain.toml`，建议 [rustup](https://rustup.rs)）
- [DotSlash](https://dotslash-cli.com)
- `protoc`（protobuf 编译器）

```bash
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh
git checkout zh-CN

cargo install dotslash
# macOS: brew install protobuf
# Debian/Ubuntu: sudo apt-get install -y protobuf-compiler pkg-config libssl-dev

export PROTOC="$(command -v protoc)"
cargo build -p xai-grok-pager-bin --release

mkdir -p ~/.local/bin
cp target/release/xai-grok-pager ~/.local/bin/grok-zh
export PATH="$HOME/.local/bin:$PATH"
grok-zh
```

开发调试：

```bash
cargo run -p xai-grok-pager-bin
```

---

## 与官方 `grok` 并存

| 命令 | 说明 |
|------|------|
| `grok` | 官方安装脚本安装的英文版 |
| `grok-zh` | 本仓库汉化版 |

配置目录（如 `~/.grok`）、登录与 API 行为与上游一致。

---

## 卸载

```bash
rm -f ~/.local/bin/grok-zh
# 可选：从 ~/.zshrc / ~/.bashrc 删除「grok-zh 汉化版」相关 PATH 段落
```

---

## 常见问题

**Q: 对话已经是中文，为什么还要汉化？**  
A: 对话语言由模型决定；本项目汉化的是软件菜单、设置、权限确认等 **UI**。

**Q: `command not found: grok-zh`？**  
A: 把安装目录加入 PATH：`export PATH="$HOME/.local/bin:$PATH"`，并重新打开终端。也可用绝对路径 `~/.local/bin/grok-zh`。

**Q: 一键安装提示没有 Release？**  
A: 脚本会自动尝试源码编译。也可等待 [Actions / Releases](https://github.com/ivan6232/grok-build-zh/releases) 出包后再装。

**Q: 能否一键切换回英文？**  
A: 当前为「硬编码中文」分支。需要英文请使用官方发行版。

**Q: 会不会上传隐私？**  
A: 请自行阅读上游与本仓库源码及官方安全说明；构建与运行请在可信环境进行。

**Q: Windows？**  
A: 请使用 WSL2，或在 Linux/macOS 上安装。暂无原生 Windows 预编译包。

---

## 下一步

- 使用教程：[USAGE.md](USAGE.md)
- 内置人设中文描述：[bundled-zh/README.md](bundled-zh/README.md)
- 翻译贡献：[TRANSLATING.md](TRANSLATING.md)
