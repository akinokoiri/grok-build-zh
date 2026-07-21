# 自定义模型

## 场景

- 自带 Key（BYOK）  
- 本地 Ollama  
- OpenAI 兼容网关  

## 配置思路

在 `config.toml` 的模型段声明：

- `base_url` / endpoint  
- `api_key` 环境变量名  
- 模型 ID 与显示名  

```toml
# 示意 — 以当前版本 schema 为准
[models]
default = "my-model"
```

切换：`/model` 或设置 → 默认模型。
