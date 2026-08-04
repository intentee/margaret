use clap::Args;

use crate::scaffold_operation::ScaffoldOperation;

#[derive(Args)]
pub(crate) struct ScaffoldCommand {
    #[command(subcommand)]
    pub operation: ScaffoldOperation,
}
