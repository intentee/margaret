use std::sync::Arc;

use crate::IssuerClient;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(client: Arc<IssuerClient>) -> anyhow::Result<Self> {}
}
