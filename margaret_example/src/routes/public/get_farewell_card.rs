use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::config::Config;
use crate::views::farewell_view::FarewellView;
use crate::views::view::View;

#[singleton]
#[responds_to_http(method = Get, path = "/farewell-card", server = "public")]
pub struct GetFarewellCard {
    config: Arc<Config>,
}

impl GetFarewellCard {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }

    #[responder]
    pub async fn respond(&self) -> Box<dyn View> {
        Box::new(FarewellView {
            name: self.config.app_name().to_string(),
        })
    }
}
