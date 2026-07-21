//! `/home` -- exit the current session and return to the welcome screen.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

/// Exit the current session and return to the welcome screen.
pub struct HomeCommand;

impl SlashCommand for HomeCommand {
    fn name(&self) -> &str {
        "home"
    }

    fn aliases(&self) -> &[&str] {
        &["welcome"]
    }

    fn description(&self) -> &str {
        "返回欢迎页"
    }

    fn usage(&self) -> &str {
        "/home"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::ExitSession)
    }
}
