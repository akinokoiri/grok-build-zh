# 当前交接

更新时间：2026-09-12（Asia/Shanghai）

## 当前结论

项目仅维护 Windows 11 x64 个人汉化发行版。本轮同步官方 `37949780`（Grok Build 1.0.24，9 月 8 日与 9 日两次同步、共 80 项列出的上游更新），`SOURCE_REV` 为 `c4ea71cfdbcdb21e32e41bc25a0043d7d4836714`。保留官方新的模式标记逻辑，官方后台更新仍硬关闭，显式更新仅使用本仓库 Release。

本轮修复 `/context` 漏译与中文显示列对齐，语言包从 414 条增至 448 条。修复 Windows PTY harness 的 Unix-only 导入及终端字形相关测试。每周同步的合并冲突改为摘要与 notice，真实错误继续失败；发布门禁逐条检查原生命令退出码，拒绝覆盖旧版本。安装器验证退出码及完整版本后才更新语言包、删除备份，失败恢复原程序；脚本保留 UTF-8 BOM，兼容现有 Windows 更新入口的解码方式。

`zh-CN` 是远端默认维护分支。[PR #1](https://github.com/akinokoiri/grok-build-zh/pull/1) 已通过 merge 合并为 `0723a12e`，保留官方上游祖先关系。正式版本从已验证提交 `f089b5ad` 构建。`legacy/full-fork-2026-08-29` 保留旧完整分支，不要删除。

## 已发布与本机状态

- Release：`v1.0.24-zh.1`（2026-09-12 13:46，北京时间；非 prerelease，已设为 latest）
- 发布页：<https://github.com/akinokoiri/grok-build-zh/releases/tag/v1.0.24-zh.1>
- 资产：`grok-zh-x86_64-pc-windows-msvc.zip` 及其 `.sha256`
- ZIP SHA-256：`a129ab6759264181523f6ae67a3a4cf433b4422e30ba3276f20c4c78a9b6686d`
- 本机命令：`C:\Users\akino\.grok\bin\grok-zh.exe`
- 本机报告版本：`grok 1.0.24-zh.1 (f089b5adbda5) [alpha]`
- 本机可执行文件 SHA-256：`0CD2CF41A4FE2D4D51B5D8E4EE5F07B0D31BC5AB52EC136D3A2B552E9951EE28`
- 本机语言包：`C:\Users\akino\.grok\i18n\zh-CN.json`（448 条翻译，与发布源码哈希一致）
- 本机更新脚本：`C:\Users\akino\.grok\bin\install-grok-zh.ps1`
- 本机已使用正式 Release 安装器完成更新，包校验与版本冒烟通过。安装前确认旧语言包没有用户自定义条目。`grok-zh update --check --json` 返回当前及最新版本均为 `1.0.24-zh.1`、`updateAvailable=false`，来源为 `github:akinokoiri/grok-build-zh`。

## 本次已发布修复

2026-09-12 根据实际截图补齐 `/context` 上下文面板、三个共享页签及底部快捷键的汉化，新增 34 条集中语言包条目。弹窗和 minimal 模式的历史输出共用该渲染实现；中文标签按终端显示列宽对齐。只在显示层翻译已知内置分类和计数词，模型名、`AGENTS.md`、`tokens` 单位及未知数据保留原样。

该修复已随 `v1.0.24-zh.1` 发布并安装到本机。面板翻译需要新版二进制和语言包配合，不能仅替换旧版的外部语言包。

验证：翻译审计、格式检查、4 项共享语言包测试、2 项 protobuf 回归测试及 Windows pager-bin 编译检查均通过。PTY harness 编译问题已修复，40 项 context_info 测试在传统及现代终端两种字形模式下均通过，32 项 usage_modal 测试通过；安装器 5 种成功/失败场景通过。最终提交的 [Windows 校验](https://github.com/akinokoiri/grok-build-zh/actions/runs/34674243170) 和 [正式发布门禁](https://github.com/akinokoiri/grok-build-zh/actions/runs/34674248647) 均成功，包括 Release profile 测试、版本冒烟和资产上传。

## 汉化与门禁快照

集中式语言包源码、最新 Release 及本机均有 448 条：

- 25 个内置斜杠命令说明已覆盖。
- 72 个内置动作/快捷键说明已覆盖。
- 欢迎页菜单、会话与看板模式标记（始终批准/自动/计划/批注）、Grok 4.6 公告、10 条当前官方轮换提示、推理强度和发布通道已覆盖；未知的未来远端文本保持英文回退，不做运行时机器翻译。
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

- 301 个英语候选需要按功能区渐进审阅，不能用“清零队列”作为目标。
- 未优化的本地 debug 可执行文件在此机器执行 `version` 时曾栈溢出，Release profile 正常；正式发布始终以 Release 工作流产物为准。
- `SOURCE_REV` 是上游源码快照标识；`ZH_VERSION` 记录个人发行策略。不要把 Git 同步提交号、上游源码快照号和个人 Release 版本混为一个字段。
