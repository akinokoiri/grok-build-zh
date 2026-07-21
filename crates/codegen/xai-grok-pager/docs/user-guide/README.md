# Grok Build 用户指南（简体中文）

> **汉化说明**：本文档为社区汉化版，基于 [xai-org/grok-build](https://github.com/xai-org/grok-build) 上游文档翻译。  
> 非官方文档；术语以源码与配置键名为准（配置键保持英文）。

了解如何安装、配置与扩展 Grok Build（终端 AI 编程助手）。

---

## 第一层：入门必读

| # | 文档 | 说明 |
|---|------|------|
| 1 | [入门指南](01-getting-started.md) | 安装、首次启动、认证、基本交互 |
| 2 | [身份认证](02-authentication.md) | 浏览器登录、API Key、OIDC/SSO |
| 3 | [键盘快捷键](03-keyboard-shortcuts.md) | TUI 全部快捷键与鼠标操作 |
| 4 | [斜杠命令](04-slash-commands.md) | 会话、模型、记忆、钩子等 `/` 命令 |
| 5 | [配置](05-configuration.md) | `config.toml`、`pager.toml`、环境变量 |

## 第二层：核心功能

| # | 文档 | 说明 |
|---|------|------|
| 6 | [主题与外观](06-theming.md) | 主题、`/theme`、`pager.toml` |
| 7 | [MCP 服务器](07-mcp-servers.md) | 外部工具集成 |
| 8 | [技能 Skills](08-skills.md) | SKILL.md 可复用提示包 |
| 9 | [插件与市场](09-plugins.md) | 插件安装与打包 |
| 10 | [钩子 Hooks](10-hooks.md) | 工具前后生命周期脚本 |
| 11 | [自定义模型](11-custom-models.md) | BYOK、Ollama、兼容端点 |
| 12 | [项目规则 AGENTS.md](12-project-rules.md) | 目录级指令与优先级 |
| 13 | [记忆 Memory](13-memory.md) | 跨会话知识与搜索 |

## 第三层：进阶

| # | 文档 | 说明 |
|---|------|------|
| 14 | [无头模式](14-headless-mode.md) | `grok -p`、CI/CD |
| 15 | [代理模式与 IDE](15-agent-mode.md) | ACP、WebSocket、SDK |
| 16 | [子代理](16-subagents.md) | 并行子会话与角色 |
| 17 | [会话管理](17-sessions.md) | 保存、恢复、回退、压缩 |
| 18 | [沙箱](18-sandbox.md) | 文件系统与网络隔离 |
| 19 | [计划模式](19-plan-mode.md) | 规划与批准后再改代码 |
| 20 | [后台任务](20-background-tasks.md) | background、/loop、monitor |
| 21 | [终端支持](21-terminal-support.md) | tmux、SSH、剪贴板 |
| 22 | [权限与安全](22-permissions-and-safety.md) | 批准、沙箱、安全策略 |
| 23 | [仪表盘](23-dashboard.md) | 多会话总览 |
| 24 | [用量监控](24-monitoring-usage.md) | OpenTelemetry 导出 |

---

**汉化仓库**：[ivan6232/grok-build-zh](https://github.com/ivan6232/grok-build-zh)（社区非官方）
