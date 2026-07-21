# 翻译贡献指南

## 原则

1. **用户可见优先**：按钮、设置名、权限文案、错误提示  
2. **术语统一**：Permission=权限，Agent=代理，Session=会话，Scrollback=回滚，Tool=工具  
3. **保留专有名词**：Grok、Mermaid、Vim、SSH、API 等可不译或括注  
4. **不要翻译**：测试里的 option id（如 `allow-once`）、配置键名、serde 字段名  

## 高频文件

- `crates/codegen/xai-grok-pager/src/settings/defs.rs` — 设置 label/description  
- `crates/codegen/xai-grok-pager/src/settings/registry.rs` — 设置分类名  
- `crates/codegen/xai-grok-workspace/src/permission/prompter.rs` — 权限选项  
- `crates/codegen/xai-grok-pager/src/views/permission_view.rs` — 权限 UI  

## 提交流程

1. Fork 本仓库，分支命名 `i18n/xxx`  
2. 只改文案字符串，避免无关格式化  
3. PR 说明改了哪些界面，最好附图  

## 长期目标

引入资源文件（如 Fluent）实现 `ui.language = "zh-CN" | "en"`，与官方更好协作。
