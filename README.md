<div align="center">

# Grok Build 汉化版（grok-build-zh）

**面向中文用户的 [Grok Build](https://github.com/xai-org/grok-build) 社区汉化分支**

把设置、权限确认等高频界面改成中文，本地编译后即可使用。

[![GitHub stars](https://img.shields.io/github/stars/ivan6232/grok-build-zh?style=social)](https://github.com/ivan6232/grok-build-zh)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Upstream](https://img.shields.io/badge/upstream-xai--org%2Fgrok--build-orange)](https://github.com/xai-org/grok-build)

[快速开始](#快速开始) · [已汉化内容](#已汉化内容) · [与官方差异](#与官方的关系) · [参与贡献](#参与贡献) · [Star 支持](#star--转发)

</div>

---

## 为什么做这个项目？

官方 Grok Build TUI 目前只有英文界面，设置项、权限弹窗、状态提示对中文用户不友好。

本仓库是 **xai-org/grok-build 的社区 Fork**，在 **Apache 2.0** 许可下：

- 汉化高频用户可见文案  
- 保持可跟随官方 upstream 同步  
- 欢迎 Issue / PR 一起完善翻译  

> 对话本身可以用中文；本项目解决的是 **软件 UI 语言**。

---

## 快速开始

### 方式 A：从源码编译（当前推荐）

依赖与官方相同：Rust（见 `rust-toolchain.toml`）、[DotSlash](https://dotslash-cli.com)、protoc 等。详见官方 README 的 Building from source。

```bash
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh
git checkout zh-CN   # 汉化主分支

# 安装 dotslash 后：
cargo build -p xai-grok-pager-bin --release

# 二进制一般在：
#   target/release/xai-grok-pager
# 可自行复制为 grok 并放入 PATH，例如：
mkdir -p ~/.local/bin
cp target/release/xai-grok-pager ~/.local/bin/grok-zh
~/.local/bin/grok-zh
```

也可：

```bash
cargo run -p xai-grok-pager-bin
```

### 方式 B：预编译包

GitHub Actions 将尝试在 Release 中提供构建产物（见 `.github/workflows/build.yml`）。若暂无产物，请用方式 A。

### 与官方版共存

不要覆盖官方 `grok` 也可，使用不同命令名如 `grok-zh`。

---

## 已汉化内容（持续扩展）

| 模块 | 状态 | 说明 |
|------|------|------|
| 设置分类标题 | ✅ | 外观 / 鼠标 / 编辑与输入 / 代理与授权… |
| 设置项名称 (label) | ✅ | 紧凑模式、权限模式、主题等 |
| 设置项说明 (description) | ✅ 大部分 | 选项说明中文 |
| 权限确认文案 | ✅ 核心 | 是 / 始终允许 / 否… |
| 底部快捷键 / 全局状态 | 🚧 | 欢迎 PR |
| 帮助文档 user-guide | 🚧 | 可选中文文档 |
| 完整 i18n 可切换语言 | 📋 规划中 | 长期目标：en/zh 可配置 |

---

## 与官方的关系

| | 官方 | 本仓库 |
|--|------|--------|
| 上游 | [xai-org/grok-build](https://github.com/xai-org/grok-build) | 本 Fork |
| 协议 | Apache 2.0 | 相同，保留 NOTICE |
| 目标 | 完整产品能力 | **中文 UI** + 同步上游能力 |
| 非目标 | — | 不修改模型、不替代 xAI 账号体系 |

同步上游示例：

```bash
git remote add upstream https://github.com/xai-org/grok-build.git  # 若尚未添加
git fetch upstream
git merge upstream/main   # 或 rebase，按需解决冲突
```

---

## 参与贡献

非常欢迎：

1. **补翻译**：提交 PR 修改 `crates/codegen/xai-grok-pager` / `xai-grok-shell` / `xai-grok-workspace` 中用户可见字符串  
2. **报漏网英文**：开 Issue，附截图与英文原文  
3. **i18n 架构**：若有 Rust 本地化经验，欢迎一起设计 `zh-CN`/`en` 资源文件方案  

翻译约定见 [docs/zh/TRANSLATING.md](docs/zh/TRANSLATING.md)。

---

## Star & 转发

如果这个汉化版帮到你：

1. 点一下右上角 **Star**，让更多中文用户看到  
2. 分享给同事 / 群友  
3. 提 PR 一起把界面做完整  

---

## 免责声明

- 本项目为**社区维护**，与 SpaceXAI / xAI **无官方隶属关系**  
- 「Grok」「Grok Build」等为相关权利人商标  
- 使用前请遵守 xAI 服务条款与 API/账号规定  
- 从源码编译与运行风险自负；请在可信环境审查代码后再使用  

---

## License

Apache License 2.0 — 见 [LICENSE](LICENSE) 与 [NOTICE](NOTICE)。  
基于 [xai-org/grok-build](https://github.com/xai-org/grok-build) 修改。
