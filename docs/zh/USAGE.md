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

| 模块 | 状态 | 说明 |
|------|------|------|
| 欢迎页 / 首屏 | ✅ | 新建工作树、恢复会话、更新日志、退出 |
| 快捷键帮助 | ✅ | **Ctrl+.** / **Ctrl+X**；分类与底栏中文 |
| 模式横幅 | ✅ | Shift+Tab 后「已切换模式：…」完整显示 |
| 会话 / 目标恢复 | ✅ | 相对时间中文 |
| 扩展 / 技能 / MCP | ✅ | 界面 chrome + 筛选「已启用/已禁用」 |
| 设置面板 | ✅ | 枚举显示中文（开/关/自动、滚轮/触控板、全屏/极简…） |
| 仪表盘 / 任务 | ✅ | 「已固定 / 工作中 / 空闲 / 等待中」等分组；任务侧「子代理/任务/监视器」 |
| 人设 / 角色 / 代理 | ✅ 界面 | 标签、底栏、字段名中文；**description** 用 bundled-zh |
| Tips | ✅ | 剪贴板图片、规划提示、双击选词、SSH wrap、排队发送等 |
| `/docs` 用户指南 | ✅ | 应用内中文 |
| 市场远程描述 | ⚠️ | 上游英文；ID 不改 |
| 正规 i18n 切换 | 📋 | 规划中（当前硬编码中文） |

### 不会改成中文的

- 插件 / 技能 / 人设 **ID**（如 `reviewer`、`superpowers`）
- 模型名（`Grok 4.5`）、命令名（`/model`）、主题商品名
- 工具短名芯片（`Bash` / `Read` / `Edit` 等，与工具 ID 一致）
- 会话标题（模型生成内容）
- 远程 marketplace 插件长描述（除非作者提供中文或本地映射）

---

## 3.1 分模块操作要点

### 设置（F2 或 `/settings`）

- 底栏：**Enter 切换 / Enter 编辑 / Space 切换 / / 搜索 / d 重置**
- 常见枚举显示为中文：**开 / 关 / 自动 / 鼠标滚轮 / 触控板 / 全屏 / 极简 / 按住说话**
- 配置键名仍为英文（写在 `~/.grok/config.toml` 中）

### 仪表盘（`/dashboard`）

- 标题 **仪表盘**；无会话时提示输入启动
- 分组：**已固定 / 等待中 / 工作中 / 空闲 / 未激活 / 完成 / 失败 / 已阻塞**
- 任务面板分组：**子代理 / 任务 / 监视器**

### 扩展 / 技能 / MCP

- 打开：`/plugins`、`/skills`、`/mcps` 等
- 筛选可在 **全部 / 已启用 / 已禁用** 间循环
- 列表里的 **ID 英文**、**远程描述可能英文** 属正常

### 人设描述中文（可选）

```bash
# 克隆本仓库后
bash docs/zh/bundled-zh/apply.sh
```

会覆盖 `~/.grok/bundled/` 下 personas / roles / agents 的 description（ID 不变）。  
客户端更新后若 bundled 被还原，可再执行一次。

### Tips 何时出现

| 场景 | 提示含义 |
|------|----------|
| 剪贴板有图片 | `ctrl+v` 粘贴 |
| 输入含 plan/design 等 | 可 Shift+Tab 进计划模式 |
| 双击选中文字 | `/settings` → 文本选择；`Ctrl+Y` 立即启用 |
| SSH 会话 | 本地 `grok wrap ssh <host>` |
| 小屏 | `/compact-mode` |
| 回合中排队 | Enter 立即发送队首 |

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
