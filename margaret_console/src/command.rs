use async_trait::async_trait;
use clap::Arg;
use clap::ArgMatches;

use crate::command_outcome::CommandOutcome;

#[async_trait]
pub trait Command: Send + Sync {
    fn arguments(&self) -> Vec<Arg>;

    async fn run(&self, matches: &ArgMatches) -> CommandOutcome;
}
