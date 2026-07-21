//! Default settings catalog — declares every user-tunable preference
//! registered in the settings modal.
//!
//! Defaults come from `UiConfig::default()` for SHELL/SHARED settings.
//! The `defaults_match_ui_config_default` test enforces this.

use super::registry::{
    DynamicEnumSource, EnumChoice, SettingCategory, SettingKind, SettingMeta, SettingOwner,
};
use crate::appearance::ScrollMode;
use crate::appearance::TextSelection;
use crate::appearance::permission_cursor::DefaultSelectedPermission;

use xai_grok_shell::agent::config::UiConfig;
use xai_grok_tools::implementations::grok_build::ask_user_question;

// ---------------------------------------------------------------------------
// Int bounds for `max_thoughts_width`.
//
// Stored as `u16` in `UiConfig`, exposed as `i64` for registry uniformity.
// 40 = min readable width on 80-col terminal; 500 = max before
// "obviously wrong" territory. `pub(crate)` so the dispatcher's clamp
// and the shell helper's defensive clamp share these bounds.
pub(crate) const MAX_THOUGHTS_WIDTH_MIN: i64 = 40;
pub(crate) const MAX_THOUGHTS_WIDTH_MAX: i64 = 500;

/// Registry key for `max_thoughts_width`. Shared between the registry
/// definition and the live-wrap-preview gate in the int stepper.
pub(crate) const MAX_THOUGHTS_WIDTH_KEY: &str = "max_thoughts_width";

// ---------------------------------------------------------------------------
// Theme choice catalogs.
//
// Canonical names MUST match `ThemeKind::display_name()`.
// Shared by `theme`, `auto_dark_theme`, and `auto_light_theme`;
// auto-* sub-pickers drop "auto" to avoid circular reference.
// Bounded by `MAX_PICKER_CHOICES`.
// ---------------------------------------------------------------------------

/// Full theme catalog including the "auto" meta-variant. Used by `theme` only.
const THEME_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "auto",
        display: "自动",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "groknight",
        display: "Grok Night",
        description: "中性深色，品红强调色。",
    },
    EnumChoice {
        canonical: "grokday",
        display: "Grok Day",
        description: "适用于明亮环境的浅色主题。",
    },
    EnumChoice {
        canonical: "tokyonight",
        display: "Tokyo Night",
        description: "深色偏蓝；需要 truecolor。",
    },
    // ASCII "Rose Pine Moon" (not "Rosé") for cross-terminal compatibility.
    EnumChoice {
        canonical: "rosepine-moon",
        display: "Rose Pine Moon",
        description: "柔和深色，淡紫强调；需要 truecolor。",
    },
    EnumChoice {
        canonical: "oscura-midnight",
        display: "Oscura Midnight",
        description: "深黑底暖色强调；需要 truecolor。",
    },
];

// ---------------------------------------------------------------------------
// Permission-mode catalog.
//
// Persisted values map onto runtime flags:
//   "always-approve" ↔ yolo_mode = true  (auto-approve all)
//   "auto"           ↔ auto_mode = true  (LLM classifier; not full yolo)
//   "ask"            ↔ both false (interactive prompts)
//   "default"        ↔ both false (agent's default — currently Ask)
//
// Canonical strings match `load_permission_mode`. `supports_preview:
// false` because toggling YOLO drains the permission queue (unsafe
// for per-keystroke preview).
//
// Adding new modes requires: (1) `PermissionModeKind` variant,
// (2) `EnumChoice` here, (3) `set_yolo_mode_inner` update,
// (4) `load_permission_mode` arm, (5) tests. `Plan` is excluded —
// it lives on its own `plan_mode` setting.
// ---------------------------------------------------------------------------

// Choice order: safe → classifier → unsafe (Default → Ask → Auto → Always approve).
// "始终批准" at the end creates a speed bump against
// accidental selection.
const PERMISSION_MODE_CHOICES: &[EnumChoice] = &[
    // "default" = agent's default behavior. Same as "ask" at runtime;
    // distinct on disk and in the modal indicator.
    EnumChoice {
        canonical: "default",
        display: "默认",
        description: "使用代理默认权限行为（当前等同于「询问」）。",
    },
    EnumChoice {
        canonical: "ask",
        display: "询问",
        description: "在工具操作前请求权限确认。",
    },
    EnumChoice {
        canonical: "auto",
        display: "自动",
        description: "由模型分类器批准安全工具；危险操作仍可能询问或拒绝。",
    },
    EnumChoice {
        canonical: "always-approve",
        display: "始终批准",
        description: "自动批准所有工具操作。跳过全部权限提示。",
    },
];

// ---------------------------------------------------------------------------
// Coding-data-sharing catalog.
//
// Persisted in auth metadata (`AuthEntry::coding_data_retention_opt_out`),
// NOT config.toml. Two choices only — the pager has no `Option`/`Unset`
// representation for this field.
//
// `supports_preview: false` — toggling fires an async ACP call that
// can fail. Commit on Enter only.
// ---------------------------------------------------------------------------

const CODING_DATA_SHARING_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "opt-in",
        display: "加入",
        description: "允许 SpaceXAI 保留编程会话数据用于模型训练与产品改进。",
    },
    EnumChoice {
        canonical: "opt-out",
        display: "退出",
        description: "不保留编程会话数据用于训练。不会关闭产品分析。",
    },
];

