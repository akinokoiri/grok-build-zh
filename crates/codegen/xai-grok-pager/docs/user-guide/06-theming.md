# 主题与外观

## 切换主题

- 命令：`/theme`  
- 或设置：`/settings` → 外观 → 主题  
- 或配置：

```toml
[ui]
# 具体键名以当前版本 settings 为准
```

内置主题示例（名称可能随版本变化）：跟随系统、Grok Night、浅色、truecolor 变体等。

## 真彩色

现代终端（iTerm2、Kitty、WezTerm、Windows Terminal 等）通常支持 truecolor。若颜色异常：

1. 确认 `COLORTERM=truecolor`  
2. 在 tmux 中正确传递 Tc / RGB  
3. 换主题试是否需要 truecolor  

## pager.toml

可细调滚动、折叠、字体宽度相关显示。详见设置面板中文说明或上游 `06-theming` 英文原文中的键表。
