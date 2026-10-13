use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::IssuerClient;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(client: Arc<IssuerClient>) -> anyhow::Result<Self> {}
}
