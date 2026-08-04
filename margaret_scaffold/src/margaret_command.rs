use clap::Subcommand;

use crate::scaffold_command::ScaffoldCommand;

#[derive(Subcommand)]
pub(crate) enum MargaretCommand {
    Scaffold(ScaffoldCommand),
}
