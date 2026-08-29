//! Small, fail-open localization catalog for the personal Chinese build.
//!
//! English source strings remain the canonical fallback. A bundled catalog is
//! compiled into the binary, while `<grok-home>/i18n/zh-CN.json` can override
//! individual entries without rebuilding Rust. Malformed or missing external
//! files are ignored so localization can never prevent the CLI from starting.

use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const EMBEDDED_ZH_CN: &str = include_str!("../i18n/zh-CN.json");
const CATALOG_VERSION: u32 = 1;
const MAX_EXTERNAL_CATALOG_BYTES: u64 = 1024 * 1024;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogDocument {
    version: u32,
    locale: String,
    translations: HashMap<String, String>,
}

#[derive(Debug)]
struct Catalog {
    translations: HashMap<String, String>,
    external_path: PathBuf,
    external_loaded: bool,
}

/// Diagnostic information for `grok-zh doctor` and tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogStatus {
    pub external_path: PathBuf,
    pub external_loaded: bool,
    pub translation_count: usize,
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();

/// Translate a stable UI id, borrowing the English fallback on a miss.
pub fn translate<'a>(id: &str, fallback: &'a str) -> Cow<'a, str> {
    match catalog().translations.get(id) {
        Some(value) => Cow::Owned(value.clone()),
        None => Cow::Borrowed(fallback),
    }
}

/// Translate a slash-command description using its canonical command name.
pub fn command_description<'a>(name: &str, fallback: &'a str) -> Cow<'a, str> {
    translate(&format!("command.{name}.description"), fallback)
}

/// Translate a built-in skill description. Third-party skills intentionally
/// fall back to their author-provided text unless an explicit id is present.
pub fn skill_description<'a>(id: &str, fallback: &'a str) -> Cow<'a, str> {
    translate(&format!("skill.{id}.description"), fallback)
}

/// Translate a bundled persona description by its stable id.
pub fn persona_description<'a>(id: &str, fallback: &'a str) -> Cow<'a, str> {
    translate(&format!("persona.{id}.description"), fallback)
}

/// Translate an installed first-party workflow description by its stable name.
pub fn workflow_description<'a>(name: &str, fallback: &'a str) -> Cow<'a, str> {
    translate(&format!("workflow.{name}.description"), fallback)
}

pub fn status() -> CatalogStatus {
    let catalog = catalog();
    CatalogStatus {
        external_path: catalog.external_path.clone(),
        external_loaded: catalog.external_loaded,
        translation_count: catalog.translations.len(),
    }
}

pub fn external_catalog_path() -> PathBuf {
    std::env::var_os("GROK_ZH_TRANSLATIONS")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| xai_dirs::grok_home().join("i18n").join("zh-CN.json"))
}

fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(|| load_catalog(&external_catalog_path()))
}

fn load_catalog(external_path: &Path) -> Catalog {
    let mut translations = parse_catalog(EMBEDDED_ZH_CN).unwrap_or_default();
    let external = read_external_catalog(external_path);
    let external_loaded = external.is_some();
    if let Some(overrides) = external {
        translations.extend(overrides);
    }
    Catalog {
        translations,
        external_path: external_path.to_path_buf(),
        external_loaded,
    }
}

fn read_external_catalog(path: &Path) -> Option<HashMap<String, String>> {
    let metadata = std::fs::metadata(path).ok()?;
    if metadata.len() > MAX_EXTERNAL_CATALOG_BYTES {
        tracing::warn!(path = %path.display(), "ignoring oversized Chinese translation catalog");
        return None;
    }
    let source = std::fs::read_to_string(path).ok()?;
    match parse_catalog(&source) {
        Some(catalog) => Some(catalog),
        None => {
            tracing::warn!(path = %path.display(), "ignoring invalid Chinese translation catalog");
            None
        }
    }
}

fn parse_catalog(source: &str) -> Option<HashMap<String, String>> {
    let document: CatalogDocument = serde_json::from_str(source).ok()?;
    if document.version != CATALOG_VERSION || document.locale != "zh-CN" {
        return None;
    }
    if document
        .translations
        .iter()
        .any(|(key, value)| key.trim().is_empty() || value.trim().is_empty())
    {
        return None;
    }
    Some(document.translations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_is_valid_and_nonempty() {
        let catalog = parse_catalog(EMBEDDED_ZH_CN).expect("embedded catalog must be valid");
        assert!(catalog.len() >= 50);
        assert_eq!(
            catalog.get("command.help.description").unwrap(),
            "浏览命令与键盘快捷键"
        );
    }

    #[test]
    fn invalid_external_catalog_keeps_embedded_fallback() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("zh-CN.json");
        std::fs::write(&path, "not json").unwrap();
        let catalog = load_catalog(&path);
        assert!(!catalog.external_loaded);
        assert_eq!(
            catalog
                .translations
                .get("command.help.description")
                .unwrap(),
            "浏览命令与键盘快捷键"
        );
    }

    #[test]
    fn external_catalog_overrides_individual_entries() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("zh-CN.json");
        std::fs::write(
            &path,
            r#"{"version":1,"locale":"zh-CN","translations":{"command.help.description":"覆盖值"}}"#,
        )
        .unwrap();
        let catalog = load_catalog(&path);
        assert!(catalog.external_loaded);
        assert_eq!(
            catalog
                .translations
                .get("command.help.description")
                .unwrap(),
            "覆盖值"
        );
        assert!(
            catalog
                .translations
                .contains_key("command.model.description")
        );
    }
}
