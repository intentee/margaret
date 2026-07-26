use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::views::renders_view::RendersView;

use super::home_view::HomeViewProps;
use super::margaret::views::Views;
use super::secrets::Secrets;

#[singleton]
#[responds_to_http(method = "get", path = "/", server = "public")]
pub struct GetHome {
    secrets: Arc<Secrets>,
}

impl GetHome {
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> anyhow::Result<Self> {
        Ok(Self { secrets })
    }

    #[process]
    pub async fn respond(&self, views: &Views) -> anyhow::Result<Response> {
        Ok({
            let _ = self.secrets.token();

            Response::html(
                200,
                views.home_view.render(HomeViewProps {
                    heading: "home".to_string(),
                })?,
            )
        })
    }
}
