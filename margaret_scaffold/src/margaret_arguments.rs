use clap::Parser;

use crate::margaret_command::MargaretCommand;

#[derive(Parser)]
#[command(name = "margaret", version)]
pub(crate) struct MargaretArguments {
    #[command(subcommand)]
    pub command: MargaretCommand,
}
