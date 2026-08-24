//! `/share` -- share current session via URL.

use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

/// Share the current session via a public URL.
pub struct ShareCommand;

impl SlashCommand for ShareCommand {
    fn name(&self) -> &str {
        "share"
    }

    fn description(&self) -> &str {
        "通过 URL 分享此会话"
    }

    fn session_scoped(&self) -> bool {
        true
    }

    fn usage(&self) -> &str {
        "/share"
    }

    fn run(&self, ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        let _ = ctx;
        CommandResult::Error("Session sharing is temporarily disabled".to_string())
    }
}