// ---------------------------------------------------------------------------
// Plan-mode catalog.
//
// PAGER-owned, per-session, ACP-mediated via `session/set_mode`.
// NOT persisted to config.toml — resets every session start.
//
// Uses `on`/`off` canonical strings (not the shell's `plan`/`default`
// wire ids). `Ask` mode is intentionally not exposed here — it's
// only reachable via Shift+Tab.
//
// `supports_preview: false` — toggling fires an ACP request that
// gates tool dispatch. Commit on Enter only.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Default-selected-permission catalog.
//
// Persisted to `[ui].default_selected_permission` in config.toml. Controls
// which row the cursor preselects on the FIRST permission prompt of a
// session; after the user confirms any prompt, the cursor sticks to the
// last-used option kind. `always_allow_all_sessions` (the effective default)
// lands the cursor on the "在所有会话中始终允许" / enable-always-approve
// row explicitly, via `is_enable_always_approve_option` — not via index 0; the
// other three map onto `acp::PermissionOptionKind::{AllowOnce, AllowAlways,
// Reject*}`.
//
// `supports_preview: false` — permission prompts aren't open in the modal
// background, so there's no live preview surface.
// ---------------------------------------------------------------------------

// Order matches the live permission prompt rendering (YOLO -> always-allow
// -> allow-once -> reject) so the picker mirrors what the user sees on the
// real prompt.
// Canonicals + display labels come from `DefaultSelectedPermission` (the
// single source of truth) so this table can never drift from the parser,
// the dispatch toast, or the cursor logic.
const DEFAULT_SELECTED_PERMISSION_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: DefaultSelectedPermission::AlwaysAllowAllSessions.as_canonical(),
        display: DefaultSelectedPermission::AlwaysAllowAllSessions.display(),
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: DefaultSelectedPermission::AllowCommandAlways.as_canonical(),
        display: DefaultSelectedPermission::AllowCommandAlways.display(),
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: DefaultSelectedPermission::AllowOnce.as_canonical(),
        display: DefaultSelectedPermission::AllowOnce.display(),
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: DefaultSelectedPermission::Reject.as_canonical(),
        display: DefaultSelectedPermission::Reject.display(),
        description: "跟随系统深色/浅色外观。",
    },
];

const PLAN_MODE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "off",
        display: "关",
        description: "代理直接运行工具并编辑文件（默认）。",
    },
    EnumChoice {
        canonical: "on",
        display: "开",
        description: "代理先总结计划，获批后再运行工具。",
    },
];

// ---------------------------------------------------------------------------
// Mermaid-rendering catalog.
//
// SHELL-owned: persisted to `[ui].render_mermaid`, with a pager-side
// process-wide cache mirror (`appearance::cache::*_render_mermaid`) for the
// render hot path. Canonicals match `RenderMermaid::as_canonical`.
// ---------------------------------------------------------------------------

const RENDER_MERMAID_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "auto",
        display: "自动",
        description: "以可点击行展示图表，可打开/复制渲染图。",
    },
    EnumChoice {
        canonical: "on",
        display: "开",
        description: "与 auto 相同：始终显示可点击操作行。",
    },
    EnumChoice {
        canonical: "off",
        display: "关",
        description: "始终以代码块显示 Mermaid 源码。",
    },
];

// Scroll-input catalog. SHELL-owned, persisted to `[ui].scroll_mode`.
// Canonical strings match `ScrollMode::as_canonical` (pinned by test).
const SCROLL_MODE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: ScrollMode::Auto.as_canonical(),
        display: "自动检测",
        description: "根据事件时序自动区分滚轮与触控板（默认）。",
    },
    EnumChoice {
        canonical: ScrollMode::Wheel.as_canonical(),
        display: "鼠标滚轮",
        description: "始终按滚轮刻度滚动（每次固定行数）。",
    },
    EnumChoice {
        canonical: ScrollMode::Trackpad.as_canonical(),
        display: "触控板",
        description: "始终按触控板方式滚动（小数累计）。",
    },
];

const TEXT_SELECTION_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: TextSelection::Flash.as_canonical(),
        display: "复制后闪烁",
        description: "鼠标松开后短暂高亮再清除。双击切换折叠。默认。",
    },
    EnumChoice {
        canonical: TextSelection::Hold.as_canonical(),
        display: "保持到关闭",
        description: "选区保持到 Esc、点击或滚动。双击切换折叠。",
    },
    EnumChoice {
        canonical: TextSelection::WordSelect.as_canonical(),
        display: "选词（类终端）",
        description: "双击选词并复制，三击选行；选区保持到关闭。",
    },
];

// Hunk-tracker-mode catalog. SHELL-owned, persisted to `[ui].hunk_tracker_mode`.
// `disabled` is accepted as an alias for `off` at parse time but not surfaced
// as a choice.
const HUNK_TRACKER_MODE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "agent_only",
        display: "Agent only",
        description: "仅跟踪代理编辑过的文件（默认）。",
    },
    EnumChoice {
        canonical: "all_dirty",
        display: "All dirty",
        description: "跟踪所有 git 脏文件，含外部修改。",
    },
    EnumChoice {
        canonical: "off",
        display: "关",
        description: "完全关闭变更块跟踪，并关闭 LOC 统计。",
    },
];

