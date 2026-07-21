# Grok Build 汉化版 · 使用教程

社区非官方。安装后命令为 **`grok-zh`**（与官方 `grok` 可并存）。

更细的安装见 [INSTALL.md](INSTALL.md)。术语与翻译约定见 [TRANSLATING.md](TRANSLATING.md)。

---

## 1. 安装后第一次使用

```bash
# 确保在 PATH 中
export PATH="$HOME/.local/bin:$PATH"

grok-zh --version
cd /path/to/your/project
grok-zh
```

- 未登录时按界面提示用浏览器登录（与官方相同，配置在 `~/.grok/`）。
- 底栏可见 **始终批准** 等中文权限标志；欢迎页为 **新建工作树 / 恢复会话 / 更新日志 / 退出**。

### 可选：内置人设描述中文包

列表里人设/角色的 **description** 可套用仓库提供的中文摘要（ID 仍为英文）：

```bash
# 在克隆的仓库根目录
bash docs/zh/bundled-zh/apply.sh
```

---

## 2. 日常操作

| 操作 | 怎么做 |
|------|--------|
| 启动 | 在项目目录执行 `grok-zh` |
| 带初始问题 | `grok-zh "帮我梳理仓库结构"` |
| 继续最近会话 | `grok-zh -c` |
| 恢复历史会话 | 欢迎页 **Ctrl+S**，或 `/resume` |
| 打开设置 | **F2** 或 `/settings` |
| 切换模型 | `/model` |
| 键盘快捷键 | **Ctrl+.** 或 **Ctrl+X** |
| 使用指南 | `/docs` |
| 扩展/技能/MCP | `/plugins`、`/skills`、`/mcps` 等 |
| 代理与人设 | `/config-agents` 或命令面板 |
| 退出 | **Ctrl+Q** 或欢迎页「退出」 |

### 权限模式（Shift+Tab 循环）

| 显示 | 含义 |
|------|------|
| 普通 | 敏感操作前询问 |
| 计划 | 先规划再执行 |
| 自动 | 分类器放行安全工具 |
| 始终批准 | 跳过权限提示（慎用） |

切换后会出现横幅：**已切换模式：始终批准**（完整中文，不会截成单字）。

### 权限弹窗

常见选项为中文：**是 / 否 / 始终允许… / 在所有会话中始终允许** 等。  
内部 canonical id（如 `always-approve`）保持英文，与配置兼容。

---

## 3. 界面汉化覆盖说明

| 模块 | 状态 |
|------|------|
| 设置分类 / 设置项名称 | ✅ 已完成 |
| 设置项说明 | ✅ 大部分（枚举显示：开/关/自动/滚轮等已中文） |
| 权限确认 | ✅ 核心路径 |
| 状态栏（思考中、回复中、压缩中、排队…） | ✅ |
| 回合事件（已工作、失败、压缩…） | ✅ |
| 快捷键面板与底栏 | ✅ 大部分（分类、footer、搜索） |
| 仪表盘 | ✅ 大部分（标题、空态、固定区） |
| 用户指南 `/docs` | ✅ 中文（部分长文精简） |
| 常见 Toast / 失败提示 | ✅ 已覆盖一批 |
| 扩展 / 市场 / 技能 / MCP **界面 chrome** | ✅ 底栏、统计、徽章、筛选 |
| 代理 / 人设 **界面 chrome** | ✅ 标签、底栏、说明 |
| 内置人设 **description** | ✅ 可用 `bundled-zh` 覆盖本机 |
| 市场插件 / 第三方技能 **远程描述** | ⚠️ 上游英文；ID 不改 |
| tips 长文案 | 🚧 核心已中文，可继续补 |
| 正规 en/zh 运行时切换 | 📋 规划中（当前为硬编码中文分支） |

### 不会改成中文的

- 插件 / 技能 / 人设 **ID**（如 `reviewer`、`superpowers`）
- 模型名（`Grok 4.5`）、命令名（`/model`）
- 会话标题（模型生成的内容）
- 远程 marketplace 插件长描述（除非作者提供中文或本地映射）

---

## 4. 配置与官方并存

| 命令 | 说明 |
|------|------|
| `grok` | 官方英文版（`curl -fsSL https://x.ai/cli/install.sh \| bash`） |
| `grok-zh` | 本汉化版 |

配置目录均为 `~/.grok/`（`config.toml`、`auth.json` 等）。**配置键名保持英文**。

```bash
# 示例：编辑配置
$EDITOR ~/.grok/config.toml
```

---

## 5. 常见问题

**Q: 对话已是中文，为何还要汉化？**  
A: 对话语言由模型决定；本项目汉化的是 **软件 UI**（菜单、权限、状态、弹窗）。

**Q: 技能/插件列表里描述仍是英文？**  
A: 界面按钮/统计已中文；**描述正文** 来自技能包或市场。内置人设可运行 `docs/zh/bundled-zh/apply.sh`；第三方请向作者要中文或本地改 SKILL.md。

**Q: `command not found: grok-zh`？**  
A: `export PATH="$HOME/.local/bin:$PATH"`，或重新执行 [install.sh](../../install.sh)。

**Q: 如何完全卸载？**  
A: `rm -f ~/.local/bin/grok-zh`，并删除 shell 配置中「grok-zh 汉化版」PATH 段。不必删 `~/.grok`（会同时影响官方版）。

---

## 6. 从源码编译（开发者）

```bash
git clone https://github.com/ivan6232/grok-build-zh.git
cd grok-build-zh && git checkout zh-CN
cargo install dotslash
export PROTOC="$(command -v protoc)"
cargo build -p xai-grok-pager-bin --release
cp target/release/xai-grok-pager ~/.local/bin/grok-zh
```

依赖与注意事项见 [INSTALL.md](INSTALL.md)。
