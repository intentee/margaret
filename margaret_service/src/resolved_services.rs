use anyhow::Result;
use async_trait::async_trait;
use trzcina::Service;
use trzcina::ServiceBundle;

pub struct ResolvedServices {
    pub services: Vec<Box<dyn Service>>,
}

#[async_trait]
impl ServiceBundle for ResolvedServices {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(self.services)
    }
}
