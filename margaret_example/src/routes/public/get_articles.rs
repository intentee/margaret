use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Get, path = "/articles", server = "public")]
pub struct GetArticles {
    articles: Arc<ArticleRepository>,
}

impl GetArticles {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        routes: &Routes,
        #[form_request(from = Query)] GetArticlesForm { author }: GetArticlesForm,
    ) -> Response {
        let author = author.as_deref();
        let links = self
            .articles
            .all()
            .into_iter()
            .filter(|article| match author {
                Some(author) => article.author_id.as_str() == author,
                None => true,
            })
            .map(|article| {
                let url = routes.public.get_article(article.id).url();

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
