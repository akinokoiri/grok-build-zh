use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

pub struct McpsCommand;

impl SlashCommand for McpsCommand {
    fn name(&self) -> &str {
        "mcps"
    }

    fn description(&self) -> &str {
        "显示 MCP 服务器状态"
    }

    fn usage(&self) -> &str {
        "/mcps"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::OpenExtensionsModal {
            tab: crate::views::extensions_modal::ExtensionsTab::McpServers,
            trigger: xai_grok_telemetry::events::ExtensionsModalTrigger::SlashCommand,
        })
    }
}
