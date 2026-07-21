# 内置人设 / 角色 / 代理 — 中文描述覆盖包

本目录提供 **description 字段** 的简体中文版（ID 名称保持英文）。

## 应用到本机（可选）

官方安装的 `~/.grok/bundled/` 在更新客户端时可能被覆盖。汉化版安装后可手动合并：

```bash
# 备份
cp -a ~/.grok/bundled ~/.grok/bundled.bak.$(date +%Y%m%d)

# 覆盖描述相关文件（ID 不变，仅 description 中文）
cp docs/zh/bundled-zh/personas/*.toml ~/.grok/bundled/personas/
cp docs/zh/bundled-zh/roles/*.toml ~/.grok/bundled/roles/
# agents 的 frontmatter description 已中文；正文系统提示仍为英文（给模型的指令）
cp docs/zh/bundled-zh/agents/*.md ~/.grok/bundled/agents/
```

也可在安装 `grok-zh` 后执行：

```bash
# 若从本仓库克隆
bash docs/zh/bundled-zh/apply.sh
```

## 原则

| 字段 | 处理 |
|------|------|
| 名称 / ID | 不改（如 `reviewer`） |
| `description`（列表展示） | 中文 |
| 系统 `instructions` / prompt 正文 | 默认保留英文（模型指令）；需要可自行改 |

市场插件描述来自远程目录，不在本包内。
