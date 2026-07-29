use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use tokio_util::sync::CancellationToken;

use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = "get", path = "/articles/digest", server = "public")]
pub struct GetArticlesDigest {
    articles: Arc<ArticleStore>,
}

impl GetArticlesDigest {
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> anyhow::Result<Self> {
        Ok(Self { articles })
    }

    #[process]
    pub async fn respond(&self, cancellation: &CancellationToken) -> anyhow::Result<Response> {
        Ok(
            match cancellation.run_until_cancelled(self.digest()).await {
                Some(digest) => Response::text(200, digest),
                None => Response::text(503, "the digest was abandoned before it finished"),
            },
        )
    }

    async fn digest(&self) -> String {
        self.articles
            .all()
            .into_iter()
            .map(|article| {
                format!(
                    "{}: {} words",
                    article.title,
                    article.body.split_whitespace().count()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio_util::sync::CancellationToken;

    use super::GetArticlesDigest;
    use crate::stores::article_store::ArticleStore;
    use crate::system_clock::SystemClock;

    fn responder() -> GetArticlesDigest {
        let clock = SystemClock::create().expect("the clock is constructed");
        let store =
            ArticleStore::create(Arc::new(clock)).expect("the article store is constructed");

        GetArticlesDigest::create(Arc::new(store)).expect("the responder is constructed")
    }

    #[tokio::test]
    async fn summarizes_every_article() {
        assert_eq!(
            responder()
                .respond(&CancellationToken::new())
                .await
                .expect("the responder succeeds")
                .status(),
            200
        );
    }

    #[tokio::test]
    async fn abandons_the_digest_once_the_request_is_cancelled() {
        let cancellation = CancellationToken::new();

        cancellation.cancel();

        assert_eq!(
            responder()
                .respond(&cancellation)
                .await
                .expect("the responder succeeds")
                .status(),
            503
        );
    }
}
