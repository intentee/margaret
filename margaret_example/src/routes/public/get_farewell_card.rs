use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::views::renders_view::RendersView;

use crate::app_name::AppName;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::views::farewell_view::FarewellViewProps;

#[singleton]
#[responds_to_http(method = "get", path = "/farewell-card", server = "public")]
pub struct GetFarewellCard {
    app_name: Arc<AppName>,
}

impl GetFarewellCard {
    #[constructor]
    pub fn create(app_name: Arc<AppName>) -> anyhow::Result<Self> {
        Ok(Self { app_name })
    }

    #[process]
    pub async fn respond(&self, routes: &Routes, views: &Views) -> anyhow::Result<Response> {
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret::framework::http::route_origin::RouteOrigin;

    use super::GetFarewellCard;
    use crate::app_name::AppName;
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
            farewell_view: Arc::new(
                FarewellView::create(card_layout.clone())
                    .expect("the farewell view is constructed"),
            ),
            greeting_view: Arc::new(
                GreetingView::create(card_layout).expect("the greeting view is constructed"),
            ),
        };
        let routes = Routes::from_origins(
            RouteOrigin::parse("https://internal.example").expect("a valid internal origin"),
            RouteOrigin::parse("https://public.example").expect("a valid public origin"),
        );
        let app_name = AppName::create().expect("the app name is constructed");
        let responder =
            GetFarewellCard::create(Arc::new(app_name)).expect("the responder is constructed");

        assert_eq!(
            responder
                .respond(&routes, &views)
                .await
                .expect("the responder succeeds")
                .status(),
            200
        );
    }
}
