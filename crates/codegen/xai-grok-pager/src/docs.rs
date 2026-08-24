//! In-app how-to documentation data (embedded markdown).
//!
//! Single source of truth: two static arrays (`USER_GUIDE`, `REFERENCE_DOCS`)
//! hold every doc. All lookups are zero-allocation; `DocEntry` exists only for
//! backward compatibility with the TUI doc picker.

/// A compile-time document entry. All fields are `&'static str`.
#[derive(Debug)]
pub struct Doc {
    pub filename: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub content: &'static str,
}

/// Owned variant for the TUI doc picker (backward compat).
#[derive(Debug, Clone)]
pub struct DocEntry {
    pub title: String,
    pub description: String,
    /// Embedded markdown content.
    pub content: &'static str,
}

impl From<&Doc> for DocEntry {
    fn from(d: &Doc) -> Self {
        Self {
            title: d.title.into(),
            description: d.description.into(),
            content: d.content,
        }
    }
}

// ── Static doc tables ────────────────────────────────────────────────────────

macro_rules! guide {
    ($file:literal, $title:literal, $desc:literal) => {
        Doc {
            filename: $file,
            title: $title,
            description: $desc,
            content: include_str!(concat!("../docs/user-guide/", $file)),
        }
    };
}

pub static USER_GUIDE: &[Doc] = &[
    guide!(
        "01-getting-started.md",
        "入门指南",
        "安装、首次启动与基本操作"
    ),
    guide!(
        "02-authentication.md",
        "身份认证",
        "浏览器登录、API Key、OIDC 与外部认证"
    ),
    guide!(
        "03-keyboard-shortcuts.md",
        "键盘快捷键",
        "TUI 全部快捷键一览"
    ),
    guide!(
        "04-slash-commands.md",
        "斜杠命令",
        "会话、模型、记忆、目标、工作流与钩子等 / 命令"
    ),
    guide!(
        "05-configuration.md",
        "配置",
        "config.toml、pager.toml、环境变量与文件位置"
    ),
    guide!(
        "06-theming.md",
        "主题与外观",
        "主题、颜色支持与 pager.toml 自定义"
    ),
    guide!(
        "07-mcp-servers.md",
        "MCP 服务器",
        "通过 MCP 接入外部工具"
    ),
    guide!(
        "08-skills.md",
        "技能 (Skills)",
        "创建与使用可复用提示包"
    ),
    guide!(
        "09-plugins.md",
        "插件与市场",
        "安装、管理与创建插件包"
    ),
    guide!(
        "10-hooks.md",
        "钩子 (Hooks)",
        "工具调用前后的生命周期脚本"
    ),
    guide!(
        "11-custom-models.md",
        "自定义模型",
        "BYOK、Ollama、OpenAI 兼容端点"
    ),
    guide!(
        "12-project-rules.md",
        "项目规则 (AGENTS.md)",
        "按目录的指令与优先级"
    ),
    guide!(
        "13-memory.md",
        "记忆",
        "跨会话知识持久化与搜索"
    ),
    guide!(
        "14-headless-mode.md",
        "无头模式与脚本",
        "自动化与 CI/CD 的非交互 CLI"
    ),
    guide!(
        "15-agent-mode.md",
        "代理模式与 IDE",
        "ACP stdio、WebSocket 与 SDK 集成"
    ),
    guide!(
        "16-subagents.md",
        "子代理与角色",
        "并行子代理与专用角色"
    ),
    guide!(
        "17-sessions.md",
        "会话管理",
        "保存、加载、恢复、回退与压缩"
    ),
    guide!(
        "18-sandbox.md",
        "沙箱模式",
        "操作系统级文件系统与网络隔离"
    ),
    guide!(
        "19-plan-mode.md",
        "计划模式",
        "结构化规划与批准对话框"
    ),
    guide!(
        "20-background-tasks.md",
        "后台任务与监控",
        "后台命令、/loop、monitor、调度器"
    ),
    guide!(
        "21-terminal-support.md",
        "终端支持与排障",
        "tmux、SSH、truecolor、剪贴板与诊断"
    ),
    guide!(
        "22-permissions-and-safety.md",
        "权限与安全",
        "模式、授权顺序、允许/询问/拒绝规则匹配与钩子"
    ),
    guide!(
        "23-dashboard.md",
        "代理仪表盘",
        "实时多会话名册：快速查看、分发、固定、停止与搜索"
    ),
    guide!(
        "24-monitoring-usage.md",
        "用量监控 (外部 OpenTelemetry)",
        "导出用量指标至 OpenTelemetry Collector"
    ),
    guide!(
        "25-status-line.md",
        "状态栏",
        "底部实时会话上下文行，或运行自定义脚本输出"
    ),
    guide!(
        "26-config-reference.md",
        "配置参考",
        "config.toml、managed_config.toml 与 requirements.toml 字段列表"
    ),
];

/// Non-user-guide reference docs. Separate from USER_GUIDE because they
/// live under `docs/` (not `docs/user-guide/`), are not extracted to disk,
/// and do not follow the NN-*.md managed naming pattern. Bundled via
/// `include_str!` so they are available at runtime without a docs path.
static REFERENCE_DOCS: &[Doc] = &[
    Doc {
        filename: "hooks-and-plugins.md",
        title: "钩子与插件指南",
        description: "使用钩子、插件与市场",
        content: include_str!("../docs/hooks-and-plugins.md"),
    },
    Doc {
        filename: "custom-hooks.md",
        title: "创建自定义钩子",
        description: "编写自己的钩子与匹配规则",
        content: include_str!("../docs/custom-hooks.md"),
    },
];

// ── Public API ───────────────────────────────────────────────────────────────

