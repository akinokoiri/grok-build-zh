# 内置内容 — 中文描述覆盖包

本目录把**列表里展示给用户的 description** 换成简体中文。  
**ID / name 保持英文**；给模型看的指令正文默认不改。

## 覆盖范围

| 类型 | 路径 | 处理方式 |
|------|------|----------|
| 人设 personas | `personas/*.toml` | 整文件覆盖（仅 description 中文） |
| 角色 roles | `roles/*.toml` | 同上 |
| 代理 agents | `agents/*.md` | frontmatter description 中文 |
| **技能 skills** | `skills/descriptions.json` | 就地改写本机 `SKILL.md` 的 description 字段 |
| **市场插件** | `marketplace/plugins.json` | **运行时映射**（`grok-zh` 内置，见下） |
| 未收录的第三方 / 新上架插件 | 远程目录 | 仍显示上游英文 description；可 PR 补映射 |

## 一键应用

```bash
# 在克隆的 grok-build-zh 仓库中
git checkout zh-CN
bash docs/zh/bundled-zh/apply.sh
```

或指定配置目录：

```bash
GROK_HOME=~/.grok bash docs/zh/bundled-zh/apply.sh
```

脚本会：

1. 覆盖 `~/.grok/bundled/personas|roles|agents`  
2. 用 `python3` 把 `descriptions.json` 写进  
   - `~/.grok/bundled/skills/*/SKILL.md`  
   - `~/.grok/skills/*/SKILL.md`（若存在）  

**重启 `grok-zh`** 后打开 `/skills` 即可看到中文描述。

## 原则

| 字段 | 处理 |
|------|------|
| 名称 / ID（如 `reviewer`、`pptx`） | **不改** |
| `description` / `short-description`（列表展示） | **中文** |
| 技能正文 / 系统 instructions | 默认保留英文（模型指令） |

## 市场插件描述映射（运行时）

**不改**远程 `marketplace.json` / 缓存文件。  
`grok-zh` 在扩展市场 UI 展示时，按插件 **ID** 套用中文 description：

| 文件 | 作用 |
|------|------|
| [marketplace/plugins.json](marketplace/plugins.json) | 人类可读映射表（文档 / 贡献入口） |
| `crates/.../zh_overlay.rs` | 编译进二进制的同一映射 |

当前已覆盖 xAI 官方市场常见插件（ID 保持英文）：

`vercel` · `sentry` · `chrome-devtools` · `cloudflare` · `superpowers` · `mongodb` · `axiom` · `neon` · `firecrawl` · `figma` · `railway` · `stripe`

分类标签也会显示为中文（部署 / 监控 / 开发 / 数据库 / 可观测性）。

**补映射：** 在 `plugins.json` 与 `zh_overlay.rs` 各加一条同 ID 中文描述，提 PR。  
未收录 ID 仍显示上游英文原文。

## 客户端更新后

官方/预编译包更新可能还原 `~/.grok/bundled/`。  
重新执行一次 `bash docs/zh/bundled-zh/apply.sh` 即可。
