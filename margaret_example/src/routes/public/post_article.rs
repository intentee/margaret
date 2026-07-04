use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::forms::post_article_form::PostArticleForm;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Post, name = "post_article", path = "/articles", server = "public")]
pub struct PostArticle {
    articles: Arc<ArticleRepository>,
}

impl PostArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Form)] PostArticleForm {
            title,
            body,
            author_id,
        }: PostArticleForm,
    ) -> Response {
        let article = self.articles.insert(title, body, author_id);

        Response::text(201, format!("created \"{}\"", article.title))
    }
}
