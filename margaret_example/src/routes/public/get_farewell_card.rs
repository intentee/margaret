use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_views::renders_view::RendersView;

use crate::config::Config;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::views::farewell_view_props::FarewellViewProps;

#[singleton]
#[responds_to_http(method = "get", path = "/farewell-card", server = "public")]
pub struct GetFarewellCard {
    config: Arc<Config>,
}

impl GetFarewellCard {
    #[constructor]
    #[must_use]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }

    #[process]
    pub async fn respond(&self, routes: &Routes, views: &Views) -> Response {
        Response::html(
            200,
            views.farewell_view.render(FarewellViewProps {
                name: self.config.app_name().to_string(),
                routes,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::GetFarewellCard;
    use crate::config::Config;
    use crate::margaret::routes::Routes;
    use crate::margaret::views::Views;
    use crate::views::card_layout::CardLayout;
    use crate::views::farewell_view::FarewellView;
    use crate::views::greeting_view::GreetingView;

    #[tokio::test]
    async fn responds_with_the_rendered_farewell_card() {
        let card_layout = Arc::new(CardLayout);
        let views = Views {
            card_layout: card_layout.clone(),
            farewell_view: Arc::new(FarewellView::create(card_layout.clone())),
            greeting_view: Arc::new(GreetingView::create(card_layout)),
        };
        let routes = Routes::from_origins(Arc::from("http://internal"), Arc::from("http://public"));
        let responder = GetFarewellCard::create(Arc::new(Config::create()));

        assert_eq!(responder.respond(&routes, &views).await.status(), 200);
    }
}
