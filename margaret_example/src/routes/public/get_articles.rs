use std::sync::Arc;

use anyhow::Context as _;

use margaret::framework::http::response::Response;
use margaret::framework::http::route_addressing::RouteAddressing;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::url_path::path_parameter_value::PathParameterValue;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::margaret::routes::props::GetArticleRouteProps;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/articles", server = "public")]
pub struct GetArticles {
    articles: Arc<ArticleRepository>,
}

impl GetArticles {
    #[must_use]
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        routes: &Routes,
        #[form_request(from = RequestInput::Query)] GetArticlesForm { author }: GetArticlesForm,
    ) -> anyhow::Result<Response> {
        Ok({
            let links = self
                .articles
                .listed(author)
                .await?
                .into_iter()
                .map(|article| {
                    let article_parameter = PathParameterValue::new(article.id.to_string())
                        .with_context(|| {
                            format!(
                                "the article {} cannot fill the route parameter 'article'",
                                article.id
                            )
                        })?;

                    match routes.public.get_article.address(GetArticleRouteProps {
                        article: article_parameter,
                    }) {
                        RouteAddressing::Addressed(route) => {
                            Ok(format!("{}: {}", article.title, route.url()))
                        }
                        RouteAddressing::Misrouted { url } => Err(anyhow::anyhow!(
                            "the link of the article {} at '{url}' reaches another route",
                            article.id
                        )),
                    }
                })
                .collect::<anyhow::Result<Vec<String>>>()?
                .join("\n");

            Response::text(
                200,
                format!("{links}\ncreate: {}", routes.public.post_article.url()),
            )
        })
    }
}
