use std::path::PathBuf;

use async_trait::async_trait;

use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::route_models::shelf::Shelf;

#[singleton]
#[provides_route_parameter]
pub struct ShelfBinder {
    catalog: PathBuf,
}

impl ShelfBinder {
    #[must_use]
    #[constructor]
    pub fn create(#[console_argument(from = "catalog")] catalog: PathBuf) -> Self {
        Self { catalog }
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ShelfBinder {
    type Model = Shelf;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Shelf>> {
        let catalog = tokio::fs::read_to_string(&self.catalog).await?;

        Ok(if catalog.lines().any(|listed| listed == value) {
            RouteParameterBindingOutcome::Bound(Shelf { name: value })
        } else {
            RouteParameterBindingOutcome::NotFound
        })
    }
}
