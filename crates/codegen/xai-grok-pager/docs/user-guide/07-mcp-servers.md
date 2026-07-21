# MCP 服务器

Model Context Protocol（MCP）用于接入外部工具（浏览器、数据库、工单系统等）。

## 配置位置

常见：`~/.grok/config.toml` 中的 `[mcp_servers.*]` 段，或项目级 MCP 配置（以当前版本为准）。

示例结构（示意）：

```toml
[mcp_servers.example]
command = "npx"
args = ["-y", "some-mcp-server"]
```

## 使用

- `/mcp` 查看与管理  
- 权限提示中批准 MCP 工具调用  
- 日志中可看到 MCP 服务器状态  

## 注意

- 只运行信任的 MCP 命令  
- 需要网络/密钥的服务请用环境变量注入，勿写入仓库  
