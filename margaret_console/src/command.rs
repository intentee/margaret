use async_trait::async_trait;

use crate::command_outcome::CommandOutcome;

#[async_trait]
pub trait Command: Send + Sync {
    async fn run(&self) -> CommandOutcome;
}
