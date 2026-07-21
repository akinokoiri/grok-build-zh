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

[一键安装](#一键安装推荐) · [使用说明](#使用说明) · [操作教程](docs/zh/USAGE.md) · [已汉化范围](#已汉化范围) · [与官方区别](#与官方版的区别) · [参与贡献](#参与贡献)

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

对照「欢迎页 → 快捷键 → 模式横幅 → 会话恢复 → 扩展/MCP → 设置 → 仪表盘 → 人设 → tips → 市场」逐项进度：

| 模块 | 状态 | 说明 |
|------|------|------|
| 欢迎页 / 首屏 | ✅ 已完成 | 新建工作树、恢复会话、更新日志、退出等 |
| 快捷键帮助 | ✅ 已完成 | 分类名、底栏、搜索；键位本身保持英文 |
| 模式横幅 | ✅ 已完成 | 「已切换模式：…」+ CJK 显示宽度修复 |
| 会话 / 目标恢复列表 | ✅ 已完成 | 相对时间（不到1分钟 / N分钟前 等） |
| 扩展 / 技能 / MCP 面板 | ✅ 已完成 | 底栏、统计、徽章、筛选（已启用/已禁用） |
| 设置面板（枚举 / 开关） | ✅ 已完成 | 开/关/自动、滚轮/触控板、全屏/极简、按住说话、语音语言等 |
| 仪表盘 / 任务 | ✅ 已完成 | 标题、空态、已固定、分组（工作中/空闲/等待中…）、任务面板分组 |
| 人设 / 角色 / 代理 **description** | ✅ 可选覆盖 | ID 英文；描述中文，见 [bundled-zh](docs/zh/bundled-zh/) |
| **内置技能 description** | ✅ 可选覆盖 | ID 英文（如 `pptx`）；列表描述中文，见 `bundled-zh/skills` |
| 常见 Tips（剪贴板/规划/选词/SSH/排队…） | ✅ 已完成 | 核心提示均中文 |
| 用户指南 `/docs` | ✅ 中文 | 部分长文精简 |
| 权限 / 状态栏 / 回合事件 / Toast | ✅ 核心路径 | |
| 市场插件 / 第三方技能 **远程描述** | ⚠️ 不可控 | 上游英文；**ID 与远程正文均不随本仓库改** |
| 可配置 en/zh 运行时切换（正规 i18n） | 📋 规划中 | 当前为硬编码中文分支 |

**原则：**

| | ID / 名称 | 列表 description |
|--|-----------|------------------|
| 人设 / 角色 / 代理 / **内置技能** | 英文 | **中文**（`bash docs/zh/bundled-zh/apply.sh`） |
| 市场 / 第三方插件技能 | 英文 | ⚠️ 远程原文（多为英文） |

工具短名（`Bash` / `Read` / `Edit`）、模型名、斜杠命令名、主题商品名保持英文。完整 100% 需社区持续 PR。欢迎对照截图提 Issue「漏网英文」。

操作教程：[docs/zh/USAGE.md](docs/zh/USAGE.md) · 安装详解：[docs/zh/INSTALL.md](docs/zh/INSTALL.md)

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
