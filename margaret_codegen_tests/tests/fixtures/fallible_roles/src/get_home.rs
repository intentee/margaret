use std::sync::Arc;

use failures::Result as Outcome;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::views::renders_view::RendersView;

use super::home_view::HomeViewProps;
use super::margaret::views::Views;
use super::secrets::Secrets;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/", server = "public")]
pub struct GetHome {
    secrets: Arc<Secrets>,
}

impl GetHome {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> Outcome<Self> {
        Ok(Self { secrets })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, views: &Views) -> Outcome<Response> {
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
