# 当前交接

更新时间：2026-08-29（Asia/Shanghai）

## 当前结论

项目已经收敛为 Windows 11 x64 个人汉化发行版。欢迎页汉化修订版 `v1.0.12-zh.2` 已发布，本机也已安装并从 `G:\SillyTavern-RP\run.cmd` 通过中文界面冒烟测试。官方后台更新已关闭；后续更新只能显式读取本仓库 Release。

当前功能实现基线截至 `bce3f556`；本交接文档和发布顺序优化位于其后的维护提交。其中：

- `bec8523b`：重建集中式汉化、Windows x64 发布、安装/更新和门禁；`v1.0.12-zh.1` 从该提交构建。
- `e059a91e`：把发布门禁改为 Release profile；后续实测发现 `release-dist` 特性图仍使最终构建重复编译。
- `26d7405e`：Windows 工作流启用 `sccache`。
- `c299266d`：固定维护中的 GitHub Actions 版本。
- `bce3f556`：接入欢迎页菜单、公告、官方轮换提示、推理强度、批准模式和版本通道汉化；`v1.0.12-zh.2` 从该提交构建。

`zh-CN` 是远端默认维护分支；`codex/windows-x64-i18n-core` 是本轮实现分支。`legacy/full-fork-2026-08-29` 保留旧完整汉化分支，不要删除。

## 已发布与本机状态

- Release：`v1.0.12-zh.2`
- 发布页：<https://github.com/akinokoiri/grok-build-zh/releases/tag/v1.0.12-zh.2>
- 资产：`grok-zh-x86_64-pc-windows-msvc.zip` 及其 `.sha256`
- 本机命令：`C:\Users\akino\.grok\bin\grok-zh.exe`
- 本机报告版本：`grok 1.0.12-zh.2 (bce3f55664e6) [stable]`
- 本机语言包：`C:\Users\akino\.grok\i18n\zh-CN.json`
- 本机更新脚本：`C:\Users\akino\.grok\bin\install-grok-zh.ps1`

发布产物已经由安装器校验 SHA-256；本机可执行文件的当前 SHA-256 为 `C43ED804839C4FBFF25057C9BD643A0826FDF7FFFC21776E6C2B9A32FE7D7C96`。为替换正在运行的旧二进制，安装时只终止了 PID 24564；未删除会话或用户配置。

## 汉化与门禁快照

集中式语言包当前有 409 条：

- 25 个内置斜杠命令说明已覆盖。
- 72 个内置动作/快捷键说明已覆盖。
- 欢迎页菜单、Grok 4.6 公告、10 条当前官方轮换提示、推理强度、批准模式和发布通道已覆盖；未知的未来远端文本保持英文回退，不做运行时机器翻译。
- 审计脚本仍报告 297 个高置信英语候选。这是供 LLM 逐批审阅的队列，不表示应当机械地全部翻译；第三方、协议、测试和诊断文本必须继续排除。

本地翻译审计当前通过。核心入口：

- `crates/codegen/xai-grok-shared/src/i18n.rs`
- `crates/codegen/xai-grok-shared/i18n/zh-CN.json`
- `crates/codegen/xai-grok-shared/i18n/schema.json`
- `scripts/audit-translations.ps1`

## CI 状态与耗时基线

Windows 校验和发布使用 `sccache 0.17.0`，Cargo 注册表由 `Swatinem/rust-cache` 缓存，但不缓存 `target/`。`CARGO_INCREMENTAL=0` 是为了保证编译任务可被 sccache 命中。

2026-08-29 普通校验对照：

- 冷运行：19 分 56 秒。
- 缓存重跑：11 分 55 秒。
- 1637 个可缓存 Rust 任务中命中 1440 个，命中率 87.97%。
- 固定 Action 版本后的最终验证约 12 分 42 秒，命中率 89.37%，无 sccache 错误。

`v1.0.12-zh.2` 是启用独立 Release 参数后的首轮冷发布，实测总耗时 57 分 02 秒：旧 `Release gates` 顺序耗时 20 分 22 秒，最终 `release-dist` 构建又耗时 33 分 58 秒；sccache 为 0/1639 命中。门禁与最终构建实际上用了不同特性图，因此没有实现注释声称的依赖复用。发布后已把工作流顺序调整为“静态门禁 -> 最终 `release-dist` 构建 -> Release 测试 -> 冒烟/发布”，让窄测试复用宽构建；下一次发布需记录暖缓存实测，不能继续沿用旧的 12 分钟估计。

不要把首次冷编译视作稳定耗时；上游大改、版本/特性参数变化会自然降低命中率。

## 本机维护环境

- Rustup 1.29.0，Rust/Cargo 1.94.0（MSVC x64，minimal profile）。
- Protobuf Compiler 29.3（Winget `Google.Protobuf`）。
- GitHub CLI 2.98.0（Winget `GitHub.cli`）；未写入独立 gh 登录，自动化通过现有 Git Credential Manager 凭据临时提供 `GH_TOKEN`。

## 下一次任务建议顺序

1. 获取 `official/main`，先确认是否有值得同步的重要变更。
2. 使用每周同步候选 PR，不自动合并、不自动发布。
3. 解决上游逻辑/编译冲突后运行翻译审计，让 LLM 只审阅新增且在维护边界内的用户可见文本。
4. 运行 Windows x64 门禁；积累到重要功能或修复后再以 `<上游版本>-zh.<修订号>` 发布。
5. 发布后更新本文件中的维护提交、Release、本机安装和 CI 基线。

## 已知但非阻塞事项

- Release 二进制来自 `bce3f556`；其后的发布顺序和交接文档调整不改变应用代码，因此不为它们单独重发应用。
- 297 个英语候选需要按功能区渐进审阅，不能用“清零队列”作为目标。
- Windows 上 `cargo test -p xai-grok-pager welcome --lib` 目前会先被既有 PTY harness 的 Unix-only `process_has_exited_without_reap` 导入错误挡住；本次改动已由共享语言包测试、pager 编译、Release profile 构建和真实 TUI 冒烟覆盖。
- 未优化的本地 debug 可执行文件在此机器执行 `version` 时曾栈溢出，Release profile 正常；正式发布始终以 Release 工作流产物为准。
- `SOURCE_REV` 是上游源码快照标识；`ZH_VERSION` 记录个人发行策略。不要把 Git 同步提交号、上游源码快照号和个人 Release 版本混为一个字段。
