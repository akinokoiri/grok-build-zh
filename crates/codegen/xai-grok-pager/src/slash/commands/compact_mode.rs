//! `/compact-mode` -- toggle compact display mode.
//!
//! Reduces user message padding by disabling vertical padding on prompt blocks.
//!
//! Dispatches `Action::ToggleCompactMode` so the slash command and the
//! keybinding share one toggle gate (which reads the USER value — the render
//! value may be auto-forced on short terminals).

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

/// Toggle compact display mode via `/compact-mode`.
pub struct CompactModeCommand;

impl SlashCommand for CompactModeCommand {
    fn name(&self) -> &str {
        "compact-mode"
    }

    fn description(&self) -> &str {
        "切换紧凑界面（减少留白，显示更多内容）"
    }

    fn usage(&self) -> &str {
        "/compact-mode"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::ToggleCompactMode)
    }
}
