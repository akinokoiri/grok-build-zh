# 钩子（Hooks）

钩子在工具调用前/后运行脚本或 HTTP 回调，用于审计、拦截危险命令、自定义策略。

## 常见事件

- PreToolUse / PostToolUse  
- 会话开始/结束（若版本支持）  

## 配置

项目或用户级 hooks 配置（JSON/脚本路径）。匹配器可限制只对 `git`/`gh` 等命令生效。

与 [权限与安全](22-permissions-and-safety.md) 配合，可实现「只允许特定命令」等策略。