/// Find a doc by title (case-insensitive). Returns the static entry.
pub fn find_doc(title: &str) -> Option<&'static Doc> {
    USER_GUIDE
        .iter()
        .chain(REFERENCE_DOCS.iter())
        .find(|d| d.title.eq_ignore_ascii_case(title))
}

/// All doc titles, zero allocation.
pub fn all_titles() -> impl Iterator<Item = &'static str> {
    USER_GUIDE
        .iter()
        .chain(REFERENCE_DOCS.iter())
        .map(|d| d.title)
}

/// Returns the content of a how-to document by exact title match (case-insensitive).
pub fn get_howto_doc(title: &str) -> Option<&'static str> {
    find_doc(title).map(|d| d.content)
}

/// Returns a list of available how-to titles for the model to choose from.
pub fn list_howto_titles() -> Vec<String> {
    all_titles().map(String::from).collect()
}

/// Returns all docs as owned `DocEntry` values for the TUI doc picker.
pub fn default_howto_entries() -> Vec<DocEntry> {
    USER_GUIDE
        .iter()
        .chain(REFERENCE_DOCS.iter())
        .map(DocEntry::from)
        .collect()
}

/// Extract user-guide docs to `<grok_home>/docs/user-guide/`.
///
/// Called from the pager binary startup so the model can read them from disk.
pub fn extract_user_guide_docs(grok_home: &std::path::Path) {
    let docs_dir = grok_home.join("docs").join("user-guide");
    if let Err(e) = std::fs::create_dir_all(&docs_dir) {
        tracing::warn!(error = %e, "无法创建用户指南目录");
        return;
    }
    for doc in USER_GUIDE {
        if let Err(e) = std::fs::write(docs_dir.join(doc.filename), doc.content) {
            tracing::debug!(error = %e, filename = doc.filename, "无法导出用户指南文档");
        }
    }
    // Clean up stale managed docs (files removed from USER_GUIDE since last run).
    // Only remove files matching the managed naming pattern (NN-*.md).
    if let Ok(entries) = std::fs::read_dir(&docs_dir) {
        let valid: std::collections::HashSet<&str> =
            USER_GUIDE.iter().map(|d| d.filename).collect();
        for dir_entry in entries.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let is_managed = name.len() > 3
                    && name.as_bytes()[0].is_ascii_digit()
                    && name.as_bytes()[1].is_ascii_digit()
                    && name.as_bytes()[2] == b'-'
                    && name.ends_with(".md");
                if is_managed
                    && !valid.contains(name)
                    && let Err(e) = std::fs::remove_file(dir_entry.path())
                {
                    tracing::debug!(error = %e, filename = name, "无法删除过期用户指南文档");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_guide_entries_are_valid() {
        for doc in USER_GUIDE {
            assert!(!doc.content.is_empty(), "Doc {} is empty", doc.filename);
            assert!(
                !doc.title.is_empty(),
                "Doc {} has empty title",
                doc.filename
            );
            assert!(
                !doc.description.is_empty(),
                "Doc {} has empty description",
                doc.filename
            );
            assert!(
                doc.content.starts_with('#'),
                "Doc {} should start with a markdown header",
                doc.filename
            );
        }
    }

    #[test]
    fn user_guide_entries_have_no_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for doc in USER_GUIDE {
            assert!(
                seen.insert(doc.filename),
                "Duplicate doc in list: {}",
                doc.filename
            );
        }
    }

    #[test]
    fn default_howto_entries_includes_all_user_guide_docs() {
        let entries = default_howto_entries();
        assert_eq!(entries.len(), USER_GUIDE.len() + REFERENCE_DOCS.len());
        for (i, doc) in USER_GUIDE.iter().enumerate() {
            assert_eq!(entries[i].title, doc.title, "Entry {} title mismatch", i);
        }
    }

    #[test]
    fn find_doc_is_case_insensitive() {
        let doc = find_doc("getting started").expect("should find Getting Started");
        assert_eq!(doc.title, "入门指南");
        assert!(find_doc("nonexistent guide").is_none());
    }

    #[test]
    fn all_titles_covers_both_tables() {
        let titles: Vec<_> = all_titles().collect();
        assert_eq!(titles.len(), USER_GUIDE.len() + REFERENCE_DOCS.len());
    }

    #[test]
    fn get_howto_doc_delegates_to_find_doc() {
        assert!(get_howto_doc("入门指南").is_some());
        assert!(get_howto_doc("Hooks & Plugins Guide").is_some());
        assert!(get_howto_doc("no such doc").is_none());
    }

    #[test]
    fn list_howto_titles_returns_all() {
        let titles = list_howto_titles();
        assert_eq!(titles.len(), USER_GUIDE.len() + REFERENCE_DOCS.len());
    }

    #[test]
    fn extract_writes_docs_and_cleans_stale() {
        let tmp = tempfile::tempdir().unwrap();
        let docs_dir = tmp.path().join("docs").join("user-guide");

        std::fs::create_dir_all(&docs_dir).unwrap();
        std::fs::write(docs_dir.join("99-removed.md"), "stale").unwrap();
        std::fs::write(docs_dir.join("notes.md"), "user notes").unwrap();

        extract_user_guide_docs(tmp.path());

        for doc in USER_GUIDE {
            let path = docs_dir.join(doc.filename);
            assert!(path.exists(), "Expected doc {} to exist", doc.filename);
            let got = std::fs::read_to_string(&path).unwrap();
            assert_eq!(got, doc.content, "Content mismatch for {}", doc.filename);
        }
        assert!(
            !docs_dir.join("99-removed.md").exists(),
            "Stale doc should be cleaned up"
        );
        assert!(
            docs_dir.join("notes.md").exists(),
            "User file should not be deleted"
        );
    }
}
