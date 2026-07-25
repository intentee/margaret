use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = "get", path = "/articles", server = "public")]
pub struct GetArticles {
    articles: Arc<ArticleStore>,
}

impl GetArticles {
    #[constructor]
    #[must_use]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        routes: &Routes,
        #[form_request(from = Query)] GetArticlesForm { author }: GetArticlesForm,
    ) -> Response {
        let author = author.as_deref();
        let articles = match self.articles.all().await {
            Ok(articles) => articles,
            Err(error) => return Response::text(500, error.to_string()),
        };
        let links = articles
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
    }
}
