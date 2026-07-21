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
| 市场插件 / 第三方技能 | 远程目录 | ⚠️ **不在本包**（上游英文，除非作者提供中文） |

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

## 市场插件为什么仍是英文？

市场插件的 description 来自远程 marketplace / 缓存（如 `~/.grok/marketplace-cache/`），  
每次同步可能被上游覆盖。本仓库**不修改第三方远程内容**。

若需要中文，可：

1. 向插件作者要中文描述，或  
2. 在本地缓存里手工改 `plugin.json` / `SKILL.md`（更新后可能丢失）

## 客户端更新后

官方/预编译包更新可能还原 `~/.grok/bundled/`。  
重新执行一次 `bash docs/zh/bundled-zh/apply.sh` 即可。
