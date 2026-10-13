use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::views::renders_view::RendersView;

use crate::config::app_name::AppName;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::views::farewell_view::FarewellViewProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/farewell-card", server = "public")]
pub struct GetFarewellCard {
    app_name: Arc<AppName>,
}

impl GetFarewellCard {
    #[must_use]
    #[constructor]
    pub fn create(app_name: Arc<AppName>) -> Self {
        Self { app_name }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, routes: &Routes, views: &Views) -> anyhow::Result<Response> {
        Ok({
            Response::html(
                200,
                views.farewell_view.render(FarewellViewProps {
                    name: self.app_name.as_str().to_string(),
                    routes,
                })?,
            )
        })
    }
}
