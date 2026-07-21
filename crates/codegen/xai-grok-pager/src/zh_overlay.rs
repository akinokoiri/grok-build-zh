//! Chinese description overlays for content we do not own (marketplace
//! remote catalogs, etc.). Plugin/skill **IDs stay English**; only the
//! user-visible description (and a few category labels) are remapped.
//!
//! Source of truth for the map (docs + future apply tooling):
//! `docs/zh/bundled-zh/marketplace/plugins.json`

/// Official xAI marketplace plugin id → simplified Chinese description.
pub fn marketplace_plugin_description(name: &str) -> Option<&'static str> {
    // Keep in sync with docs/zh/bundled-zh/marketplace/plugins.json
    Some(match name {
        "vercel" => {
            "Vercel 部署平台集成。可在 Grok 中管理部署、查看构建状态与日志、配置域名，并控制前端基础设施。"
        }
        "sentry" => {
            "Sentry 错误监控集成。可查看错误报告、分析堆栈、按指纹搜索问题，并在开发环境中调试生产错误。"
        }
        "chrome-devtools" => {
            "Chrome DevTools 集成。控制并检查实时 Chrome 浏览器：录制性能轨迹、分析网络请求、查看带 source map 的控制台消息，并自动化浏览器操作。"
        }
        "cloudflare" => {
            "Cloudflare 开发者平台技能：Workers、Durable Objects、Agents SDK、MCP 服务器、Wrangler CLI 与 Web 性能。"
        }
        "superpowers" => {
            "软件开发核心技能库：测试驱动开发、系统化调试、协作模式，以及经过验证的工程工作流与技巧。"
        }
        "mongodb" => {
            "MongoDB 官方插件（MCP Server + Skills）。连接数据库、浏览数据、管理集合、优化查询、生成可靠代码、实践最佳做法并开发高级功能等。"
        }
        "axiom" => {
            "Axiom 可观测性官方集成（MCP Server + Skills）。用 APL 查询日志与指标、假设驱动的 SRE 排查、构建仪表盘、管理监控与告警、翻译 Splunk SPL，并分析与优化成本。"
        }
        "neon" => {
            "Neon Serverless Postgres 集成（MCP Server + Skills）。上手 Neon、管理项目与数据库、选择连接方式，并创建分支用于迁移测试与隔离开发。"
        }
        "firecrawl" => {
            "将任意网站转为干净、适合 LLM 的 Markdown 或结构化数据。通过内置托管 Firecrawl MCP 搜索、抓取、映射、爬取与提取实时网页数据——合格网络可无 Key 使用（每月 1000 免费额度），也可选免费 API Key 提高用量。自动 JS 渲染、反爬与代理轮换；另有 CLI 技能作回退。"
        }
        "figma" => {
            "Figma 官方 MCP 服务器与技能，面向设计到代码工作流。读取 Figma 设计上下文、实现设计、使用 Code Connect、写入画布，以及从网页生成 Figma 设计。"
        }
        "railway" => {
            "Railway 部署平台集成（MCP + 技能）。创建项目、开通服务与数据库、部署代码、管理环境/变量/卷/对象存储与功能开关、配置域名、排查构建失败，并在 Grok 中查看状态与指标。"
        }
        "stripe" => {
            "面向 Grok Build 的 Stripe 开发插件：最佳实践、API/SDK 升级指引，以及 Stripe MCP 服务器。"
        }
        _ => return None,
    })
}

/// Marketplace source catalog description (by marketplace name).
pub fn marketplace_source_description(name: &str) -> Option<&'static str> {
    match name {
        "xai-official" => Some("xAI 官方插件市场"),
        _ => None,
    }
}

/// Category slug → Chinese label for display.
pub fn marketplace_category_label(category: &str) -> &str {
    match category {
        "deployment" => "部署",
        "monitoring" => "监控",
        "development" => "开发",
        "database" => "数据库",
        "observability" => "可观测性",
        other => other,
    }
}

/// Prefer Chinese overlay; fall back to the remote/original description.
pub fn display_plugin_description(name: &str, original: Option<&str>) -> String {
    if let Some(zh) = marketplace_plugin_description(name) {
        return zh.to_string();
    }
    original.unwrap_or("").to_string()
}

/// Whether `query` matches the plugin name or its (possibly overlaid) description.
pub fn plugin_matches_query(name: &str, original_desc: Option<&str>, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.to_lowercase();
    if name.to_lowercase().contains(&q) {
        return true;
    }
    let desc = display_plugin_description(name, original_desc);
    desc.to_lowercase().contains(&q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_plugins_have_zh() {
        for id in [
            "vercel",
            "sentry",
            "chrome-devtools",
            "cloudflare",
            "superpowers",
            "mongodb",
            "axiom",
            "neon",
            "firecrawl",
            "figma",
            "railway",
            "stripe",
        ] {
            let d = marketplace_plugin_description(id).expect(id);
            assert!(
                d.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)),
                "{id} description should contain CJK"
            );
        }
    }

    #[test]
    fn unknown_falls_back() {
        assert_eq!(
            display_plugin_description("totally-unknown", Some("Hello")),
            "Hello"
        );
        assert!(marketplace_plugin_description("totally-unknown").is_none());
    }

    #[test]
    fn search_matches_zh_description() {
        assert!(plugin_matches_query(
            "neon",
            Some("Neon Serverless Postgres"),
            "Postgres"
        ));
        assert!(plugin_matches_query("neon", Some("Neon"), "分支"));
        assert!(!plugin_matches_query("neon", Some("Neon"), "zzzz"));
    }
}
