use anyhow::Result;
use async_trait::async_trait;
use trzcina::Service;
use trzcina::ServiceBundle;

pub struct NoBundle;

#[async_trait]
impl ServiceBundle for NoBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(Vec::new())
    }
}
