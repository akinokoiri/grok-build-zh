# 当前交接

更新时间：2026-09-27（Asia/Shanghai）

## 当前结论

项目仅维护 Windows 11 x64 个人汉化发行版。上周发布的 `v1.0.35-zh.1`（构建提交 `58e02c2e`）已在本机前台执行安装脚本实测完成替换与版本校验。

官方上游本周已跃迁至 1.0.41（`f0e3be11`，`SOURCE_REV` 为 `036a5d8348cd744767cd0b08518ab17bf608fa7f`）。本周同步 [PR #3](https://github.com/akinokoiri/grok-build-zh/pull/3) 已由分支 `sync/official-1.0.41` 合并进 `zh-CN`（合并提交 `d93d183b`）：
- CI 校验工作流（run `36315448800`）耗时 11 分 41 秒全绿通过。
- 解决了 3 处合并冲突（`settings_modal/render.rs` 适配多行换行与生命周期、`Cargo.toml` 采纳官方 `test-support`、`slash_commands.rs` 统一接入 `available_command` 汉化）。
- 集中语言包新增 `dashboard_preview`（看板预览）与 `subagent_model_inheritance`（子代理模型继承）条目，总词条数由 449 增至 453 条，翻译审计通过。
- 更新 `find-msvc-tools` 至 0.1.14，修复 Windows MSVC 环境下 `cc` 编译类型不匹配。
- 官方后台更新仍硬关闭，显式更新仅使用本仓库 Release。PR 合并不触发自动发布，待手动运行 `Release Windows x64` 发布 `v1.0.41-zh.1`。

`zh-CN` 仍是远端默认维护分支。`legacy/full-fork-2026-08-29` 保留旧完整分支，不要删除。

## 已发布与本机状态

- Release：`v1.0.35-zh.1`（2026-09-19 19:52，北京时间；非 prerelease，已设为 latest）
- 发布页：<https://github.com/akinokoiri/grok-build-zh/releases/tag/v1.0.35-zh.1>
- 构建提交：`58e02c2e`
- 资产：`grok-zh-x86_64-pc-windows-msvc.zip` 及其 `.sha256`
- ZIP SHA-256：`0ec23aa04fdaed283e8155fda2081d6d5cde8bb3e812b7a56832dca70ed5cba6`
- 本机命令：`C:\Users\akino\.grok\bin\grok-zh.exe`
- 本机报告版本：`grok 1.0.35-zh.1 (58e02c2ea2d6) [alpha]`（已完成替换）
- 本机可执行文件 SHA-256：`9504AAFF7E20E77E65373AEA5D8265A034509CEDB3142D9E8746C74ADB10E4C5`
- 本机语言包：`C:\Users\akino\.grok\i18n\zh-CN.json`（449 条，已含 `mode.ask`，无用户自定义条目）
- 本机更新脚本：`C:\Users\akino\.grok\bin\install-grok-zh.ps1`
- `grok-zh update --check --json`：`currentVersion=1.0.35-zh.1`，`latestVersion=1.0.35-zh.1`，`updateAvailable=false`，来源 `github:akinokoiri/grok-build-zh`。

## 本次已发布修复

2026-09-19 同步官方 1.0.24 → 1.0.35。用户可见变化包括 Windows ProjFS 启动崩溃修复、`grok clone` 反斜杠问题、`/memory` 弹窗、会话头、MCP 展示、`/btw` 句中提问等。个人版保留集中语言包，新增询问模式标记。

验证：[PR Windows 校验](https://github.com/akinokoiri/grok-build-zh/actions/runs/35438354593) 和 [正式发布](https://github.com/akinokoiri/grok-build-zh/actions/runs/35439173351) 均成功，包括静态门禁、Release 编译、语言包测试、protobuf 回归、版本冒烟和资产上传。本机因 `grok-zh.exe` 正在运行未能替换，需退出后执行 `grok-zh update`。

## 汉化与门禁快照

集中式语言包源码和 Release 包内有 449 条；本机外部语言包仍为 448 条，待安装后替换：

- 25 个内置斜杠命令说明已覆盖（含 `/memory`、`/flush`、`/dream`）。
- 72 个内置动作/快捷键说明已覆盖。
- 欢迎页菜单、会话与看板模式标记（始终批准/自动/计划/询问/批注）、Grok 4.6 公告、10 条当前官方轮换提示、推理强度和发布通道已覆盖；未知的未来远端文本保持英文回退，不做运行时机器翻译。
- 审计脚本报告 301 个高置信英语候选。这是供 LLM 逐批审阅的队列，不表示应当机械地全部翻译；第三方、协议、测试和诊断文本必须继续排除。

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

2026-09-12 本轮最终 Windows 校验耗时 10 分 32 秒；`v1.0.24-zh.1` 发布任务约 51 分 20 秒，其中正式编译 44 分 57 秒，Release 测试 4 分 01 秒。合并后的同内容校验无需由主代理高频轮询；需要跨会话监控时必须确认持久调度和唤醒路径，不能仅凭临时子代理承诺持续监控。

2026-09-19 [PR #2 Windows 校验](https://github.com/akinokoiri/grok-build-zh/actions/runs/35438354593) 成功，check 作业 15 分 56 秒；其中 `cargo check -p xai-grok-pager-bin` 约 11 分 02 秒。[翻译审计](https://github.com/akinokoiri/grok-build-zh/actions/runs/35438354529) 18 秒通过。这次上游跨 1.0.24 到 1.0.35，编译比上次 10 分 32 秒的同内容校验更久。

2026-09-19 `v1.0.35-zh.1` 发布任务约 48 分 55 秒：正式编译 41 分 16 秒，Release 测试 3 分 58 秒。比 `v1.0.24-zh.1` 的 51 分 20 秒略短，正式编译仍远长于校验用的 `cargo check`。

## 本机维护环境

- Rustup 1.29.0，Rust/Cargo 1.94.0（MSVC x64，minimal profile）。
- Protobuf Compiler 29.3（Winget `Google.Protobuf`）。
- GitHub CLI 2.98.0（Winget `GitHub.cli`）；未写入独立 gh 登录，自动化通过现有 Git Credential Manager 凭据临时提供 `GH_TOKEN`。

## 下一次任务建议顺序

1. （已完成）[PR #3](https://github.com/akinokoiri/grok-build-zh/pull/3) CI 全绿通过并已成功合并进 `zh-CN`（`d93d183b`）。
2. 在 GitHub Actions 页面手动触发 `Release Windows x64` 工作流（输入版本号 `1.0.41-zh.1`）构建正式 Release 包。
3. 发布完成后在退出现有 grok-zh 会话的情况下执行 `grok-zh update`，实测核对本机 1.0.41-zh.1 的 SHA-256 和语言包。

## 已知但非阻塞事项

- 301 个英语候选需要按功能区渐进审阅，不能用“清零队列”作为目标。
- 未优化的本地 debug 可执行文件在此机器执行 `version` 时曾栈溢出，Release profile 正常；正式发布始终以 Release 工作流产物为准。
- `SOURCE_REV` 是上游源码快照标识；`ZH_VERSION` 记录个人发行策略。不要把 Git 同步提交号、上游源码快照号和个人 Release 版本混为一个字段。
