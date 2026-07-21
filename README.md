<div align="center">

# Grok Build 汉化版（grok-build-zh）

### 社区维护 · 非官方 · 开源免费

把 [Grok Build](https://github.com/xai-org/grok-build) 终端界面（设置、权限、状态栏等）翻译成**简体中文**。

[![GitHub stars](https://img.shields.io/github/stars/ivan6232/grok-build-zh?style=social)](https://github.com/ivan6232/grok-build-zh/stargazers)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Upstream](https://img.shields.io/badge/upstream-xai--org%2Fgrok--build-orange)](https://github.com/xai-org/grok-build)
[![Release](https://img.shields.io/github/v/release/ivan6232/grok-build-zh?include_prereleases)](https://github.com/ivan6232/grok-build-zh/releases)

**本项目是社区汉化分支，与 SpaceXAI / xAI 无官方隶属关系。**  
「Grok」「Grok Build」为相关权利人商标。使用请遵守 xAI 服务条款与账号规定。

[一键安装](#一键安装推荐) · [使用说明](#使用说明) · [已汉化范围](#已汉化范围) · [与官方区别](#与官方版的区别) · [参与贡献](#参与贡献)

</div>

---

## 重要声明（请先读）

| | |
|--|--|
| **性质** | 社区爱好者维护的 **非官方汉化** |
| **上游** | 基于 [xai-org/grok-build](https://github.com/xai-org/grok-build)（Apache 2.0） |
| **不包含** | 官方账号代充、破解、API 密钥分发 |
| **风险** | 请自行审查代码后再编译运行；生产环境请谨慎 |
| **商标** | 未获 xAI 背书；不得伪称为官方发行版 |

如果你需要**纯官方、英文界面**，请使用：

```bash
curl -fsSL https://x.ai/cli/install.sh | bash
```

---

## 这个汉化版解决什么问题？

Grok Build 的 **对话**可以用中文，但 **软件 UI**（权限弹窗、设置页、状态提示「Responding…」等）默认是英文。

本仓库在源码层把这些**用户可见文案**改成中文，编译后即可使用中文界面。命令名为 **`grok-zh`**，不会覆盖官方的 `grok`。

---

## 一键安装（推荐）

与官方类似的 `curl | bash` 方式：

```bash
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash
```

装完后：

```bash
grok-zh
```

若提示找不到命令，先执行：

```bash
export PATH="$HOME/.local/bin:$PATH"
grok-zh
```

脚本会自动：

1. 识别系统架构并下载 [Releases](https://github.com/ivan6232/grok-build-zh/releases) 中的预编译包  
2. 安装到 `~/.local/bin/grok-zh`  
3. 写入 shell `PATH`（`~/.zshrc` / `~/.bashrc` 等）  
4. **若暂无预编译包**，回退为本地源码编译  

### 可选参数

```bash
# 安装指定版本
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash -s -- v0.1.0-zh.4

# 强制从源码编译
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | GROK_ZH_FROM_SOURCE=1 bash

# 自定义安装目录
curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | GROK_ZH_BIN_DIR="$HOME/bin" bash
```

更细的说明见 [docs/zh/INSTALL.md](docs/zh/INSTALL.md)。

---

## 其他安装方式

### 下载预编译二进制

| 平台 | 资产名 |
|------|--------|
| macOS Apple Silicon | `grok-zh-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `grok-zh-x86_64-apple-darwin.tar.gz` |
| Linux x64 | `grok-zh-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `grok-zh-aarch64-unknown-linux-gnu.tar.gz` |

在 [Releases](https://github.com/ivan6232/grok-build-zh/releases) 下载后：

```bash
tar -xzf grok-zh-*.tar.gz
chmod +x */grok-zh
mkdir -p ~/.local/bin && mv */grok-zh ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
grok-zh --version
```

### 源码编译

依赖：Rust（`rust-toolchain.toml`）、[DotSlash](https://dotslash-cli.com)、`protoc`。

```bash
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh && git checkout zh-CN
cargo install dotslash
# macOS: brew install protobuf
# Ubuntu: sudo apt-get install -y protobuf-compiler pkg-config libssl-dev
export PROTOC="$(command -v protoc)"
cargo build -p xai-grok-pager-bin --release
mkdir -p ~/.local/bin
cp target/release/xai-grok-pager ~/.local/bin/grok-zh
grok-zh
```

---

## 使用说明

### 与官方 `grok` 并存

| 命令 | 说明 |
|------|------|
| `grok` | 官方英文版（`curl -fsSL https://x.ai/cli/install.sh \| bash`） |
| `grok-zh` | **本汉化版** |

登录、API Key、配置目录（`~/.grok`）等与上游一致。

### 基本操作

```bash
# 在项目目录启动中文 TUI
cd /path/to/your/project
grok-zh

# 带初始提示
grok-zh "帮我梳理这个仓库的结构"

# 继续最近会话
grok-zh -c

# 查看版本 / 帮助
grok-zh --version
grok-zh --help
```

应用内常用（界面文案为中文，斜杠命令与上游兼容）：

| 命令 | 说明 |
|------|------|
| `/settings` | 设置 |
| `/model` | 切换模型 |
| `/help` | 帮助 |
| `/docs` | 用户指南（应用内中文文档） |

配置文件：`~/.grok/config.toml`（**键名保持英文**，与上游兼容）。

### 卸载

```bash
rm -f ~/.local/bin/grok-zh
# 可选：删除 shell 配置中「grok-zh 汉化版」PATH 段落
```

---

## 已汉化范围

| 模块 | 状态 |
|------|------|
| 设置分类 / 设置项名称 | ✅ 已完成 |
| 设置项说明文字 | ✅ 大部分 |
| 权限确认（是 / 始终允许 / 否…） | ✅ 核心路径 |
| 状态栏：正在回复、思考中、压缩中、等待… | ✅ |
| 回合事件：已工作、回合失败、上下文压缩… | ✅ |
| 快捷键条：Esc 取消/关闭 等 | ✅ 部分 |
| 仪表盘：等待你的输入、新会话 等 | ✅ 部分 |
| 用户指南（应用内 /docs/user-guide） | ✅ 已译为中文（部分长文为精简全中文版） |
| 常见 Toast / 失败提示 | ✅ 已覆盖一批 |
| 全部 tips 长文案 | 🚧 仍可继续补 |
| 可配置 en/zh 切换（正规 i18n） | 📋 规划中 |

完整 100% 文案需要社区持续 PR。欢迎对照英文截图提 Issue「漏网英文」。

---

## 与官方版的区别

| | 官方 Grok Build | 本仓库 grok-build-zh |
|--|-----------------|----------------------|
| 维护方 | SpaceXAI / xAI | 社区 |
| 界面语言 | 英文 | **简体中文（硬编码）** |
| 功能能力 | 完整上游能力 | 同步上游，另加汉化补丁 |
| 安装 | `curl … x.ai/cli/install.sh` → `grok` | `curl … install.sh` → **`grok-zh`** |
| 默认分支 | main | **zh-CN** |

---

## 发布与 CI

- 推送标签 `v*`（例如 `v0.1.0-zh.4`）会触发 [Release 工作流](.github/workflows/release.yml)  
- 为 macOS (arm64/x64)、Linux (x64/arm64) 编译并上传 `grok-zh-*.tar.gz`  
- 也可在 Actions 里 **workflow_dispatch** 手动发版  

维护者示例：

```bash
git checkout zh-CN
git pull
git tag v0.1.0-zh.4
git push origin v0.1.0-zh.4
```

---

## 同步官方更新

```bash
git remote add upstream https://github.com/xai-org/grok-build.git  # 只需一次
git fetch upstream
git checkout zh-CN
git merge upstream/main   # 或 rebase
# 解决冲突后：
git push origin zh-CN
```

---

## 参与贡献

1. Fork 本仓库，分支建议：`i18n/xxx`  
2. 优先改用户可见字符串，勿改 option id、配置键  
3. 术语见 [docs/zh/TRANSLATING.md](docs/zh/TRANSLATING.md)  
4. 安装说明见 [docs/zh/INSTALL.md](docs/zh/INSTALL.md)  
5. 提交 PR，最好附图  

高频文件：

- `crates/codegen/xai-grok-pager/src/settings/`  
- `crates/codegen/xai-grok-pager/src/views/turn_status.rs`  
- `crates/codegen/xai-grok-workspace/src/permission/prompter.rs`  
- `crates/codegen/xai-grok-pager/src/scrollback/blocks/session_event.rs`  

---

## Star 与传播

如果对你有用：

1. 点仓库 **Star**  
2. 分享给需要中文界面的朋友  
3. 提 PR 补翻译  

---

## License

Apache License 2.0 — 见 [LICENSE](LICENSE)、[NOTICE](NOTICE)。  
基于 [xai-org/grok-build](https://github.com/xai-org/grok-build) 修改。
