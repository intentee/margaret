use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(
    method = "post",
    name = "post_article",
    path = "/articles",
    server = "public"
)]
pub struct PostArticle {
    articles: Arc<ArticleStore>,
}

impl PostArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> anyhow::Result<Self> {
        Ok(Self { articles })
    }

    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Form)] PostArticleForm {
            title,
            body,
            author_id,
        }: PostArticleForm,
    ) -> anyhow::Result<Response> {
        Ok({
            match self.articles.insert(title, body, author_id) {
                Ok(article) => Response::text(201, format!("created \"{}\"", article.title)),
                Err(error) => Response::text(500, error.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use super::PostArticle;
    use crate::forms::post_article_form::PostArticleForm;
    use crate::stores::article_store::ArticleStore;
    use crate::system_clock::SystemClock;

    #[tokio::test]
    async fn responds_with_500_when_the_author_is_unknown() {
        let clock = SystemClock::create().expect("the clock is constructed");
        let store =
            ArticleStore::create(Arc::new(clock)).expect("the article store is constructed");
        let responder = PostArticle::create(Arc::new(store)).expect("the responder is constructed");
        let form = PostArticleForm {
            title: "Title".to_string(),
            body: "Body".to_string(),
            author_id: Uuid::from_u128(999),
        };

        assert_eq!(
            responder
                .respond(form)
                .await
                .expect("the responder succeeds")
                .status(),
            500
        );
    }
}
