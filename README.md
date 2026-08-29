# Grok Build 中文个人版

这是基于 [xai-org/grok-build](https://github.com/xai-org/grok-build) 的非官方个人维护版本。目标很窄：在 Windows 11 x64 上稳定使用 Grok Build，并补全常见 TUI、内置命令、内置技能、人设与工作流的中文说明。

> 本仓库公开用于保留来源、构建记录和许可证，不承诺兼容性、问题响应或社区支持。一般问题请先确认是否能在官方版本复现；本仓库只处理个人版本自身的构建和汉化问题。

## 安装

支持范围只有 **Windows 11 x64**。不发布 Windows ARM64、Windows 10、macOS 或 Linux 构建。

在 PowerShell 中运行：

```powershell
irm https://raw.githubusercontent.com/akinokoiri/grok-build-zh/zh-CN/install.ps1 | iex
grok-zh version
```

安装器会从本仓库最新 GitHub Release 下载固定名称的 Windows x64 压缩包，校验 SHA-256 后写入 `%USERPROFILE%\.grok\bin\grok-zh.exe`。旧可执行文件会保留到新版本通过冒烟测试为止。

## 更新策略

该版本永久关闭官方后台更新，不会在启动时查询或安装官方版本。需要更新时手动运行：

```powershell
grok-zh update
grok-zh update --check
```

显式更新只读取 `akinokoiri/grok-build-zh` 的 GitHub Release。版本格式为 `上游版本-zh.修订号`，例如 `0.2.106-zh.1`。

上游同步每周只创建一次候选 PR；只有重要功能或累计到值得发布时才手动发布。纯翻译改动只运行快速审计，Rust 代码改动才触发 Windows x64 编译。

## 汉化边界

当前集中式语言包位于 `crates/codegen/xai-grok-shared/i18n/zh-CN.json`：

- 汉化 TUI 设置、内置斜杠命令、内置技能、人设和工作流说明。
- 第三方技能、插件及用户内容保持原文，避免维护不可控的外部文本。
- `%USERPROFILE%\.grok\i18n\zh-CN.json` 可覆盖内置语言包；文件损坏、结构错误或超过 1 MiB 时自动回退到内置版本。
- 不在运行时调用机器翻译；上游新增文本由 LLM 辅助人工审阅后写入语言包。

## 维护与构建

协作 Agent 先读 [AGENTS.md](AGENTS.md) 和 [当前交接](docs/zh/HANDOFF.md)；详细流程见 [docs/zh/MAINTENANCE.md](docs/zh/MAINTENANCE.md)。核心门禁：

```powershell
./scripts/audit-translations.ps1
cargo fmt --all -- --check
cargo test -p xai-grok-shared i18n --lib
cargo test -p xai-proto-build dependency_tests --lib
cargo check -p xai-grok-pager-bin
```

源码许可证沿用上游的 [Apache-2.0](LICENSE)。上游源码快照标识保存在 [SOURCE_REV](SOURCE_REV)，个人版本信息保存在 [ZH_VERSION](ZH_VERSION)。
