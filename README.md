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

[安装使用](#安装与使用) · [已汉化范围](#已汉化范围) · [与官方区别](#与官方版的区别) · [同步上游](#同步官方更新) · [参与贡献](#参与贡献)

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

本仓库在源码层把这些**用户可见文案**改成中文，编译后即可使用中文界面。

---

## 安装与使用

### 方式一：下载预编译二进制（推荐）

1. 打开 [Releases](https://github.com/ivan6232/grok-build-zh/releases)  
2. 按系统下载：
   - macOS Apple Silicon：`grok-zh-aarch64-apple-darwin.tar.gz`
   - macOS Intel：`grok-zh-x86_64-apple-darwin.tar.gz`
   - Linux x64：`grok-zh-x86_64-unknown-linux-gnu.tar.gz`
3. 解压并安装到 PATH：

```bash
# 示例：macOS / Linux
tar -xzf grok-zh-*.tar.gz
chmod +x grok-zh
mkdir -p ~/.local/bin
mv grok-zh ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc   # 或 ~/.bashrc
source ~/.zshrc

grok-zh --version   # 或 grok-zh
```

> 若 Releases 尚无对应平台包，请用下方「源码编译」。

### 方式二：源码编译

依赖与官方一致：

- Rust（见仓库 `rust-toolchain.toml`，建议用 rustup）
- [DotSlash](https://dotslash-cli.com)（构建 hermetic 工具需要）
- protoc（可通过 DotSlash 的 `bin/protoc`）

```bash
# 1. 克隆汉化分支
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh
git checkout zh-CN

# 2. 安装 DotSlash（首次）
cargo install dotslash
# 确认：
dotslash --help

# 3. 编译
cargo build -p xai-grok-pager-bin --release

# 4. 安装为 grok-zh（不覆盖官方 grok）
mkdir -p ~/.local/bin
cp target/release/xai-grok-pager ~/.local/bin/grok-zh
~/.local/bin/grok-zh
```

开发调试：

```bash
cargo run -p xai-grok-pager-bin
```

### 与官方 `grok` 并存

| 命令 | 说明 |
|------|------|
| `grok` | 官方安装脚本安装的英文版 |
| `grok-zh` | 本汉化版（建议这样命名，避免覆盖） |

登录、API Key、配置目录（如 `~/.grok`）等行为与上游一致，**请自行阅读上游文档与隐私说明**。

### 基本使用

```bash
# 在项目目录启动 TUI
cd /path/to/your/project
grok-zh

# 常用（与官方 slash 命令相同，界面文案为中文）
# /settings   打开设置
# /model      切换模型
# /help       帮助
```

配置文件仍为 `~/.grok/config.toml` 等，键名保持英文（与上游兼容）。

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
| 安装包 | x.ai/cli | GitHub Releases / 源码编译 |
| 默认分支 | main | **zh-CN** |

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
4. 提交 PR，最好附图  

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
