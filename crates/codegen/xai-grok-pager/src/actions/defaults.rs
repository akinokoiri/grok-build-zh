//! Default action definitions for the MVP.
//!
//! All key bindings are defined here — not scattered across event handlers.

use crate::key;
use crate::terminal::{TerminalName, terminal_context};

use super::{ActionDef, ActionId, Category, When};

/// True when `Ctrl+.` is not a reliable shortcuts-cheatsheet primary.
///
/// Callers pick a deliverable alternate primary (`Ctrl+X` on the agent
/// screen, `?` on the dashboard). Both keys stay registered either way;
/// this only chooses which the UI advertises.
///
/// Driven by [`crate::terminal::TerminalContext::ctrl_dot_unreliable`]
/// (any KKP skip — brand, tmux `extended-keys off`, screen, unknown host),
/// plus host-OS signals: native Windows on a non-branded console, or a
/// Linux binary inside Win32's console pipeline (WSL).
pub fn ctrl_dot_unreliable() -> bool {
    terminal_context().ctrl_dot_unreliable() || cfg!(target_os = "windows") || crate::host::is_wsl()
}

/// Build the default action definitions.
///
/// `mouse_reporting_toggle_enabled` gates the opt-in `ToggleMouseCapture`
/// shortcut (see below); pass `false` for the standard set.
pub fn default_actions(mouse_reporting_toggle_enabled: bool) -> Vec<ActionDef> {
    let ctx = terminal_context();
    // xterm.js embeds: no KKP; host often steals Ctrl+I. Share one family flag for
    // quit / half-page / interject so VS Code-family embeds match VS Code.
    let in_vscode_family = ctx.brand.is_vscode_family();
    let in_vscode = in_vscode_family;
    let in_apple_terminal = ctx.brand == TerminalName::AppleTerminal;
    let ctrl_dot_unreliable = ctrl_dot_unreliable();

    let mut actions = vec![
        // ── Navigation (scrollback) ─────────────────────────────────
        ActionDef {
            id: ActionId::SelectNext,
            label: "导航",
            description: "选择下一项",
            default_key: key!('j'),
            alt_keys: vec![key!(Down)],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: Some(0),
            hint_key_display: Some("j/k"),
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::SelectPrev,
            label: "导航",
            description: "选择上一项",
            default_key: key!('k'),
            alt_keys: vec![key!(Up)],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::NextTurn,
            label: "回合",
            description: "下一回合",
            default_key: key!('L'),
            alt_keys: vec![key!(Right, SHIFT)],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: Some(1),
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::PrevTurn,
            label: "回合",
            description: "上一回合",
            default_key: key!('H'),
            alt_keys: vec![key!(Left, SHIFT)],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::NextResponse,
            label: "回复",
            description: "下一条回复",
            default_key: key!('J'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::PrevResponse,
            label: "回复",
            description: "上一条回复",
            default_key: key!('K'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::GotoTop,
            label: "顶/底",
            description: "跳到顶部",
            default_key: key!('g'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: Some(4),
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::GotoBottom,
            label: "底部",
            description: "跳到底部",
            default_key: key!('G'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::ScrollUp,
            label: "上滚",
            description: "向上滚动一行",
            default_key: key!('k', CONTROL),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::ScrollDown,
            label: "下滚",
            description: "向下滚动一行",
            default_key: key!('j', CONTROL),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::HalfPageUp,
            label: "上半页",
            description: "向上半页",
            default_key: key!('u', CONTROL),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::HalfPageDown,
            label: "下半页",
            description: "向下半页",
            default_key: if in_vscode {
                key!('D')
            } else {
                key!('d', CONTROL)
            },
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::PageUp,
            label: "上页",
            description: "向上一页",
            default_key: key!(PageUp),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::PageDown,
            label: "下页",
            description: "向下一页",
            default_key: key!(PageDown),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        // ── View (scrollback) ───────────────────────────────────────
        ActionDef {
            id: ActionId::Collapse,
            label: "折叠",
            description: "折叠选中项",
            default_key: key!('h'),
            alt_keys: vec![key!(Left)],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::Expand,
            label: "折叠",
            description: "展开选中项",
            default_key: key!('l'),
            alt_keys: vec![key!(Right)],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::ToggleFold,
            label: "折叠",
            description: "展开/折叠",
            default_key: key!('e'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: Some(3),
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "折叠或展开当前选中的回滚条目，隐藏或显示完整正文。\n便于快速浏览冗长的工具输出或推理内容。\n相关：E 会折叠/展开全部条目；Ctrl+E 切换全部思考块。",
            ),
        },
        ActionDef {
            id: ActionId::ToggleExpandAll,
            label: "全部",
            description: "全部展开/折叠",
            default_key: key!('E'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "一次性折叠或展开全部回滚条目；小写 e 只切换当前选中行。\n可先收起长对话只看标题，再全部展开。\n思考块另有开关：Ctrl+E。",
            ),
        },
        ActionDef {
            id: ActionId::ExpandAllThinking,
            label: "展开/折叠思考",
            description: "切换全部思考块",
            default_key: key!('e', CONTROL),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: Some(3),
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "一键显示或隐藏整段对话中的思考（推理）块。\n可查看代理如何得出结论，或隐藏推理只看结果。\n与 E 不同：E 会折叠所有类型的条目。",
            ),
        },
        ActionDef {
            id: ActionId::ToggleRaw,
            label: "原始",
            description: "切换原始 Markdown",
            default_key: key!('r'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "在选中条目的「渲染 Markdown」与「原始源码」之间切换。\n便于复制原始 Markdown、查看链接目标，或查看渲染时隐藏的格式。\n再按一次回到渲染视图。",
            ),
        },
        // ── Block content ────────────────────────────────────────────
        ActionDef {
            id: ActionId::CopyBlockContent,
            label: "复制",
            description: "复制内容",
            default_key: key!('y'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None, // shown dynamically when block supports copy
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "将选中块的正文复制到剪贴板：消息文本、完整工具输出或代码块内容。\n仅在支持复制的块上可用。\n若只要命令行或文件路径，请用大写 Y。",
            ),
        },
        ActionDef {
            id: ActionId::CopyBlockMeta,
            label: "复制命令",
            description: "复制命令/路径",
            default_key: key!('Y'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "只复制块的标识：工具调用的命令行或文件块路径，不含正文。\n便于重跑命令或把路径粘贴到别处。\n复制完整内容请用小写 y。",
            ),
        },
        ActionDef {
            id: ActionId::OpenBlockViewer,
            label: "查看",
            description: "在查看器中打开",
            default_key: key!(Enter),
            alt_keys: vec![key!('f', CONTROL)],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "在可滚动的全屏查看器中打开选中块。\n适合阅读长工具输出、大文件，或希望与周围对话分开看的代码。\nEsc 返回对话。",
            ),
        },
        // ── Link navigation ─────────────────────────────────────────
        ActionDef {
            id: ActionId::OpenNextLink,
            label: "链接",
            description: "下一链接",
            default_key: key!('o'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::OpenPrevLink,
            label: "链接",
            description: "上一链接",
            default_key: key!('O'),
            alt_keys: vec![],
            category: Category::ConversationNav,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        // ── Scrollback (contextual — block-type-dependent) ────────────
        ActionDef {
            id: ActionId::Rewind,
            label: "回退",
            description: "回退到选中回合",
            default_key: key!(Null),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "将对话回退到更早的回合，恢复当时的文件快照并丢弃之后的更改。\n从列表中选中回合并选择恢复范围（全部 / 仅对话 / 仅文件）；若有进行中的回合会先提示取消，冲突或错误会在执行后报告。\n破坏性操作：之后的回合会被丢弃。\n空闲且输入为空时也可用 Esc Esc（800ms 内）触发，与 `/rewind` 相同。",
            ),
        },
        ActionDef {
            id: ActionId::KillBgTask,
            label: "终止",
            description: "终止后台任务",
            default_key: key!('x'),
            alt_keys: vec![],
            category: Category::ConversationAction,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "终止选中任务块对应的后台任务（例如已转入后台的长 shell 命令）。\n用于停下失控或不再需要的进程。\n仅对仍在运行的任务生效；已结束的不受影响。",
            ),
        },
        // ── Essentials ────────────────────────────────────────────────
        ActionDef {
            id: ActionId::SendPrompt,
            label: "发送",
            description: "发送",
            default_key: key!(Enter),
            alt_keys: vec![],
            category: Category::GettingStarted,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::FocusPrompt,
            label: "输入",
            description: "聚焦输入框",
            default_key: key!(Tab),
            alt_keys: vec![key!('i'), key!(' ')],
            category: Category::GettingStarted,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::FocusScrollback,
            label: "回滚",
            description: "聚焦回滚区",
            default_key: key!(Tab),
            alt_keys: vec![],
            category: Category::GettingStarted,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "把焦点从输入框移到回滚区，以便浏览对话记录。\n简单模式与 vim 回滚模式下都可用 Tab。\nEsc 保留给清空/回退（空闲策略），不用于切换焦点。",
            ),
        },
        ActionDef {
            id: ActionId::CancelTurn,
            label: "取消",
            description: "取消当前回合",
            default_key: key!('c', CONTROL),
            alt_keys: vec![],
            category: Category::GettingStarted,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "中断代理当前回合并停止生成，会话保持打开。\n输入为空时 Ctrl+C 取消回合；有草稿时先清空输入，回合继续运行。\n只停当前回合，不退出应用；退出请用退出快捷键。",
            ),
        },
        ActionDef {
            id: ActionId::CycleMode,
            label: "模式",
            description: "循环模式（普通/计划/始终批准）",
            // All Shift+Tab encodings — see `input::key::shift_tab_keys()`.
            default_key: crate::input::key::shift_tab_keys()[0],
            alt_keys: crate::input::key::shift_tab_keys()[1..].to_vec(),
            category: Category::GettingStarted,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: Some("Shift+Tab"),
            requires_confirmation: false,
            long_help: Some(
                "循环会话模式：普通 → 计划 → 始终批准 → 普通。\n计划模式先规划、不写文件；始终批准对每次工具调用不再询问。\nCtrl+O 可直接切换自动批准。",
            ),
        },
        // ── Panes (agent-level — toggle side panes) ─────────────────
        ActionDef {
            id: ActionId::ToggleTodos,
            label: "待办",
            description: "切换待办面板",
            default_key: key!('t', CONTROL),
            alt_keys: vec![],
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "显示或隐藏待办面板：当前工作的实时任务清单。\n可在回合进行中查看计划与剩余事项。\n侧栏面板；关掉可腾出宽度。",
            ),
        },
        ActionDef {
            id: ActionId::ToggleTasks,
            label: "任务",
            description: "切换任务面板",
            default_key: key!('b', CONTROL),
            alt_keys: vec![],
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "显示或隐藏任务面板，列出后台任务及其状态。\n用于监控或回到用 Ctrl+G 转入后台的工作。\n侧栏面板；关掉可腾出宽度。",
            ),
        },
        ActionDef {
            id: ActionId::ToggleQueue,
            label: "队列",
            description: "切换提示队列",
            // Local macOS VS Code family only: ; / ' often never arrive (saw
            // Ctrl+4 in input-debug). SSH and non-Mac keep ; (+ ' alt). Win/Linux
            // VS maps Ctrl+4 to focusFourthEditorGroup.
            default_key: if in_vscode_family && !ctx.is_ssh && cfg!(target_os = "macos") {
                key!('4', CONTROL)
            } else {
                key!(';', CONTROL)
            },
            // Apostrophe alt for consoles that drop Ctrl on `;`. Local Mac VS
            // also keeps ; / ' as alts alongside primary Ctrl+4.
            alt_keys: if in_vscode_family && !ctx.is_ssh && cfg!(target_os = "macos") {
                vec![key!(';', CONTROL), key!('\'', CONTROL)]
            } else {
                vec![key!('\'', CONTROL)]
            },
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "显示或隐藏提示队列。\n回合进行中可预先排好后续提示，代理结束后会自动依次发送。\n本机 macOS VS Code 系：主快捷键 Ctrl+4（备选 Ctrl+; / Ctrl+'）。其他环境：Ctrl+;，备选 Ctrl+'。",
            ),
        },
        ActionDef {
            id: ActionId::OpenSessions,
            label: "会话",
            description: "打开会话列表",
            default_key: key!('s', CONTROL),
            alt_keys: vec![],
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "打开会话浏览器，恢复或切换历史对话。\n选中一项即可重新接入完整历史。\n与代理仪表盘（Ctrl+\\）不同：仪表盘用于同时管理多个在线代理。",
            ),
        },
        ActionDef {
            id: ActionId::OpenExtensions,
            label: "扩展",
            description: "打开扩展",
            // VS Code family: Ctrl+L is interject; plugins via /plugins (no chord here).
            default_key: if in_vscode_family {
                key!(Null)
            } else {
                key!('l', CONTROL)
            },
            alt_keys: vec![],
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "打开扩展管理：查看已连接的 MCP 服务器与插件及其提供的工具。\n可用于确认集成是否加载，或浏览可用工具。\n与「设置」不同：设置是通用应用选项。",
            ),
        },
        ActionDef {
            id: ActionId::SendToBackground,
            label: "转后台",
            description: "将运行中任务转入后台",
            default_key: key!('g', CONTROL),
            alt_keys: vec![],
            category: Category::Panels,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "将正在运行的回合转入后台，可继续阅读、排队提示或做别的事。\n在任务面板（Ctrl+B）中跟踪与恢复。\n仅在回合实际运行时有意义。",
            ),
        },
        // ── Prompt ───────────────────────────────────────────────────
        ActionDef {
            id: ActionId::InterjectPrompt,
            // "send now" label: Enter queues a follow-up while a turn runs;
            // this chord is cancel-and-send — stop the current turn and run
            // the message as the next one ("send now").
            label: "立即发送",
            description: "运行中立即发送（会取消当前回合）",
            default_key: if in_apple_terminal {
                key!('o', CONTROL)
            } else if in_vscode_family {
                // Ctrl+L is a stable C0 form feed on xterm.js; see user-guide § interject.
                key!('l', CONTROL)
            } else {
                key!(Enter, CONTROL)
            },
            // Windows: Ctrl+Enter may drop Ctrl → Ctrl+I alt. VS Code family: no alts
            // (Ctrl+L sole chord; OpenExtensions unbound so it does not steal).
            alt_keys: if in_apple_terminal {
                vec![key!(Enter, CONTROL), key!('i', CONTROL)]
            } else if in_vscode_family {
                vec![]
            } else {
                vec![key!('i', CONTROL)]
            },
            category: Category::Input,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "在回合进行中向代理插入消息（插话），不取消当前回合，便于边跑边纠偏或补充上下文。\n回合运行时普通 Enter 会排队后续提示；此快捷键则把输入区内容并入当前回合。\n输入为空时，单独 Enter（或此快捷键）会从输入区强制发送队列顶部的后续项，无需聚焦队列面板；在队列面板上则强制发送当前选中行。\n适合在不丢弃当前进度的情况下改道。",
            ),
        },
        ActionDef {
            id: ActionId::EnableVoiceMode,
            label: "语音模式",
            description: "开始语音听写（Ctrl+Space / F8）",
            // No key binding (`KeyCode::Null`): dispatched directly by the voice
            // chord's hold-to-talk press in the event loop, not via the registry.
            default_key: key!(Null),
            alt_keys: vec![],
            category: Category::Input,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            // Voice capture chord (same surface as `/voice`; Esc/Enter stop).
            // Bound to BOTH Ctrl+Space and F8 — Ctrl+Space decodes on every
            // terminal (without the Kitty protocol it collapses to NUL, reported
            // as `Char(' ')`+CONTROL), and F8 is a fallback for OSes/terminals
            // that intercept Ctrl+Space (e.g. macOS input-source switching; use
            // Fn+F8 on a laptop). The event loop maps a press to hold-to-talk or
            // tap-toggle per `[ui].voice_capture_mode` before normal routing.
            id: ActionId::VoiceToggle,
            label: "麦克风",
            description: "语音听写（Ctrl+Space / F8）",
            default_key: key!(' ', CONTROL),
            alt_keys: vec![key!(F(8))],
            category: Category::Input,
            // `Always` so the toggle key works on the agent screen AND the
            // session-less dashboard (resolved via the global fallthrough).
            context: When::Always,
            hint_priority: Some(11),
            hint_key_display: Some("Ctrl+Space / F8"),
            requires_confirmation: false,
            long_help: Some(
                "麦克风听写：绑定 Ctrl+Space（或 F8——当 Ctrl+Space 被占用时很有用，例如 macOS 输入法切换；笔记本可用 Fn+F8）。\n行为跟随「语音捕获」设置：切换模式（按一下开始、再按停止）或按住说话（按住录音、松开停止）；按住模式需要 Kitty 协议终端，否则回退为切换。`/voice` 可在各处切换。\n语音会直接转写进输入框。",
            ),
        },
        // Prompt history has no key chord (Ctrl+R is deliberately unbound):
        // `/history` opens the search panel; Up on an empty prompt browses.
        ActionDef {
            id: ActionId::ToggleMultiline,
            label: "多行",
            description: "切换多行输入",
            default_key: key!('m', CONTROL),
            alt_keys: vec![],
            category: Category::Input,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "切换持久多行输入，便于撰写较长消息。\n用 Shift+Enter 或 Alt+Enter（或行尾反斜杠）换行；单独 Enter 仍发送。\n在输入框内 Ctrl+M 切换多行；焦点不在输入框时则打开模型选择器。",
            ),
        },
        ActionDef {
            id: ActionId::BashMode,
            label: "Shell 模式",
            description: "Shell 模式（空输入时键入 !）",
            default_key: key!('!'),
            alt_keys: vec![],
            category: Category::Input,
            context: When::PromptFocused,
            hint_priority: None,
            hint_key_display: Some("!"),
            requires_confirmation: false,
            long_help: Some(
                "不离开对话即可运行 shell 命令：在空输入框以 ! 开头再输入命令。\n命令输出会写入回滚区。\n删掉开头的 ! 即回到普通提示输入。",
            ),
        },
        // ── Agent ────────────────────────────────────────────────────
        ActionDef {
            id: ActionId::ToggleYolo,
            label: "始终批准",
            description: "切换始终批准",
            default_key: key!('o', CONTROL),
            alt_keys: vec![],
            category: Category::Session,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "为本会话打开或关闭自动批准（YOLO）。\n开启后，代理的每次工具调用（编辑、shell、删除等）不再逐项确认。\n与 Shift+Tab 循环中的「始终批准」同一状态；请谨慎使用。",
            ),
        },
        ActionDef {
            id: ActionId::NewSession,
            label: "新建",
            description: "新会话",
            default_key: key!('n', CONTROL),
            alt_keys: vec![],
            category: Category::Session,
            context: When::Always,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: true,
            long_help: Some(
                "开启全新会话，回滚区与上下文为空。\n需要确认：连按两次（第一次预备，第二次真正开始），避免误丢当前对话。",
            ),
        },
        ActionDef {
            id: ActionId::Quit,
            label: "退出",
            description: "退出",
            default_key: if in_vscode {
                key!('d', CONTROL)
            } else {
                key!('q', CONTROL)
            },
            alt_keys: if in_vscode {
                vec![]
            } else {
                vec![key!('d', CONTROL)]
            },
            category: Category::GettingStarted,
            context: When::Always,
            hint_priority: Some(10),
            hint_key_display: None,
            requires_confirmation: true,
            long_help: Some(
                "退出应用。需要确认：短时间内连按两次；单独一次按键会被当作误触忽略。\n绑定 Ctrl+Q，Ctrl+D 为别名（在 VS Code 终端中 Ctrl+D 为主键）。",
            ),
        },
        ActionDef {
            id: ActionId::CommandPalette,
            label: "命令",
            description: "命令面板",
            default_key: key!('p', CONTROL),
            alt_keys: vec![key!('?')],
            category: Category::GettingStarted,
            context: When::AgentScreen,
            hint_priority: Some(5),
            hint_key_display: Some("?"),
            requires_confirmation: false,
            long_help: Some(
                "模糊搜索全部操作与斜杠命令，按名称运行。\n记不住快捷键时很有用。\n焦点在回滚区时也可用 ? 打开。",
            ),
        },
        ActionDef {
            id: ActionId::ShortcutsHelp,
            label: "快捷键",
            description: "键盘快捷键",
            default_key: if ctrl_dot_unreliable {
                key!('x', CONTROL)
            } else {
                key!('.', CONTROL)
            },
            alt_keys: vec![if ctrl_dot_unreliable {
                key!('.', CONTROL)
            } else {
                key!('x', CONTROL)
            }],
            category: Category::GettingStarted,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "打开本键盘快捷键速查表。\n用 j/k 浏览，e 展开行内说明，Enter 打开某快捷键的完整详情页。\n同时绑定 Ctrl+. 与 Ctrl+X；底栏会显示你终端能稳定送达的那一个。",
            ),
        },
        ActionDef {
            id: ActionId::ModelPicker,
            label: "模型",
            description: "选择模型",
            default_key: key!('m', CONTROL),
            alt_keys: vec![],
            category: Category::Session,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "打开模型选择器，切换本会话所用模型；选择对之后的回合生效。\n绑定 Ctrl+M，但焦点在输入框时该组合键会改为切换多行。\n可从回滚区或命令面板进入。",
            ),
        },
        ActionDef {
            id: ActionId::OpenSettings,
            label: "设置",
            description: "打开设置",
            default_key: key!(F(2)),
            alt_keys: vec![key!(',', CONTROL), key!(',', SUPER)],
            category: Category::GettingStarted,
            context: When::AgentScreen,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
    ];

    // Toggle terminal mouse reporting (mouse capture). Opt-in via
    // `[ui] mouse_reporting_toggle = true` in config.toml. Disabling capture
    // hands mouse selection back to the terminal for native click-drag
    // copy/paste; re-enabling restores in-app mouse support.
    //
    // Single binding: Ctrl+R on scrollback only (not prompt — Ctrl+R there
    // remains prompt history search). Plain Ctrl+letter passes through Apple
    // Terminal; avoids Ctrl+Shift+… chords that Terminal.app often swallows.
    // Under Panels (not Essentials) — advanced/opt-in only.
    if mouse_reporting_toggle_enabled {
        actions.push(ActionDef {
            id: ActionId::ToggleMouseCapture,
            label: "鼠标上报",
            description: "切换鼠标上报（原生复制粘贴）",
            default_key: key!('r', CONTROL),
            alt_keys: vec![],
            category: Category::Panels,
            context: When::ScrollbackFocused,
            hint_priority: None,
            hint_key_display: Some("Ctrl+r"),
            requires_confirmation: false,
            long_help: None,
        });
    }

    // Agent Dashboard ----------------------------------------------------
    //
    // The `Ctrl+\` entry point AND every in-dashboard shortcut are registered
    // here. They all share the dedicated `Category::Dashboard` section so the
    // cheatsheet groups them under a single "Dashboard" header instead of
    // scattering them through Panels / Session / Navigation.
    //
    // `Ctrl+\` (OpenDashboard) is registered against `Always` (global) so it
    // works from any view — welcome, agent, or dashboard itself (which Esc
    // closes). Configurable through the standard config.toml mechanism.
    actions.extend([
        ActionDef {
            id: ActionId::OpenDashboard,
            label: "仪表盘",
            description: "打开代理仪表盘",
            default_key: key!('\\', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::Always,
            hint_priority: None,
            hint_key_display: Some("Ctrl+\\"),
            requires_confirmation: false,
            long_help: Some(
                "打开代理仪表盘：列出全部运行中与最近的代理，便于监视与切换。\n可在欢迎页或会话内随时打开。\n在此可派发、接入、停止、分组与重排代理。",
            ),
        },
        // Register all in-dashboard shortcuts through
        // the registry under `When::DashboardFocused`. The dispatch
        // path in `dashboard::state::handle_key` looks these up via
        // `registry.lookup(key, When::DashboardFocused)` so users can
        // rebind any of them through `~/.grok/config.toml`.
        ActionDef {
            id: ActionId::DashboardSelectNext,
            label: "下一个",
            description: "选择下一行",
            default_key: key!(Down),
            alt_keys: vec![key!('j')],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("\u{2191}\u{2193}"),
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardSelectPrev,
            label: "上一个",
            description: "选择上一行",
            default_key: key!(Up),
            alt_keys: vec![key!('k')],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardTogglePin,
            label: "固定",
            description: "固定/取消固定代理",
            default_key: key!('t', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "固定或取消固定选中代理，使其不受排序/分组影响、始终靠前显示。\n便于在其他代理进出列表时仍盯住关心的项。\n固定状态会跨仪表盘会话保留。",
            ),
        },
        ActionDef {
            id: ActionId::DashboardBeginRename,
            label: "重命名",
            description: "重命名代理",
            default_key: key!('r', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardStop,
            label: "停止",
            description: "停止/关闭代理",
            default_key: key!('x', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "停止选中代理并从仪表盘移除该行；若有进行中的回合会先中断。\n用于清理已完成或不需要的代理，无需先接入。\n浮层内对应操作（Ctrl+X）停止前会要求确认。",
            ),
        },
        ActionDef {
            id: ActionId::DashboardCycleMode,
            label: "模式",
            description: "循环派发模式",
            // All Shift+Tab encodings — see `input::key::shift_tab_keys()`.
            // Registry `matches` is exact-modifier, so the SHIFT-bearing
            // forms must be alts.
            default_key: crate::input::key::shift_tab_keys()[0],
            alt_keys: crate::input::key::shift_tab_keys()[1..].to_vec(),
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Shift+Tab"),
            requires_confirmation: false,
            long_help: Some(
                "循环仪表盘派发新代理时的模式：普通 → 计划 → 始终批准。\n计划模式会先规划再改文件；始终批准运行工具时不再询问。\n与会话内 Shift+Tab 循环一致，作用于新派发。",
            ),
        },
        ActionDef {
            id: ActionId::DashboardToggleGrouping,
            label: "分组",
            description: "切换行分组",
            // `Ctrl+G` ("group"). `Ctrl+S` was reassigned to the peek /
            // dispatch "send + open" chord so `Shift+Enter` could be
            // freed for newline insertion. (`Ctrl+G` is also bound to
            // `SendToBackground`, but that lives in `When::AgentScreen`,
            // a context that never overlaps the dashboard.)
            default_key: key!('g', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: Some(
                "在「扁平列表」与「按状态分组」（如工作中 / 空闲）之间切换仪表盘。\n分组便于突出需要关注的代理；扁平列表顺序更稳定。\n选择会跨会话保留。",
            ),
        },
        ActionDef {
            id: ActionId::DashboardReorderUp,
            label: "上移",
            description: "上移代理",
            default_key: key!(Up, SHIFT),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Shift+\u{2191}"),
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardReorderDown,
            label: "下移",
            description: "下移代理",
            default_key: key!(Down, SHIFT),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardShortcutsHelp,
            label: "快捷键",
            description: "显示快捷键浮层",
            // Ctrl+. / `?` dual-bound; primary follows ctrl_dot_unreliable.
            // Ctrl+X is DashboardStop — never an alt here.
            default_key: if ctrl_dot_unreliable {
                key!('?')
            } else {
                key!('.', CONTROL)
            },
            alt_keys: vec![if ctrl_dot_unreliable {
                key!('.', CONTROL)
            } else {
                key!('?')
            }],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: None,
            requires_confirmation: false,
            long_help: None,
        },
        // `DashboardExit` is registered as a discoverable
        // action with its DEFAULT key set to Esc, but the in-dashboard
        // Esc behaviour is a multi-tier cascade (peek → input/filter
        // → exit) that no single action can express. The Esc cascade
        // in `state::handle_key` runs BEFORE this registry lookup so
        // Esc always cascades. A user who REBINDS Esc to something
        // else gains a discoverable exit shortcut for the rebound key,
        // and the original Esc cascade still works because the
        // cascade is keyed on `KeyCode::Esc` directly. The contract
        // is therefore: "Esc always cascades; any other key bound to
        // `DashboardExit` exits directly." The hint key shows the
        // effective binding via `Esc` as a fallback.
        ActionDef {
            id: ActionId::DashboardExit,
            label: "退出",
            description: "关闭仪表盘",
            default_key: key!(Esc),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Esc"),
            requires_confirmation: false,
            long_help: Some(
                "关闭仪表盘并返回原先所在位置。\nEsc 为级联：先关闭预览或清除筛选，没有待处理项后才真正退出。\n若将此操作改绑到其他键，可直接退出。",
            ),
        },
        // Mirror of `ToggleYolo` (Ctrl+O) but scoped to the
        // dashboard — flips the selected row's agent's
        // always-approve / YOLO mode. Reachable from the dashboard
        // view (and from inside the session overlay).
        ActionDef {
            id: ActionId::DashboardToggleAutoApprove,
            label: "始终批准",
            description: "切换始终批准",
            default_key: key!('o', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Ctrl+O"),
            requires_confirmation: false,
            long_help: Some(
                "在仪表盘上直接切换选中代理的自动批准（YOLO），无需接入会话。\n开启后该代理每次工具调用不再逐项确认。\n会话内对应操作为 Ctrl+O。",
            ),
        },
        // Open the location picker — a floating modal to change the
        // working directory new dashboard sessions spawn in. Ctrl+L
        // ("location") is free under `DashboardFocused` (it only binds
        // OpenExtensions under `AgentScreen`, a different context).
        ActionDef {
            id: ActionId::DashboardOpenLocationPicker,
            label: "目录",
            description: "更改新代理的工作目录",
            default_key: key!('l', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Ctrl+l"),
            requires_confirmation: false,
            long_help: Some(
                "打开选择器，设置仪表盘新派发代理的工作目录。\n无需离开仪表盘即可对其他仓库或文件夹启动代理。\n仅影响新派发，不影响已在运行的代理。",
            ),
        },
        // Toggle worktree-dispatch mode. Ctrl+W ("worktree") arms the next
        // dashboard-dispatched session to spawn in a fresh git worktree; the
        // dispatcher gates it on the cwd being a git repo. Free under
        // `DashboardFocused` (Ctrl+W only binds the overlay-exit fallback
        // under `DashboardOverlay`, a different context).
        ActionDef {
            id: ActionId::DashboardToggleWorktree,
            label: "worktree",
            description: "切换新代理的 worktree 模式",
            default_key: key!('w', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardFocused,
            hint_priority: None,
            hint_key_display: Some("Ctrl+w"),
            requires_confirmation: false,
            long_help: Some(
                "使下一次从仪表盘派发的代理在新的 git worktree 中启动，在独立检出中隔离工作。\n仅当工作目录是 git 仓库时生效。\n仅影响新派发，不影响已在运行的代理。",
            ),
        },
        // Session overlay (dashboard → agent attach)
        // bindings. They use `When::DashboardOverlay`: the agent-side
        // overlay intercept (`app_view`) looks them up in that context, and
        // the cheatsheet uses it to dim them on the dashboard LIST (where
        // they don't apply) while keeping them lit inside the overlay.
        ActionDef {
            id: ActionId::DashboardOverlayExit,
            label: "关闭浮层",
            description: "返回仪表盘",
            // The primary back-out shortcuts are reached through
            // different routes:
            //   - Ctrl+\\ → OpenDashboard (registered separately above);
            //     the overlay-input intercept treats it as overlay-exit.
            //   - `q` when scrollback is focused — handled by the
            //     overlay intercept directly.
            //   - Esc when the agent is in a "neutral" state
            //     (no modals/viewers/overlays, no text selection,
            //     no link highlight, no question/goal/rewind/
            //     permission overlays). Per-pane Esc consumers
            //     still take precedence — see `overlay_esc_*`
            //     tests in `app_view`.
            //   - `[✗]` click — routed via this action by the
            //     mouse handler.
            // The `default_key` mirrors the real primary route, Ctrl+\
            // (OpenDashboard, treated as overlay-exit), so the cheatsheet hint
            // is accurate. (Ctrl+W is NOT used here — it's the dashboard's
            // worktree toggle.)
            default_key: key!('\\', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardOverlay,
            hint_priority: None,
            hint_key_display: Some("Ctrl+\\"),
            requires_confirmation: false,
            long_help: Some(
                "离开已接入的会话浮层并返回仪表盘列表，不停止代理。\n也可在回滚区按 q、中性 Esc 或关闭按钮。\n若要停止代理而非仅分离，请用 Ctrl+X。",
            ),
        },
        ActionDef {
            id: ActionId::DashboardOverlayPrev,
            label: "上一会话",
            description: "上一会话",
            default_key: key!('[', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardOverlay,
            hint_priority: None,
            hint_key_display: Some("Ctrl+["),
            requires_confirmation: false,
            long_help: None,
        },
        ActionDef {
            id: ActionId::DashboardOverlayNext,
            label: "下一会话",
            description: "下一会话",
            default_key: key!(']', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardOverlay,
            hint_priority: None,
            hint_key_display: Some("Ctrl+]"),
            requires_confirmation: false,
            long_help: None,
        },
        // Dashboard-parity stop inside the session overlay — state
        // machine documented at `dispatch_dashboard_overlay_stop`.
        // Intentionally shadows the agent view's `ShortcutsHelp` alt
        // binding (Ctrl+X) inside the overlay; Ctrl+. still opens the
        // cheatsheet there.
        ActionDef {
            id: ActionId::DashboardOverlayStop,
            label: "停止",
            description: "停止代理并关闭会话（返回仪表盘）",
            default_key: key!('x', CONTROL),
            alt_keys: vec![],
            category: Category::Dashboard,
            context: When::DashboardOverlay,
            hint_priority: None,
            hint_key_display: Some("Ctrl+x"),
            requires_confirmation: true,
            long_help: Some(
                "在会话浮层内，停止已接入的代理并关闭，返回仪表盘列表。\n需要确认：连按两次 Ctrl+X。\n此处 Ctrl+. 仍打开快捷键表；仅 Ctrl+X 被占用为停止。",
            ),
        },
    ]);

    actions
}
