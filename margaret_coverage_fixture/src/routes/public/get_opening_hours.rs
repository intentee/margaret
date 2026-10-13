use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::config::opening_hours::OpeningHours;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/opening-hours", server = "public")]
pub struct GetOpeningHours {
    opening_hours: Arc<OpeningHours>,
}

impl GetOpeningHours {
    #[must_use]
    #[constructor]
    pub fn create(opening_hours: Arc<OpeningHours>) -> Self {
        Self { opening_hours }
    }

    /// # Errors
    ///
    /// Never returns an error; the opening hours are always known.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            format!(
                "{}-{}",
                self.opening_hours.opens_at(),
                self.opening_hours.closes_at()
            ),
        ))
    }
}
