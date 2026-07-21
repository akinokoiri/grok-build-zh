# 身份认证

Grok Build 支持多种登录方式。

## 浏览器登录（默认）

首次运行 `grok` 会打开浏览器登录 grok.com。成功后凭证写入：

```text
~/.grok/auth.json
```

会自动刷新；无法续期时再要求登录。

## API Key

适合 CI/无头环境：

```bash
export XAI_API_KEY="xai-..."
grok
```

也可写在环境或 shell 配置中。**不要**把 Key 提交到 git。

## 其它方式

上游还支持 OIDC、外部身份提供商、设备码流程等。完整字段与环境变量见源码旁英文原文或官方文档；配置键名保持英文。

常见文件：

| 路径 | 用途 |
|------|------|
| `~/.grok/auth.json` | 登录凭证 |
| `~/.grok/config.toml` | 主配置 |

排障：

1. 删除过期 `auth.json` 后重新 `grok` 登录  
2. 检查系统时间是否准确（影响 token）  
3. 公司代理/防火墙是否拦截浏览器回调  
