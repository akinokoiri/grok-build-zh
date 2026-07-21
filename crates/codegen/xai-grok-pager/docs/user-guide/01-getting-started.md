# 入门指南

Grok Build 是 SpaceXAI 的终端 AI 编程助手。它以 TUI（终端用户界面）运行，能理解代码库、执行 shell 命令、编辑文件、搜索网页并管理任务。

可用方式：全屏交互 TUI、无头模式（脚本/CI）、或通过 Agent Client Protocol（ACP）接入编辑器。

> 社区汉化版可使用 `grok-zh` 命令（见 [grok-build-zh](https://github.com/ivan6232/grok-build-zh)）。官方安装脚本默认命令为 `grok`。

---

## 安装

macOS / Linux / Windows Git Bash：

```bash
curl -fsSL https://x.ai/cli/install.sh | bash
```

指定版本：

```bash
curl -fsSL https://x.ai/cli/install.sh | bash -s 0.1.42
```

**Windows PowerShell：**

```powershell
irm https://x.ai/cli/install.ps1 | iex
```

安装后验证：

```bash
grok --version
# 或汉化版：
grok-zh --version
```

更新：

```bash
grok update
```

---

## 首次启动

```bash
grok
```

首次启动会打开浏览器完成 grok.com 登录。凭证保存在 `~/.grok/auth.json`，会自动刷新；无法续期时再提示登录。

无浏览器或 CI 环境可用 API Key：

```bash
export XAI_API_KEY="xai-..."
grok
```

完整认证方式见 [身份认证](02-authentication.md)。

---

## 基本交互

认证后 TUI 主要区域：

- **回滚区（Scrollback）**：对话历史、工具调用、编辑 diff 等  
- **提示区（Prompt）**：底部输入框  

输入消息后按 `Enter` 发送。代理会按需读文件、跑命令、改代码；工具输出实时流入回滚区。

常用：

- `/`：斜杠命令自动完成（见 [斜杠命令](04-slash-commands.md)）  
- `@`：引用仓库中的文件  
- `Esc`：取消当前操作 / 关闭菜单  
- `/settings`：打开设置（汉化版中文界面）  
- `/model`：切换模型  

---

## 关键概念

| 概念 | 含义 |
|------|------|
| 会话 Session | 一次对话上下文，可保存与恢复 |
| 工具 Tool | 读文件、shell、搜索、MCP 等 |
| 权限 Permission | 危险操作前是否询问你 |
| 计划模式 Plan | 先写方案再改代码 |
| Skills / 插件 | 可复用的能力包 |

---

## 下一步

1. [键盘快捷键](03-keyboard-shortcuts.md)  
2. [配置](05-configuration.md)  
3. [权限与安全](22-permissions-and-safety.md)  
