use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/articles", server = "public")]
pub struct GetArticles {
    articles: Arc<ArticleStore>,
}

impl GetArticles {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> anyhow::Result<Self> {
        Ok(Self { articles })
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
            let author = author.as_deref();
            let links = self
                .articles
                .all()
                .await?
                .into_iter()
                .filter(|article| match author {
                    Some(author) => article.author.name == author,
                    None => true,
                })
                .map(|article| {
                    let url = routes.public.get_article(article.id.to_string()).url();

                    format!("{}: {url}", article.title)
                })
                .collect::<Vec<String>>()
                .join("\n");

            Response::text(
                200,
                format!("{links}\ncreate: {}", routes.public.post_article.url()),
            )
        })
    }
}
