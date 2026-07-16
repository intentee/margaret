use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ScheduledArgumentRunner<Argument>: Send + Sync + 'static
where
    Argument: Send + 'static,
{
    async fn run_scheduled_tick(&self, argument: Argument) -> Result<()>;
}
