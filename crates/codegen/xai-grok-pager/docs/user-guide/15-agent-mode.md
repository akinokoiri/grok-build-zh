# 代理模式与 IDE 集成

通过 **ACP（Agent Client Protocol）** 把 Grok 作为编辑器后端。

## 能力

- stdio 传输  
- WebSocket 中继（若启用）  
- 官方/社区 SDK（TypeScript、Rust、Python 等）  

IDE 插件负责 UI；Grok 负责工具与推理。详见上游 agent-mode 文档与 SDK 仓库。
