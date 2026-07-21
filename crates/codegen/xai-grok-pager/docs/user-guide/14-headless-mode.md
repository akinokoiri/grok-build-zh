# 无头模式与脚本

非交互运行适合 CI/CD：

```bash
grok -p "运行测试并总结失败原因"
```

## 常用点

- 输出格式（文本/JSON 等，以 `--help` 为准）  
- 通过环境变量注入 `XAI_API_KEY`  
- 管道与退出码  

```bash
grok -p "lint the project" --help   # 查看当前版本参数
```

权限：无头场景常配合 yolo/预批准策略，**务必在可信 CI 中使用**。
