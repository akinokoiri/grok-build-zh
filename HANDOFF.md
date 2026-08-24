# Grok Build 汉化版（Windows 11 适配与自动化构建）项目交接文档 (HANDOFF)

## 1. 项目基本信息 (Project Overview)

* **项目名称**：`grok-build-zh` (Grok Build 社区汉化版 Windows 11 适配与云端自动化构建分发)
* **当前仓库地址**：[https://github.com/akinokoiri/grok-build-zh](https://github.com/akinokoiri/grok-build-zh)
* **默认分支**：`zh-CN`
* **最新 Release 版本**：`v0.1.0-zh.4`
* **代码上下游拓扑**：
  $$\text{xai-org/grok-build (官方英文原版)} \xrightarrow{\text{汉化维护}} \text{ivan6232/grok-build-zh (社区汉化)} \xrightarrow{\text{Windows适配与CI}} \text{akinokoiri/grok-build-zh (本仓库)}$$

---

## 2. 工作区定位与解决的问题 (Purpose & Problems Solved)

1. **解决原汉化版平台缺失问题**：
   社区上游 `ivan6232/grok-build-zh` 仅配置了 Linux 与 macOS 的构建流及 bash 脚本，Windows 用户无法直接获得预编译包，且在 Windows 上自行编译极其繁重（需要数 GB 的 VS C++ Build Tools、Rust 1.92+、Protoc 与 CMake）。
2. **打通 Windows 11 云端自动化编译 (GitHub Actions)**：
   在 GitHub Actions 运行器（`windows-latest`）上构建原生 Windows PE 二进制（`x86_64-pc-windows-msvc`），产出轻量级 `.zip` 压缩包并自动发布到 GitHub Releases。
3. **提供 Windows 11 PowerShell 一键秒级安装与升级**：
   编写了 `install.ps1` 脚本，任何 Windows 11 电脑无需配置任何编译环境，只需一行命令即可在 5 秒内完成全量中文版下载、PATH 环境变量注入与内置中文技能/人设释放。

---

## 3. 已完成的关键技术改造 (What Was Done)

### A. 修复底层 Protobuf 代码生成的 Windows 兼容性缺陷
* **文件**：[`crates/build/xai-proto-build/src/lib.rs`](file:///g:/grok-build-zh/crates/build/xai-proto-build/src/lib.rs)
  * **缺陷 1**：上游在 `emit_rerun_if_changed` 中硬编码了 Linux 设备路径 `--dependency_out=/dev/stdout` 和 `--descriptor_set_out=/dev/null`，导致 Windows 下 `protoc.exe` 执行报错 `/dev/stdout: No such file or directory`。
  * **修复 1**：重构为使用 `tempfile` 跨平台安全临时文件，并追加 `--experimental_allow_proto3_optional`。
  * **缺陷 2**：`protoc` 在 Windows 输出的 Makefile 依赖文件中，盘符会被转义（如 `D\:\...`），导致 `std::fs::exists(line)?` 抛出 Windows 系统错误 `os error 123 (The filename, directory name, or volume label syntax is incorrect)`。
  * **修复 2**：重构 Makefile 反转义逻辑（`.replace(r"\:", ":")`），将文件存在性检查改为安全容错模式，避免阻塞构建。
* **文件**：[`crates/build/xai-proto-build/src/find_protoc.rs`](file:///g:/grok-build-zh/crates/build/xai-proto-build/src/find_protoc.rs)
  * 完善了 Windows 下对 `PROTOC` 环境变量与 `protoc.exe` 的查找容错与路径探测。

### B. 重构 GitHub Actions Release 发布流
* **文件**：[`.github/workflows/release.yml`](file:///g:/grok-build-zh/.github/workflows/release.yml)
  * 在矩阵中添加 `windows-latest`（`x86_64-pc-windows-msvc`），打包生成 `grok-zh-x86_64-pc-windows-msvc.zip` 及 SHA256 校验文件。
  * 全平台统一使用 `arduino/setup-protoc@v3`（Protoc v29.3），消除 Linux 与 Windows 之间的编译器版本差异。
  * 移除了卡在排队队列的废弃 runner `macos-13`。

### C. 编写 Windows 专属一键安装/升级与资源部署脚本
* **文件**：[`install.ps1`](file:///g:/grok-build-zh/install.ps1)
  * 自动请求 GitHub API 匹配最新 Release 中的 Windows 资产包并校验 SHA256。
  * 将可执行文件部署至 `$HOME\.grok\bin\grok-zh.exe`，并自动把 `$HOME\.grok\bin` 写入用户 `PATH`。
  * 自动将全套中文人设、角色与内置技能描述（`bundled-zh`）部署到 `$HOME\.grok\bundled\`。
* **文件**：[`docs/zh/bundled-zh/apply.ps1`](file:///g:/grok-build-zh/docs/zh/bundled-zh/apply.ps1)
  * 提供独立的 PowerShell 中文技能与人设覆盖脚本。

### D. 文档更新
* **文件**：[`README.md`](file:///g:/grok-build-zh/README.md)
  * 补充了 Windows 11 PowerShell 一键安装命令与预编译下载表格。

---

## 4. 验证结果与端到端状态 (Verification Results)

1. **CI 状态**：GitHub Actions Run `#32632683901` 全平台编译成功（Windows x64、Linux x64、Linux ARM64、macOS Apple Silicon）。
2. **Release 状态**：[Release v0.1.0-zh.4](https://github.com/akinokoiri/grok-build-zh/releases/tag/v0.1.0-zh.4) 包含 46.87 MB 的 `grok-zh-x86_64-pc-windows-msvc.zip` 产物。
3. **本地实测**：通过 `install.ps1` 部署成功，`grok-zh --version` 正常输出 `grok 0.2.106 (18ca2f3) [stable]`。

---

## 5. 日常使用与维护指南 (Usage & Maintenance Guide)

### A. 在任何一台 Windows 11 上安装 / 升级
在 PowerShell 中运行：
```powershell
irm https://raw.githubusercontent.com/akinokoiri/grok-build-zh/zh-CN/install.ps1 | iex
```
启动中文版：
```powershell
grok-zh
```

### B. 上游更新同步流程 (Sync Upstream)
当社区汉化 `ivan6232/grok-build-zh` 或官方 `xai-org/grok-build` 更新时：
```bash
# 1. 拉取上游最新提交
git fetch upstream zh-CN
git checkout zh-CN
git merge upstream/zh-CN

# 2. 推送到自己的仓库
git push origin zh-CN

# 3. 打新 Tag 触发 Release 自动编译（如 v0.1.0-zh.5）
git tag v0.1.0-zh.5
git push origin v0.1.0-zh.5
```
CI 构建完成后，客户端再次运行 `install.ps1` 即可静默升级。

---

## 6. 后续接手 Agent 推荐技能 (Suggested Skills)

* `systematic-debugging`: 如遇 Rust 跨平台编译或 GitHub Actions CI 失败时，执行系统化根因排查。
* `managing-python-dependencies`: 如需调整 `apply_skill_descriptions.py` 技能描述批量替换脚本。
* `writing-great-skills`: 如需为 Grok Build 定制中文技能（`SKILL.md`）。
