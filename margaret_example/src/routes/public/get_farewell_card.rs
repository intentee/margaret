use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::config::Config;
use crate::margaret::routes::Routes;
use crate::views::farewell_view::farewell_view;
use crate::views::farewell_view_props::FarewellViewProps;

#[singleton]
#[responds_to_http(method = "get", path = "/farewell-card", server = "public")]
pub struct GetFarewellCard {
    config: Arc<Config>,
}

impl GetFarewellCard {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }

    #[process]
    pub async fn respond(&self, routes: &Routes) -> Response {
        Response::html(
            200,
            farewell_view(FarewellViewProps {
                home_url: routes.public.get_greeting.url(),
                name: self.config.app_name().to_string(),
            }),
        )
    }
}
