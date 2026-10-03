# 当前交接

更新时间：2026-10-04（Asia/Shanghai）

## 当前结论

Windows 11 x64 个人汉化版已从 `1.0.41-zh.1` 同步、发布并在本机安装为 `1.0.45-zh.1`。安装器完成 SHA-256 校验、程序替换及版本冒烟；本机显式更新检查确认已是最新版，来源仍为 `github:akinokoiri/grok-build-zh`。

- 同步 [PR #4](https://github.com/akinokoiri/grok-build-zh/pull/4) 已合并，维护分支和正式构建提交为 `39f8c78f1607726f283f59b7a377cdf8f75c43b9`。
- 官方公开源码：`2bdd1d6a6369de0e8c68132ea4539e9abd9e14a8`，版本 `1.0.45`。
- `SOURCE_REV`：`559751fdcec02d413e4c57c8832ab275e4f44980`，是官方内部源码快照标识，不是个人版构建提交。
- 本轮查询时官方发布说明已到 `1.0.46`，公开源码尚未跟进；本次发行不包含该版本的新增修复。

`zh-CN` 是 GitHub 默认维护分支；`official/main` 跟踪官方源码。`legacy/full-fork-2026-08-29` 是旧完整分支保留点，不要删除。

## 本次变更

- 纳入 Windows 剪贴板并发崩溃、非剪贴板粘贴误附旧图片、同文件并发编辑等修复。
- 纳入 `/context-window`、模型选择时的窗口选项、自定义子代理选择、MCP token 文件按请求重读和子代理等待状态改进。
- 解决每周同步任务发现的 7 处冲突：采用官方 cloud-config 依赖、设置分类渲染及实际服务模型显示逻辑，保留集中汉化和个人更新边界。
- 新增上下文窗口命令、窗口选项和选择器标题汉化；模式标记适配 Auto-review；推理强度汉化移入上游新的模型标签生成函数。
- `ZH_VERSION` 的官方提交记录已校正为 `2bdd1d6a`。官方后台更新仍硬关闭，显式更新仍只读本仓库 Release。

## 已发布与本机状态

- Release：[v1.0.45-zh.1](https://github.com/akinokoiri/grok-build-zh/releases/tag/v1.0.45-zh.1)，2026-10-04 00:53（北京时间）；非 prerelease，已设为 latest。
- 资产：`grok-zh-x86_64-pc-windows-msvc.zip` 及其 `.sha256`，均由正式发布工作流生成。
- ZIP SHA-256：`db481eba7eb391b428d31949d9bf3dfe7923fc921f9d81766440135128080c1b`。
- 本机命令：`C:\Users\akino\.grok\bin\grok-zh.exe`。
- 实测版本：`grok 1.0.45-zh.1 (39f8c78f1607) [alpha]`。
- 本机程序 SHA-256：`9CCC8E7B815BE6A870584796E8151FBDB9E827630F3F39BE9625638AA27B4079`。
- 本机语言包：`C:\Users\akino\.grok\i18n\zh-CN.json`，461 条，与当前源码文件哈希一致；安装前已确认旧文件无自定义词条。
- 本机安装器：`C:\Users\akino\.grok\bin\install-grok-zh.ps1`。
- `grok-zh update --check --json` 实测：`currentVersion=1.0.45-zh.1`，`latestVersion=1.0.45-zh.1`，`updateAvailable=false`，来源 `github:akinokoiri/grok-build-zh`。
- 用户已退出旧程序，安装脚本在前台实测完成；上述验证不代表已人工遍历全部 TUI 交互。

## 验证与编译基线

- [PR Windows 校验](https://github.com/akinokoiri/grok-build-zh/actions/runs/37135157259)：固定候选提交 `b42c1423324651f1784a4affa491644dad201f5c`，全部通过，作业 11 分 50 秒；应用编译检查 8 分 16 秒。
- 翻译审计、72 个内置动作覆盖、安装器成功/回滚/版本校验用例、格式、语言包加载测试和 Windows protobuf 回归测试通过。
- [正式发布](https://github.com/akinokoiri/grok-build-zh/actions/runs/37135985366)：固定合并提交 `39f8c78f`；正式优化编译 36 分 54 秒，Release 测试 2 分 10 秒，版本冒烟、打包及发布均通过。
- 相对 `official/main` 的个人补丁 `git diff --check` 通过。上游导入的 changelog 和测试夹具有原生空白告警，保留原样，不清理语义可能依赖空白的夹具。

工作流使用 `sccache 0.17.0` 缓存编译单元，Cargo 缓存不保存 `target/`，`CARGO_INCREMENTAL=0`。发布顺序是静态门禁 → 最终 release-dist 宽编译 → Release 窄测试 → 冒烟/发布。不要把约 12 分钟的 `cargo check` 当作正式发布耗时。

对照：2026-09-27 的 `v1.0.41-zh.1` 发布约 46 分 48 秒，正式编译 38 分 24 秒；更早跨版本发布约 49–51 分钟。不同上游改动和特性图会影响缓存命中。

## 后续与已知边界

1. 日常使用已可直接运行新版 `grok-zh`；下次同步先核对公开源码是否已包含 1.0.46 或更新版本。
2. 当前语言包 461 条；审计覆盖 26 个可发现的内置命令描述和 72 个内置动作。审计仍报告 305 个高置信英语候选，这是渐进审阅队列，不要求清零。
3. 新的子代理等待状态及部分 Auto-review 设置枚举、提示仍保留英文，列入后续汉化；第三方内容、协议字段、测试夹具和诊断日志不翻译。
4. 未优化的本地 debug 程序曾在本机执行 `version` 时栈溢出；正式 Release profile 冒烟正常，发行始终以工作流产物为准。

维护流程见 [MAINTENANCE.md](MAINTENANCE.md)，工程边界和最小验证矩阵见 [AGENTS.md](../../AGENTS.md)。本机维护环境仍为 Rust/Cargo 1.94.0、Rustup 1.29.0、Protobuf 29.3、GitHub CLI 2.98.0；自动化临时使用现有 Git Credential Manager 凭据，不写入独立 gh 登录。
