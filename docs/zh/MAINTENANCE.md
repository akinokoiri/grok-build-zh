# 个人版维护流程

这份流程刻意限制维护面，优先级是：运行稳定性 > 汉化覆盖率 ≥ 安装便利 > 更新速度 > 更多设备。

## 每周同步

`Propose weekly upstream sync` 工作流每周获取 `xai-org/grok-build:main`，合并到 `automation/upstream-sync` 并创建候选 PR。它不自动合并、不自动发布。若上游产生冲突，工作流直接失败，由一次 LLM 维护任务在本地处理；不要为了保持历史补丁形状而长期保留散落的硬编码翻译。

同步时按以下顺序处理：

1. 读取上游变更和编译错误，先恢复官方逻辑的可编译状态。
2. 运行 `scripts/audit-translations.ps1`，检查新增内置命令、技能、人设和工作流。
3. 让 LLM 审阅审计脚本报告的英语候选，只翻译明确面向用户且属于维护边界的文本。
4. 运行 Windows x64 门禁。第三方技能、插件、协议字段、测试夹具和日志不纳入翻译目标。
5. 合并候选 PR；只有累计到重要功能或修复时才发布。

## 语言包

`xai-grok-shared::i18n` 嵌入 `i18n/zh-CN.json`，并允许用户目录中的同名文件覆盖。外部文件只接受 `version = 1`、`locale = zh-CN`、非空字符串键值和不超过 1 MiB 的内容；任何错误都静默回退到随程序编译的语言包。

增加翻译时先改 JSON，再扩展审计脚本的必需 ID。不要把翻译重新散落到上游 Rust 常量中，也不要翻译第三方内容。

## 发布

在 Actions 中手动运行 `Release Windows x64`，版本必须类似 `0.2.106-zh.1`。工作流按顺序执行翻译审计、格式检查、语言包测试、Windows protobuf 回归测试、Release 编译和版本冒烟测试；任一步失败都不会创建 Release。

成功后只发布：

- `grok-zh-x86_64-pc-windows-msvc.zip`
- 对应 `.sha256`

Release 必须保持为非 prerelease，这样 GitHub 的 `/releases/latest` API 能被安装器使用。尽管版本字符串带 `-zh.N`，这只是个人版版本约定。

## 回滚与边界

- `legacy/full-fork-2026-08-29` 保存重构前的完整汉化分支。
- 安装器替换前备份现有 `grok-zh.exe`，新程序无法运行时恢复备份。
- 官方后台更新在源码中硬关闭；`grok-zh update` 只启动本仓库安装器。
- 不维护 ARM64、Windows 10、PowerShell 5.1、macOS、Linux、源码安装或与官方 `grok` 并存的专门逻辑。
