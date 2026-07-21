# 终端支持与排障

## 推荐终端

iTerm2、Kitty、WezTerm、Alacritty、Windows Terminal、现代 Ghostty 等。

## tmux / SSH

- 正确配置 truecolor 透传  
- 剪贴板：OSC 52；SSH 可考虑 `grok wrap ssh`（若提供）  

## 常见问题

| 现象 | 尝试 |
|------|------|
| 颜色异常 | 检查 truecolor / 主题 |
| 快捷键失灵 | 是否被 tmux 前缀截获 |
| 剪贴板空 | OSC 52 是否允许、SSH 转发 |
| 显示错乱 | 终端字体与单元格宽度 |

诊断命令以当前版本 `/` 菜单与 `--help` 为准。
