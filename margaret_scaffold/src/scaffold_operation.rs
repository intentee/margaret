use clap::Subcommand;

use crate::init_arguments::InitArguments;

#[derive(Subcommand)]
pub(crate) enum ScaffoldOperation {
    Init(InitArguments),
}
