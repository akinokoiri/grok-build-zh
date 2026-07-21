# 配置

Grok 从配置文件、环境变量与 CLI 参数读取设置。

## 优先级（高 → 低）

1. CLI 参数（如 `--yolo`、`--model`）  
2. 环境变量（如 `XAI_API_KEY`）  
3. `~/.grok/config.toml`  
4. 组织托管配置（若有）  
5. 内置默认值  

## 主配置：`~/.grok/config.toml`

常见段落：

```toml
[cli]
auto_update = true

[models]
default = "grok-build"

[ui]
simple_mode = true          # 提示区 readline 编辑
vim_mode = false            # 回滚 Vim 导航
show_thinking_blocks = true
screen_mode = "fullscreen"  # 或 "minimal"
remember_tool_approvals = false

[session]
auto_compact_threshold_percent = 85

[features]
telemetry = false
```

汉化版设置面板中的中文名称对应上述英文键；**键名不要改成中文**。

## `pager.toml`

控制滚动、块折叠、显示等更细 UI。可与 `/settings` 联动写入。

## 常用环境变量

| 变量 | 含义 |
|------|------|
| `XAI_API_KEY` | API 密钥登录 |
| `GROK_*` | 多种覆盖项（见上游完整列表） |

## 文件位置

| 路径 | 用途 |
|------|------|
| `~/.grok/config.toml` | 主配置 |
| `~/.grok/auth.json` | 认证 |
| `~/.grok/sessions/` | 会话数据 |
| `~/.grok/docs/user-guide/` | 导出的用户指南 |

修改配置后部分项需重启 TUI 生效（设置项若标注「需重启」请重启）。
