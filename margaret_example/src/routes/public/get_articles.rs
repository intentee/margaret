use std::sync::Arc;

use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http::route_addressing::RouteAddressing;
use margaret::framework::http::unaddressable_parameter::UnaddressableParameter;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/articles", server = "public")]
pub struct GetArticles {
    database: Arc<Database>,
}

impl GetArticles {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
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
            let links = Article::listed(self.database.as_ref(), author)
                .await?
                .into_iter()
                .map(
                    |article| match routes.public.get_article(article.id.to_string()) {
                        RouteAddressing::Addressed(route) => {
                            Ok(format!("{}: {}", article.title, route.url()))
                        }
                        RouteAddressing::Unaddressable(UnaddressableParameter {
                            name,
                            rejection,
                        }) => Err(anyhow::Error::from(rejection).context(format!(
                            "the article {} cannot fill the route parameter '{name}'",
                            article.id
                        ))),
                    },
                )
                .collect::<anyhow::Result<Vec<String>>>()?
                .join("\n");

            Response::text(
                200,
                format!("{links}\ncreate: {}", routes.public.post_article.url()),
            )
        })
    }
}
