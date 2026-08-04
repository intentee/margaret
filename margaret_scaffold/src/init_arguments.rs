use clap::Args;

#[derive(Args)]
pub(crate) struct InitArguments {
    #[arg(long)]
    pub margaret_rev: String,
    pub project_name: String,
}
