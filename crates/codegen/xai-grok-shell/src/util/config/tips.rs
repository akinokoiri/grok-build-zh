use super::RemoteSettings;
use serde::Deserialize;
use toml::Value as TomlValue;

/// Read `[cli] show_tips` from config.toml. Returns `None` if not set.
/// When `Some(false)`, the tip-of-the-day is suppressed on startup.
pub fn show_tips_from_toml_opt(root: &TomlValue) -> Option<bool> {
    if let TomlValue::Table(table) = root
        && let Some(TomlValue::Table(cli)) = table.get("cli")
    {
        cli.get("show_tips").and_then(|v| v.as_bool())
    } else {
        None
    }
}
/// Local `[tips]` config section.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TipsOverride {
    pub tips: Vec<String>,
    /// When true, drop remote/default tips entirely.
    pub exclude_default: bool,
}

/// Parse `[tips]` from a TOML value.
pub fn tips_from_toml(root: &TomlValue) -> Option<TipsOverride> {
    root.get("tips")?.clone().try_into::<TipsOverride>().ok()
}

/// 汉化版内置中文 tips（替代远程英文 tip-of-the-day）。
pub fn chinese_default_tips() -> Vec<String> {
    [
        "可以让 Agent 搜索 X 或网页。",
        "用 /model 切换模型与推理强度。",
        "用 /settings 或 F2 打开设置。",
        "用 Ctrl+S 恢复历史会话。",
        "用 Ctrl+W 在 git 工作树中开新会话。",
        "用 /compact 压缩上下文以腾出空间。",
        "用 /plan 进入计划模式，先想清楚再改代码。",
        "用 Ctrl+. 查看键盘快捷键。",
        "用 /docs 打开应用内使用指南。",
        "权限模式可选：询问 / 自动 / 始终批准。",
        "多行输入可用 /multiline 切换。",
        "用 /memory 浏览跨会话记忆。",
        "用 /dashboard 管理并行代理。",
        "粘贴图片时，可让 Agent 结合截图分析。",
        "用 @ 引用文件或路径作为上下文。",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

/// Merge tip sources in priority order.
///
/// 汉化版：默认使用内置中文 tips，而不是远程英文 tips。
/// If any local source sets `exclude_default = true`, built-in/remote tips are dropped.
/// User/managed/requirements tips are still appended.
pub fn merge_tips(
    requirements: Option<TipsOverride>,
    user: Option<TipsOverride>,
    managed: Option<TipsOverride>,
    remote_tips: Option<&[String]>,
) -> Vec<String> {
    let exclude = [&requirements, &user, &managed]
        .into_iter()
        .flatten()
        .any(|s| s.exclude_default);

    let mut out = Vec::new();
    if let Some(src) = requirements.as_ref() {
        out.extend(src.tips.iter().cloned());
    }
    if !exclude {
        // Prefer Chinese defaults over remote English tips (zh-CN fork).
        let zh = chinese_default_tips();
        if !zh.is_empty() {
            out.extend(zh);
        } else if let Some(remote) = remote_tips {
            out.extend(remote.iter().cloned());
        }
    }
    if let Some(src) = user.as_ref() {
        out.extend(src.tips.iter().cloned());
    }
    if let Some(src) = managed.as_ref() {
        out.extend(src.tips.iter().cloned());
    }
    out
}

/// Resolve the merged tip list from pre-loaded config layers.
///
/// Priority: requirements > remote > user config > managed config.
/// `GROK_TIPS_OVERRIDE` env var overrides everything (debug builds only).
/// `[cli] show_tips = false` in requirements or user config kills all tips.
pub fn resolve_tips(
    requirements: Option<&TomlValue>,
    user: Option<&TomlValue>,
    managed: Option<&TomlValue>,
    remote_tips: Option<&[String]>,
) -> Vec<String> {
    if requirements.and_then(show_tips_from_toml_opt) == Some(false) {
        return Vec::new();
    }
    if user.and_then(show_tips_from_toml_opt) == Some(false) {
        return Vec::new();
    }

    #[cfg(debug_assertions)]
    if let Ok(raw) = std::env::var("GROK_TIPS_OVERRIDE") {
        return raw.split('|').map(str::to_string).collect();
    }

    let req = requirements.and_then(tips_from_toml);
    let usr = user.and_then(tips_from_toml);
    let mgd = managed.and_then(tips_from_toml);

    // Priority: requirements > remote > user > managed.
    merge_tips(req, usr, mgd, remote_tips)
}

/// Convenience wrapper that loads config layers from disk and picks one tip.
/// Prefer [`resolve_tips`] when layers are already loaded.
pub fn resolve_tips_from_disk(
    raw_config: &TomlValue,
    remote_settings: Option<&RemoteSettings>,
    grok_home: &std::path::Path,
) -> Option<String> {
    let requirements = crate::config::load_merged_requirements();
    let managed = crate::config::load_managed_config().ok();
    let remote = remote_settings.and_then(|s| s.tips.as_deref());

    let all = resolve_tips(
        requirements.as_ref(),
        Some(raw_config),
        managed.as_ref(),
        remote,
    );
    if all.is_empty() {
        return None;
    }
    crate::util::tips::pick_and_advance(&all, grok_home)
}

/// Read `[cli] channel` from config.toml.
/// Returns `None` when absent (falls through to remote settings).
pub fn channel_from_toml_opt(root: &TomlValue) -> Option<String> {
    if let TomlValue::Table(table) = root
        && let Some(TomlValue::Table(cli)) = table.get("cli")
    {
        cli.get("channel")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::RemoteSettings;
    use super::*;
    use toml::Value as TomlValue;

    #[test]
    fn show_tips_defaults_to_none() {
        let config = TomlValue::Table(toml::map::Map::new());
        assert_eq!(show_tips_from_toml_opt(&config), None);
    }

    #[test]
    fn show_tips_reads_false() {
        let config: TomlValue = toml::from_str("[cli]\nshow_tips = false").unwrap();
        assert_eq!(show_tips_from_toml_opt(&config), Some(false));
    }

    #[test]
    fn show_tips_reads_true() {
        let config: TomlValue = toml::from_str("[cli]\nshow_tips = true").unwrap();
        assert_eq!(show_tips_from_toml_opt(&config), Some(true));
    }

    #[test]
    fn remote_settings_tips_absent() {
        let json = r#"{}"#;
        let s: RemoteSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.tips, None);
    }

    #[test]
    fn remote_settings_tips_null() {
        let json = r#"{"tips": null}"#;
        let s: RemoteSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.tips, None);
    }

    #[test]
    fn remote_settings_tips_empty() {
        let json = r#"{"tips": []}"#;
        let s: RemoteSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.tips, Some(vec![]));
    }

    #[test]
    fn remote_settings_tips_populated() {
        let json = r#"{"tips": ["a", "b"]}"#;
        let s: RemoteSettings = serde_json::from_str(json).unwrap();
        assert_eq!(s.tips, Some(vec!["a".to_string(), "b".to_string()]));
    }
}