const SCREEN_MODE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "fullscreen",
        display: "Fullscreen",
        description: "以标准全屏 TUI 打开 grok。未设置时的默认。",
    },
    EnumChoice {
        canonical: "minimal",
        display: "Minimal",
        description: "以回滚原生（极简）模式打开 grok。",
    },
];

// Voice-capture-mode catalog. SHELL-owned, persisted to `[ui].voice_capture_mode`.
// `hold` is only offered on terminals that report key releases (Kitty keyboard
// protocol); `effective_enum_choices` hides it elsewhere, and it falls back to
// `toggle` at runtime.
const VOICE_CAPTURE_MODE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "toggle",
        display: "Toggle",
        description: "Ctrl+Space / F8 开始听写；再按（或 Esc/Enter）停止。",
    },
    EnumChoice {
        canonical: "hold",
        display: "Hold to talk",
        description: "按住 Ctrl+Space / F8 录音，松开停止。需要 Kitty 协议终端。",
    },
];

// Voice STT language choices for the settings modal.
//
// Concrete codes must match `xai_grok_voice::STT_LANGUAGES` (official Grok STT
// catalog — https://docs.x.ai/developers/model-capabilities/audio/speech-to-text).
// `auto` is client-only; the voice crate resolves it to a concrete code before
// the STT handshake. Order: English (default), System, then remaining languages
// A–Z by English name. A registry unit test locks this list to the voice crate.
const VOICE_STT_LANGUAGE_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "en",
        display: "English",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "auto",
        display: "System",
        description: "系统语言若为支持的语音识别语言则使用，否则英语。",
    },
    EnumChoice {
        canonical: "ar",
        display: "Arabic",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "cs",
        display: "Czech",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "da",
        display: "Danish",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "nl",
        display: "Dutch",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "fil",
        display: "Filipino",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "fr",
        display: "French",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "de",
        display: "German",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "hi",
        display: "Hindi",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "id",
        display: "Indonesian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "it",
        display: "Italian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "ja",
        display: "Japanese",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "ko",
        display: "Korean",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "mk",
        display: "Macedonian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "ms",
        display: "Malay",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "fa",
        display: "Persian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "pl",
        display: "Polish",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "pt",
        display: "Portuguese",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "ro",
        display: "Romanian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "ru",
        display: "Russian",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "es",
        display: "Spanish",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "sv",
        display: "Swedish",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "th",
        display: "Thai",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "tr",
        display: "Turkish",
        description: "跟随系统深色/浅色外观。",
    },
    EnumChoice {
        canonical: "vi",
        display: "Vietnamese",
        description: "跟随系统深色/浅色外观。",
    },
];

/// Concrete-only theme catalog (excludes "auto"). Used by both
/// `auto_dark_theme` and `auto_light_theme`. No dark/light filtering —
/// the user can pair any theme with any system-appearance bucket.
const CONCRETE_THEME_CHOICES: &[EnumChoice] = &[
    EnumChoice {
        canonical: "groknight",
        display: "Grok Night",
        description: "中性深色，品红强调色。",
    },
    EnumChoice {
        canonical: "grokday",
        display: "Grok Day",
        description: "适用于明亮环境的浅色主题。",
    },
    EnumChoice {
        canonical: "tokyonight",
        display: "Tokyo Night",
        description: "深色偏蓝；需要 truecolor。",
    },
    EnumChoice {
        canonical: "rosepine-moon",
        display: "Rose Pine Moon",
        description: "柔和深色，淡紫强调；需要 truecolor。",
    },
    EnumChoice {
        canonical: "oscura-midnight",
        display: "Oscura Midnight",
        description: "深黑底暖色强调；需要 truecolor。",
    },
];

/// Child settings shown inside the "Show contextual hints" group sub-sheet.
/// Keys match the `[ui.contextual_hints]` serde fields (namespaced so they stay
/// globally unique — bare `plan_mode` collides with the plan-mode enum row).
/// They are registered as normal Bool settings but hidden from the top-level
/// list (`build_rows` skips any key that is a group child).
const CONTEXTUAL_HINTS_CHILDREN: &[&str] = &[
    "contextual_hints.undo",
    "contextual_hints.plan_mode",
    "contextual_hints.image_input",
    "contextual_hints.send_now",
    "contextual_hints.small_screen",
    "contextual_hints.word_select",
    "contextual_hints.ssh_wrap",
];

