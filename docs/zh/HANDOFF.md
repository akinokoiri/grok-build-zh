# 当前交接

更新时间：2026-08-29（Asia/Shanghai）

## 当前结论

项目已经收敛为 Windows 11 x64 个人汉化发行版。首个稳定个人版本 `v1.0.12-zh.1` 已发布，本机也已安装并通过版本检查。官方后台更新已关闭；后续更新只能显式读取本仓库 Release。

当前功能实现基线截至 `c299266d`；本交接文档本身位于其后的维护提交。其中：

- `bec8523b`：重建集中式汉化、Windows x64 发布、安装/更新和门禁；`v1.0.12-zh.1` 从该提交构建。
- `e059a91e`：发布门禁复用 Release profile，避免同一依赖图重复编译。
- `26d7405e`：Windows 工作流启用 `sccache`。
- `c299266d`：固定维护中的 GitHub Actions 版本。

`zh-CN` 是远端默认维护分支；`codex/windows-x64-i18n-core` 是本轮实现分支。`legacy/full-fork-2026-08-29` 保留旧完整汉化分支，不要删除。

## 已发布与本机状态

- Release：`v1.0.12-zh.1`
- 发布页：<https://github.com/akinokoiri/grok-build-zh/releases/tag/v1.0.12-zh.1>
- 资产：`grok-zh-x86_64-pc-windows-msvc.zip` 及其 `.sha256`
- 本机命令：`C:\Users\akino\.grok\bin\grok-zh.exe`
- 本机报告版本：`grok 1.0.12-zh.1 (bec8523b7e82) [stable]`
- 本机语言包：`C:\Users\akino\.grok\i18n\zh-CN.json`
- 本机更新脚本：`C:\Users\akino\.grok\bin\install-grok-zh.ps1`

发布产物已经由安装器校验 SHA-256；本机可执行文件的当前 SHA-256 为 `B41427B04C315E729737614B04E2F5A456E09A0C79B0AAAC364821EA6BEE661E`。

## 汉化与门禁快照

集中式语言包当前有 378 条：

- 25 个内置斜杠命令说明已覆盖。
- 72 个内置动作/快捷键说明已覆盖。
- 审计脚本仍报告 297 个高置信英语候选。这是供 LLM 逐批审阅的队列，不表示应当机械地全部翻译；第三方、协议、测试和诊断文本必须继续排除。

本地翻译审计当前通过。核心入口：

- `crates/codegen/xai-grok-shared/src/i18n.rs`
- `crates/codegen/xai-grok-shared/i18n/zh-CN.json`
- `crates/codegen/xai-grok-shared/i18n/schema.json`
- `scripts/audit-translations.ps1`

## CI 状态与耗时基线

Windows 校验和发布使用 `sccache 0.17.0`，Cargo 注册表由 `Swatinem/rust-cache` 缓存，但不缓存 `target/`。`CARGO_INCREMENTAL=0` 是为了保证编译任务可被 sccache 命中。

2026-08-29 同提交对照：

- 冷运行：19 分 56 秒。
- 缓存重跑：11 分 55 秒。
- 1637 个可缓存 Rust 任务中命中 1440 个，命中率 87.97%。
- 固定 Action 版本后的最终验证约 12 分 42 秒，命中率 89.37%，无 sccache 错误。

不要把首次冷编译视作稳定耗时；上游大改或编译参数变化会自然降低命中率。

## 下一次任务建议顺序

1. 获取 `official/main`，先确认是否有值得同步的重要变更。
2. 使用每周同步候选 PR，不自动合并、不自动发布。
3. 解决上游逻辑/编译冲突后运行翻译审计，让 LLM 只审阅新增且在维护边界内的用户可见文本。
4. 运行 Windows x64 门禁；积累到重要功能或修复后再以 `<上游版本>-zh.<修订号>` 发布。
5. 发布后更新本文件中的维护提交、Release、本机安装和 CI 基线。

## 已知但非阻塞事项

- Release 二进制来自 `bec8523b`；后续两个提交只调整 CI 缓存与 Action 固定策略，因此没有为了它们单独重发应用。
- 297 个英语候选需要按功能区渐进审阅，不能用“清零队列”作为目标。
- `SOURCE_REV` 是上游源码快照标识；`ZH_VERSION` 记录个人发行策略。不要把 Git 同步提交号、上游源码快照号和个人 Release 版本混为一个字段。
