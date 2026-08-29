# Grok Build 中文个人版：Agent 协作约定

本仓库是 `xai-org/grok-build` 的 Windows 11 x64 个人汉化发行版，不是通用多平台分叉。开始任务前先读 `docs/zh/HANDOFF.md` 和 `docs/zh/MAINTENANCE.md`。

## 不可偏离的维护边界

- 优先级：运行稳定性 > 汉化覆盖率 >= 安装便利 > 更新速度 > 更多设备。
- 只构建和发布 `x86_64-pc-windows-msvc`；不恢复 ARM64、Windows 10、macOS 或 Linux 发行任务。
- 官方后台更新永久关闭。显式 `grok-zh update` 只能读取 `akinokoiri/grok-build-zh` 的 GitHub Release。
- 翻译范围仅含 TUI、内置命令、内置技能、人设和工作流。第三方插件、第三方技能、用户内容、协议字段、测试夹具和诊断日志保持原文。
- 不引入运行时机器翻译。上游新增英文由 LLM 辅助审阅后写入集中语言包。
- 本项目公开是为了保留来源、构建记录和许可证，不承诺社区支持或多环境兼容。

## 修改规则

- 翻译唯一来源是 `crates/codegen/xai-grok-shared/i18n/zh-CN.json`；不要在 Rust 常量里新增散落的中文硬编码。
- 新增必需翻译 ID 时同步更新 `scripts/audit-translations.ps1` 和必要测试。
- 保留语言包外部覆盖的严格校验和失败回退，不让损坏的用户文件影响启动。
- 上游同步以恢复官方逻辑和可编译状态为先，再处理汉化，不为保留旧补丁形状钻牛角尖。
- 工作流 Action 尽量固定到已核验的提交；Windows 编译继续由 `sccache` 负责产物缓存，Cargo 缓存不保存 `target/`。
- 不提交 `target/`、本地日志、下载缓存、会话、凭据或构建产物。Release 资产只能由发布工作流生成。

## 最小验证矩阵

纯文档改动至少检查链接和 `git diff --check`。翻译改动运行：

```powershell
./scripts/audit-translations.ps1
cargo fmt --all -- --check
cargo test -p xai-grok-shared i18n --lib
```

Rust、构建或更新逻辑改动还应运行：

```powershell
cargo test -p xai-proto-build dependency_tests --lib
cargo check -p xai-grok-pager-bin
```

发布前使用 `Release Windows x64` 工作流完成 Release profile 门禁和版本冒烟测试，不用本地二进制代替正式发布产物。

## 分支与交接

- `zh-CN` 是 GitHub 默认维护分支。
- `official/main` 跟踪官方上游；`upstream/zh-CN` 是早期参考汉化项目，只作比对来源。
- `legacy/full-fork-2026-08-29` 是重构前完整分支的保留点，不在日常任务中修改或删除。
- 每次完成发布、上游同步、工作流策略调整或重要本机环境变化后，更新 `docs/zh/HANDOFF.md`；不要把流水账和大段命令输出写进去。

## 本地清理边界

- 可删除：仓库 `target/`、已经结束的 `grok-zh-install-*` 临时目录，以及确认无进程使用的 `.grok/downloads` 下载缓存、`.grok/logs` 和 `.grok/memtrace` 诊断输出。
- 不可自动删除：`.grok/sessions`、认证/配置、技能、人设、工作流、市场缓存、用户语言包和 `legacy/` Git 引用。
- 递归清理前必须解析并核对绝对路径；只清理明确目标，不对工作区根目录或用户目录执行宽泛删除。