/// Build the catalog. Called once at process start via
/// `SettingsRegistry::defaults()`.
pub fn default_settings() -> Vec<SettingMeta> {
    // Shell schema defaults, used as registry source of truth.
    let ui_default = UiConfig::default();

    vec![
        SettingMeta {
            key: "compact_mode",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "紧凑模式",
            description: "减少消息边距以提高密度。终端高度 ≤20 行时自动开启。需重启。",
            keywords: &[
                "compact", "density", "padding", "tight", "small", "screen", "auto",
            ],
            kind: SettingKind::Bool {
                default: ui_default.compact_mode,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "screen_mode",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "默认屏幕模式",
            description: "下次启动 plain grok 的方式：Fullscreen（默认）或 Minimal。写入 [ui] screen_mode。",
            keywords: &[
                "screen",
                "mode",
                "minimal",
                "fullscreen",
                "full",
                "scrollback",
                "native",
                "alt-screen",
                "render",
                "default",
            ],
            kind: SettingKind::Enum {
                default: "fullscreen",
                choices: SCREEN_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: true,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "show_timestamps",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "显示时间戳",
            description: "在用户消息与代理回复旁显示时钟时间。",
            keywords: &["timestamps", "time", "clock", "date"],
            kind: SettingKind::Bool {
                // `Option<bool>` — `None` treated as `true`.
                default: ui_default.show_timestamps.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "show_timeline",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "时间线侧栏",
            description: "用每轮时间刻度条替代滚动条：悬停预览，点击跳转。",
            keywords: &["timeline", "sidebar", "ticks", "turns", "navigator", "rail"],
            kind: SettingKind::Bool {
                // Single source: UiConfig::SHOW_TIMELINE_DEFAULT (opt-in).
                default: ui_default.show_timeline_enabled(),
            },
            restart_required: false,
            // Minimal mode has no interactive scrollback pane for the rail.
            hidden_in_minimal: true,
        },
        SettingMeta {
            key: "page_flip_on_send",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "发送后提示置顶",
            description: "发送提示后滚到屏幕顶部，使回复从新页开始（默认）。",
            keywords: &[
                "page", "flip", "send", "prompt", "scroll", "top", "jump", "auto", "snap",
            ],
            kind: SettingKind::Bool {
                default: ui_default.page_flip_on_send_enabled(),
            },
            restart_required: false,
            hidden_in_minimal: true,
        },
        SettingMeta {
            // Persisted key stays `simple_mode`; the user-facing label
            // distinguishes the PROMPT vim-mode (this setting) from the
            // scrollback `vim_mode` keybindings below.
            key: "simple_mode",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "禁用 Vim 输入模式",
            description: "提示输入使用 readline 风格，而非 Vim 键位。实验性。",
            keywords: &[
                "simple",
                "ascii",
                "minimal",
                "plain",
                "vim",
                "readline",
                "experimental",
                "editor",
                "input",
                "prompt",
            ],
            kind: SettingKind::Bool {
                // `Option<bool>` — `None` treated as `true`.
                default: ui_default.simple_mode.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].vim_mode` in config.toml.
        // Defaults to the same value main's `appearance::persist::VIM_MODE_DEFAULT`
        // shipped with. Bundled next to `simple_mode` because they pair up:
        // simple_mode controls the input editor's vim behaviour,
        // vim_mode controls the scrollback's vim behaviour.
        SettingMeta {
            key: "vim_mode",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "Vim 回滚导航",
            description: "启用 Vim 键（h/j/k/l、gg/G、/）浏览回滚。不影响输入框。",
            keywords: &[
                "vim",
                "scrollback",
                "navigation",
                "hjkl",
                "keys",
                "keybindings",
                "scroll",
            ],
            kind: SettingKind::Bool {
                default: ui_default.vim_mode.unwrap_or(false),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // --- theme + auto themes ---------------------------------------------
        SettingMeta {
            key: "theme",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "主题",
            description: "Pager 界面的配色主题。",
            keywords: &[
                "theme",
                "color",
                "colour",
                "palette",
                "appearance",
                "dark",
                "light",
            ],
            kind: SettingKind::Enum {
                // `Option<String>` — `None` resolved to "groknight".
                default: "groknight",
                choices: THEME_CHOICES,
                supports_preview: true,
            },
            restart_required: false,
            hidden_in_minimal: true,
        },
        SettingMeta {
            key: "auto_dark_theme",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "自动深色主题",
            description: "系统深色模式时使用的主题（仅 theme=auto 时生效）。",
            keywords: &["auto", "dark", "theme", "system", "appearance", "night"],
            kind: SettingKind::Enum {
                // `Option<String>` — `None` falls back to "groknight".
                default: "groknight",
                choices: CONCRETE_THEME_CHOICES,
                supports_preview: true,
            },
            restart_required: false,
            hidden_in_minimal: true,
        },
        SettingMeta {
            key: "auto_light_theme",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "自动浅色主题",
            description: "系统浅色模式时使用的主题（仅 theme=auto 时生效）。",
            keywords: &["auto", "light", "theme", "system", "appearance", "day"],
            kind: SettingKind::Enum {
                // `Option<String>` — `None` falls back to "grokday".
                default: "grokday",
                choices: CONCRETE_THEME_CHOICES,
                supports_preview: true,
            },
            restart_required: false,
            hidden_in_minimal: true,
        },
        // SHELL-owned: persisted to `[ui].render_mermaid`, with a pager-side
        // process-wide cache mirror (like `vim_mode`). Default pinned to "auto"
        // by `defaults_match_ui_config_default`.
        SettingMeta {
            key: "render_mermaid",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "渲染 Mermaid 图表",
            description: "```mermaid 代码块展示方式：auto/on 增加可点击行打开渲染图；off 显示源码。",
            keywords: &[
                "mermaid",
                "diagram",
                "diagrams",
                "render",
                "flowchart",
                "graph",
                "chart",
            ],
            kind: SettingKind::Enum {
                default: "auto",
                choices: RENDER_MERMAID_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // Security-relevant: "always-approve" bypasses all permission prompts.
        // Modal reads live state from `PagerLocalSnapshot.yolo_mode`
        // (not `ui.permission_mode`) to reflect Ctrl+O toggles immediately.
        SettingMeta {
            key: "permission_mode",
            category: SettingCategory::Agent,
            owner: SettingOwner::Shell,
            label: "权限模式",
            description: "Default 使用代理内置行为；Ask 每次工具操作询问；Auto 用模型判断高风险工具；Always approve 自动放行全部权限。",
            keywords: &[
                "permission",
                "approve",
                "yolo",
                "agent",
                "always",
                "ask",
                "auto",
                "classifier",
                "tool",
                "danger",
            ],
            kind: SettingKind::Enum {
                default: "ask",
                choices: PERMISSION_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned `[ui].remember_tool_approvals`. Gates the per-tool
        // "Always allow …" prompt options. `restart_required` — resolved at
        // permission-manager spawn (also fed by env/requirements/managed/remote settings).
        SettingMeta {
            key: "remember_tool_approvals",
            category: SettingCategory::Agent,
            owner: SettingOwner::Shell,
            label: "记住工具授权",
            description: "在权限提示中显示「始终允许」选项，避免同一命令/工具反复询问。适用于 ask 与 auto；Always-approve 仍会跳过全部提示。需重启生效。",
            keywords: &[
                "permission",
                "approve",
                "approval",
                "always",
                "allow",
                "remember",
                "tool",
                "command",
                "kubectl",
                "ask",
                "again",
                "whitelist",
            ],
            kind: SettingKind::Bool {
                default: ui_default.remember_tool_approvals.unwrap_or(false),
            },
            restart_required: true,
            hidden_in_minimal: false,
        },
        // PAGER-owned; default pinned by `defaults_match_pager_state`.
        SettingMeta {
            key: "multiline_mode",
            category: SettingCategory::Editor,
            owner: SettingOwner::Pager,
            label: "多行输入",
            description: "开启后 Enter 换行，Shift+Enter 发送。每个会话重置。",
            keywords: &["multiline", "newline", "input", "editor", "enter"],
            kind: SettingKind::Bool { default: false },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned. Reads from `pager.current_model_name` (not
        // `cfg.models.default`) so the modal reflects `/model` switches.
        // Empty-string default = "no opinion" / use shell's resolution.
        SettingMeta {
            key: "default_model",
            category: SettingCategory::Models,
            owner: SettingOwner::Shell,
            label: "默认模型",
            description: "新会话使用的模型。修改时也会切换当前会话。选择「(不覆盖)」可清除。",
            keywords: &["model", "default", "agent", "llm", "grok", "switch"],
            kind: SettingKind::DynamicEnum {
                default: "",
                source: DynamicEnumSource::ActiveModelCatalog,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHARED. `u16` in UiConfig, widened to `i64` for registry.
        // Width changes apply on the next render frame.
        SettingMeta {
            key: MAX_THOUGHTS_WIDTH_KEY,
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shared,
            label: "思考块最大宽度",
            description: "代理思考面板的列宽预算（40-500，默认 120）。",
            keywords: &[
                "thoughts",
                "width",
                "max",
                "thinking",
                "panel",
                "reasoning",
                "columns",
            ],
            kind: SettingKind::Int {
                default: ui_default.max_thoughts_width as i64,
                min: MAX_THOUGHTS_WIDTH_MIN,
                max: MAX_THOUGHTS_WIDTH_MAX,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui].show_thinking_blocks` + process-wide cache. Default ON.
        SettingMeta {
            key: "show_thinking_blocks",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "显示思考过程",
            description: "流式输出时在回滚中显示思考/推理块。",
            keywords: &[
                "thinking",
                "reasoning",
                "thoughts",
                "blocks",
                "show",
                "hide",
            ],
            kind: SettingKind::Bool {
                default: ui_default.show_thinking_blocks.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui].prompt_suggestions` + process-wide cache. Default ON.
        // The `GROK_PROMPT_SUGGESTIONS` env var overrides at runtime.
        SettingMeta {
            key: "prompt_suggestions",
            category: SettingCategory::Editor,
            owner: SettingOwner::Shell,
            label: "提示建议",
            description: "每轮结束后预测下一条提示，以幽灵文字显示在输入框（Tab 接受）。",
            keywords: &[
                "prompt",
                "suggestion",
                "suggestions",
                "autocomplete",
                "ghost",
                "tab",
                "predict",
                "next",
            ],
            kind: SettingKind::Bool {
                default: ui_default.prompt_suggestions.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // PAGER-owned, persisted to `[scrollback.scroll].respect_manual_folds`
        // in pager.toml (NOT config.toml). Live value is the appearance
        // config (`AppView::set_appearance` fans changes out to every agent);
        // the flag is read at use time, so no restart.
        SettingMeta {
            key: "respect_manual_folds",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Pager,
            label: "保留手动折叠",
            description: "流式输出时保留手动折叠，展开块时停止自动滚动。实验性。",
            keywords: &[
                "fold", "pin", "collapse", "expand", "thinking", "follow", "scroll",
            ],
            kind: SettingKind::Bool {
                default: crate::appearance::ScrollConfig::default().respect_manual_folds,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui].group_tool_verbs` + process-wide cache. Default ON.
        SettingMeta {
            key: "group_tool_verbs",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "合并工具调用",
            description: "将连续的读/搜/列表工具调用与子代理折叠为一行摘要；其间已完成思考一并折叠。",
            keywords: &[
                "group", "tool", "verbs", "fold", "collapse", "read", "search", "summary",
                "thinking", "subagent",
            ],
            kind: SettingKind::Bool {
                default: ui_default.group_tool_verbs.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui].collapsed_edit_blocks` + process-wide cache.
        // Default OFF (rollout flag; remote settings / managed config can enable).
        SettingMeta {
            key: "collapsed_edit_blocks",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "折叠编辑块",
            description: "编辑显示为一行 +N/-M 摘要，并将同一文件连续编辑合并为一行。",
            keywords: &[
                "edit",
                "edits",
                "diff",
                "diffstat",
                "collapse",
                "collapsed",
                "summary",
                "expand",
                "one-line",
                "merge",
                "coalesce",
            ],
            kind: SettingKind::Bool {
                default: ui_default.collapsed_edit_blocks.unwrap_or(false),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui.display_refresh].auto_cadence_enabled`. Restart-
        // required (cadence pinned at startup); hidden in minimal.
        SettingMeta {
            key: "display_refresh_auto_cadence",
            category: SettingCategory::Appearance,
            owner: SettingOwner::Shell,
            label: "匹配显示器刷新率",
            description: "高刷屏上 TUI 以更快节奏流式/滚动。关闭则保持经典节奏。",
            keywords: &[
                "display", "refresh", "rate", "hz", "cadence", "fps", "smooth", "scroll", "stream",
                "high", "120", "144",
            ],
            kind: SettingKind::Bool {
                default: ui_default
                    .display_refresh
                    .auto_cadence_enabled
                    .unwrap_or(false),
            },
            restart_required: true,
            hidden_in_minimal: true,
        },
        // SHELL-owned, persisted to `[ui].scroll_speed` in config.toml.
        SettingMeta {
            key: "scroll_speed",
            category: SettingCategory::Mouse,
            owner: SettingOwner::Shell,
            label: "滚动速度",
            description: "鼠标滚轮与触控板滚动速度倍数（1-100）。越大越快。",
            keywords: &[
                "scroll", "speed", "mouse", "wheel", "trackpad", "fast", "slow",
            ],
            kind: SettingKind::Int {
                default: ui_default.scroll_speed.unwrap_or(50) as i64,
                min: 1,
                max: 100,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned `auto` | `wheel` | `trackpad` on `[ui].scroll_mode`.
        SettingMeta {
            key: "scroll_mode",
            category: SettingCategory::Mouse,
            owner: SettingOwner::Shell,
            label: "滚动输入",
            description: "自动检测不准时，强制按滚轮或触控板行为滚动。",
            keywords: &[
                "scroll", "mode", "wheel", "trackpad", "mouse", "detect", "force", "input",
            ],
            kind: SettingKind::Enum {
                default: ui_default
                    .scroll_mode
                    .as_deref()
                    .and_then(ScrollMode::from_canonical)
                    .unwrap_or_default()
                    .as_canonical(),
                choices: SCROLL_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].scroll_lines`. One knob for BOTH
        // wheel and trackpad lines-per-tick; the registered default 3 matches
        // most terminal profiles, but until the user first commits a value
        // the per-terminal profile stays in charge (cache unset → no override).
        SettingMeta {
            key: "scroll_lines",
            category: SettingCategory::Mouse,
            owner: SettingOwner::Shell,
            label: "滚动行数",
            description: "滚轮与触控板每次滚动的行数（1-10）。未设置前使用终端自身配置。",
            keywords: &[
                "scroll", "lines", "tick", "notch", "wheel", "trackpad", "mouse",
            ],
            kind: SettingKind::Int {
                default: ui_default.scroll_lines.map(i64::from).unwrap_or(3),
                min: 1,
                max: 10,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned: `[ui].invert_scroll` + process-wide cache. Default OFF.
        SettingMeta {
            key: "invert_scroll",
            category: SettingCategory::Mouse,
            owner: SettingOwner::Shell,
            label: "反转滚动",
            description: "反转纵向滚动方向（自然滚动）。",
            keywords: &[
                "invert",
                "scroll",
                "natural",
                "direction",
                "reverse",
                "mouse",
                "trackpad",
            ],
            kind: SettingKind::Bool {
                default: ui_default.invert_scroll.unwrap_or(false),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned `flash` | `hold` on `[ui].keep_text_selection`.
        SettingMeta {
            key: "keep_text_selection",
            category: SettingCategory::Mouse,
            owner: SettingOwner::Shell,
            label: "文本选择",
            description: "应用内选区保持时长，以及双击行为（折叠 vs 选词并复制）。",
            keywords: &[
                "selection",
                "drag",
                "copy",
                "flash",
                "hold",
                "shift",
                "native",
                "mouse",
                "tmux",
                "double",
                "double-click",
                "word",
                "terminal",
            ],
            kind: SettingKind::Enum {
                default: TextSelection::Flash.as_canonical(),
                choices: TEXT_SELECTION_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned. Persisted in auth metadata (not config.toml).
        // Reads from `PagerLocalSnapshot.coding_data_sharing_opt_out`.
        // Default "opt-out" matches `AuthEntry::coding_data_retention_opt_out = true`
        // (safer consumer default; server enrichment may still opt the user in).
        // ZDR / non-admin guards are enforced at dispatch time.
        // Do not put "telemetry" in keywords — that word is the config-file
        // analytics toggle (Monitoring / Configuration docs).
        SettingMeta {
            key: "coding_data_sharing",
            category: SettingCategory::Privacy,
            owner: SettingOwner::Shell,
            label: "编程数据共享",
            description: "控制 SpaceXAI 是否可保留并基于编程会话数据训练。不影响产品分析。",
            keywords: &[
                "privacy",
                "data",
                "sharing",
                "coding",
                "retention",
                "training",
                "opt-in",
                "opt-out",
            ],
            kind: SettingKind::Enum {
                default: "opt-out",
                choices: CODING_DATA_SHARING_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].default_selected_permission` in
        // config.toml. Read by the pager via `appearance::permission_cursor`.
        // Canonical `always_allow_all_sessions` (the effective default) lands
        // the first prompt's cursor on the enable-always-approve row;
        // subsequent prompts stick to the last-used kind.
        SettingMeta {
            key: "default_selected_permission",
            category: SettingCategory::Agent,
            owner: SettingOwner::Shell,
            label: "默认选中的权限项",
            description: "权限提示中光标默认选中哪一行。",
            keywords: &[
                "permission",
                "approval",
                "cursor",
                "preselect",
                "default",
                "sticky",
                "last",
                "used",
                "yes",
                "no",
                "reject",
                "allow",
            ],
            kind: SettingKind::Enum {
                default: DefaultSelectedPermission::AlwaysAllowAllSessions.as_canonical(),
                choices: DEFAULT_SELECTED_PERMISSION_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned `[toolset.ask_user_question].timeout_enabled`. Surfaces
        // the user-config layer of the tiered timeout gate (requirements/env/
        // managed/remote settings feed the effective value at agent build); the
        // default is the resolver-shared const. `restart_required` — resolved
        // when an agent is built, like `remember_tool_approvals`.
        SettingMeta {
            key: "toolset.ask_user_question.timeout_enabled",
            category: SettingCategory::Agent,
            owner: SettingOwner::Shell,
            label: "提问超时",
            description: "开启后，ask_user_question 工具会在设定时间后超时，而非无限阻塞。",
            keywords: &[
                "ask",
                "question",
                "questionnaire",
                "timeout",
                "ask_user_question",
                "block",
                "wait",
                "forever",
                "tool",
            ],
            kind: SettingKind::Bool {
                default: ask_user_question::DEFAULT_ASK_USER_QUESTION_TIMEOUT_ENABLED,
            },
            restart_required: true,
            hidden_in_minimal: false,
        },
        // PAGER-owned, ACP-mediated. Reads from
        // `PagerLocalSnapshot.plan_mode_active`. Default "off" matches
        // `AgentView::new`'s `plan_mode_active = false`.
        SettingMeta {
            key: "plan_mode",
            category: SettingCategory::Agent,
            owner: SettingOwner::Pager,
            label: "计划模式",
            description: "开启后，代理在运行工具或编辑前先总结计划。",
            keywords: &[
                "plan", "mode", "agent", "summary", "approval", "review", "session",
            ],
            kind: SettingKind::Enum {
                default: "off",
                choices: PLAN_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned startup-time settings (restart_required: true).
        // The running pager doesn't re-read these mid-session.
        SettingMeta {
            key: "show_tips",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "显示提示",
            description: "启动时显示每日提示横幅。需重启。",
            keywords: &[
                "tips", "tip", "show", "banner", "welcome", "startup", "launch",
            ],
            kind: SettingKind::Bool { default: true },
            restart_required: true,
            hidden_in_minimal: false,
        },
        // Contextual hints: one Advanced row that opens a sub-sheet of per-tip
        // toggles. Applies live (restart_required: false); the group carries no
        // value and its children are hidden from the top-level list.
        SettingMeta {
            key: "contextual_hints",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "显示上下文快捷键提示",
            description: "工作时显示简短上下文快捷键提示；可单独开关。",
            keywords: &[
                "contextual",
                "hints",
                "tips",
                "undo",
                "plan",
                "nudge",
                "image",
                "clipboard",
                "ephemeral",
                "send",
                "interject",
                "queue",
                // Child-specific terms: the per-tip children are hidden from the
                // top-level list, so mirror their search words here to keep a
                // query like "ctrl+z" or "shift+tab" from dead-ending.
                "ctrl+z",
                "draft",
                "wipe",
                "mode",
                "shift+tab",
                "paste",
                "input",
                "enter",
                "follow-up",
                "small",
                "screen",
                "compact",
                "ssh",
                "wrap",
                "remote",
            ],
            kind: SettingKind::Group {
                children: CONTEXTUAL_HINTS_CHILDREN,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "auto_update",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "自动更新",
            description: "启动时自动下载并安装 pager 更新。需重启。",
            keywords: &[
                "auto", "update", "updates", "upgrade", "version", "install", "channel",
            ],
            kind: SettingKind::Bool { default: true },
            restart_required: true,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].hunk_tracker_mode`. Restart-required:
        // the mode is read once when the session connects.
        SettingMeta {
            key: "hunk_tracker_mode",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "变更块跟踪",
            description: "代理将哪些文件变更跟踪为 hunk。Off 完全关闭跟踪（及 LOC 统计）。需重启。",
            keywords: &[
                "hunk", "tracker", "tracking", "diff", "changes", "git", "loc", "off", "disable",
            ],
            kind: SettingKind::Enum {
                default: "agent_only",
                choices: HUNK_TRACKER_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: true,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].voice_capture_mode`. The `hold` choice
        // is hidden on terminals without key-release reporting (see
        // `effective_enum_choices`) and falls back to `toggle` at runtime.
        SettingMeta {
            key: "voice_capture_mode",
            category: SettingCategory::Editor,
            owner: SettingOwner::Shell,
            label: "语音采集",
            description: "语音快捷键（Ctrl+Space / F8）行为：切换（再按停止）或按住说话（松开结束）。",
            keywords: &[
                "voice",
                "dictation",
                "dictate",
                "mic",
                "microphone",
                "speech",
                "stt",
                "toggle",
                "hold",
                "ctrl+space",
                "f8",
                "push-to-talk",
            ],
            kind: SettingKind::Enum {
                default: "hold",
                choices: VOICE_CAPTURE_MODE_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // SHELL-owned, persisted to `[ui].voice_stt_language`. Live-applied to
        // the next voice capture (no restart). Default English; System (`auto`)
        // follows the process locale when it maps to a Grok STT language.
        // Catalog = official STT languages (see xai_grok_voice::STT_LANGUAGES).
        SettingMeta {
            key: "voice_stt_language",
            category: SettingCategory::Editor,
            owner: SettingOwner::Shell,
            label: "语音语言",
            description: "语音听写的转写语言（Grok STT）。默认英语；System 在支持时使用系统语言。",
            keywords: &["voice", "language", "locale", "dictation", "stt", "speech"],
            kind: SettingKind::Enum {
                default: "en",
                choices: VOICE_STT_LANGUAGE_CHOICES,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // Contextual-hint children (hidden from the top-level list; reached via
        // the group sub-sheet). Default ON — `None` (inherit) reads as `true`.
        SettingMeta {
            key: "contextual_hints.undo",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "撤销",
            description: "清空提示后提醒可用 Ctrl+Z 恢复。",
            keywords: &["undo", "ctrl+z", "draft", "wipe", "hint"],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.undo.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.plan_mode",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "计划模式",
            description: "当提示像规划请求时，建议进入计划模式（Shift+Tab）。",
            keywords: &["plan", "mode", "nudge", "shift+tab", "hint"],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.plan_mode.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.image_input",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "图片输入",
            description: "剪贴板有图片且模型支持时，提示粘贴图片。",
            keywords: &["image", "clipboard", "paste", "input", "hint"],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.image_input.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.send_now",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "立即发送",
            description: "中途排队跟进后，提醒空提示框按 Enter 会发送队列首项。",
            keywords: &[
                "send",
                "now",
                "interject",
                "queue",
                "follow-up",
                "enter",
                "empty",
                "hint",
            ],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.send_now.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.small_screen",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "小屏幕",
            description: "终端行数不足时，每次运行提示一次 /compact-mode。",
            keywords: &["small", "screen", "compact", "space", "rows", "hint"],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.small_screen.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.word_select",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "按词选择",
            description: "在折叠/导航选择模式下双击对话文本后，提醒词选模式可复制单词。",
            keywords: &[
                "word",
                "select",
                "double",
                "double-click",
                "click",
                "fold",
                "selection",
                "settings",
                "hint",
            ],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.word_select.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        SettingMeta {
            key: "contextual_hints.ssh_wrap",
            category: SettingCategory::Advanced,
            owner: SettingOwner::Shell,
            label: "SSH 换行",
            description: "SSH 加载会话时，推荐 `grok wrap ssh` 以转发剪贴板并恢复终端。",
            keywords: &[
                "ssh",
                "wrap",
                "remote",
                "clipboard",
                "restore",
                "startup",
                "hint",
            ],
            kind: SettingKind::Bool {
                default: ui_default.contextual_hints.ssh_wrap.unwrap_or(true),
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
        // ── TodoGate (runtime turn-end backstop) ──────────────────────
        //
        // Only the CLI flag (`--todo-gate`) is wired. Settings-modal
        // entries for `[reminder.todo_gate]` are deferred — the modal
        // dispatcher requires per-key action arms in
        // `settings_modal.rs` + `app/dispatch.rs` + `settings/registry.rs`
        // that don't yet have a place to land.
        // SHELL-owned. `restart_required: false` — the config-reloader
        // rebroadcasts UI changes; mid-session forks pick up new values.
        // Empty-string default = "no opinion" / use shell's resolution.
        SettingMeta {
            key: "fork_secondary_model",
            category: SettingCategory::Models,
            owner: SettingOwner::Shell,
            label: "分叉副模型",
            description: "分叉时副代理使用的模型。选「(不覆盖)」可清除。",
            keywords: &[
                "fork",
                "secondary",
                "model",
                "agent",
                "subagent",
                "branch",
                "models",
            ],
            kind: SettingKind::DynamicEnum {
                default: "",
                source: DynamicEnumSource::ActiveModelCatalog,
                supports_preview: false,
            },
            restart_required: false,
            hidden_in_minimal: false,
        },
    ]
}
