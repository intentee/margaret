use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use dashmap::DashMap;
use uuid::Uuid;

use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_macros::constructor;
use margaret_macros::provides_route_parameter;
use margaret_macros::singleton;

use crate::clock::Clock;
use crate::models::article::Article;
use crate::models::author::Author;
use crate::stores::article_store_error::ArticleStoreError;

const FIRST_AUTHORED_ID: u64 = 103;

pub const FEATURED_ARTICLE_ID: Uuid = Uuid::from_u128(100);

fn at_epoch_seconds(seconds: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(seconds * 1_000_000_000)
}

fn milo() -> Author {
    Author {
        id: Uuid::from_u128(3),
        name: "Milo".to_string(),
        active: true,
        joined_at: at_epoch_seconds(1_600_000_000),
        bio: Some("Writes public notes.".to_string()),
    }
}

fn mona() -> Author {
    Author {
        id: Uuid::from_u128(2),
        name: "Mona".to_string(),
        active: true,
        joined_at: at_epoch_seconds(1_610_000_000),
        bio: None,
    }
}

fn known_authors() -> Vec<Author> {
    vec![milo(), mona()]
}

fn seed() -> Vec<Article> {
    vec![
        Article {
            id: FEATURED_ARTICLE_ID,
            title: "Shipping Margaret".to_string(),
            body: "A public note from Milo.".to_string(),
            published: true,
            created_at: at_epoch_seconds(1_704_067_200),
            author: milo(),
        },
        Article {
            id: Uuid::from_u128(101),
            title: "Milo's draft".to_string(),
            body: "An unpublished draft from Milo.".to_string(),
            published: false,
            created_at: at_epoch_seconds(1_704_153_600),
            author: milo(),
        },
        Article {
            id: Uuid::from_u128(102),
            title: "Mona's draft".to_string(),
            body: "An unpublished draft from Mona.".to_string(),
            published: false,
            created_at: at_epoch_seconds(1_704_240_000),
            author: mona(),
        },
    ]
}

#[singleton]
#[provides_route_parameter]
pub struct ArticleStore {
    articles: DashMap<Uuid, Article>,
    authors: Vec<Author>,
    clock: Arc<dyn Clock>,
    next_id: AtomicU64,
}

impl ArticleStore {
    #[constructor]
    pub fn create(clock: Arc<dyn Clock>) -> Self {
        let articles = DashMap::new();

        for article in seed() {
            articles.insert(article.id, article);
        }

        Self {
            articles,
            authors: known_authors(),
            clock,
            next_id: AtomicU64::new(FIRST_AUTHORED_ID),
        }
    }

    pub fn all(&self) -> Vec<Article> {
        let mut all: Vec<Article> = self
            .articles
            .iter()
            .map(|article| article.value().clone())
            .collect();

        all.sort_by_key(|article| article.id);

        all
    }

    pub fn find_article_by_id(&self, id: Uuid) -> Option<Article> {
        self.articles
            .get(&id)
            .map(|article| article.value().clone())
    }

    pub fn insert(
        &self,
        title: String,
        body: String,
        author_id: Uuid,
    ) -> Result<Article, ArticleStoreError> {
        let author = self
            .authors
            .iter()
            .find(|author| author.id == author_id)
            .cloned()
            .ok_or(ArticleStoreError::AuthorNotFound { author_id })?;

        let id = Uuid::from_u128(u128::from(self.next_id.fetch_add(1, Ordering::Relaxed)));
        let article = Article {
            id,
            title,
            body,
            published: false,
            created_at: self.clock.now(),
            author,
        };

        self.articles.insert(id, article.clone());

        Ok(article)
    }

    pub fn remove(&self, id: Uuid) {
        self.articles.remove(&id);
    }

    pub fn save(&self, article: Article) {
        self.articles.insert(article.id, article);
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ArticleStore {
    type Model = Article;

    async fn bind(&self, value: String) -> Option<Article> {
        Uuid::parse_str(&value)
            .ok()
            .and_then(|id| self.find_article_by_id(id))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder;

    use super::ArticleStore;
    use super::FEATURED_ARTICLE_ID;
    use crate::system_clock::SystemClock;

    fn store() -> ArticleStore {
        ArticleStore::create(Arc::new(SystemClock::create()))
    }

    #[test]
    fn seeds_the_featured_article_with_its_author() {
        let article = store()
            .find_article_by_id(FEATURED_ARTICLE_ID)
            .expect("the featured article is seeded");

        assert_eq!(article.title, "Shipping Margaret");
        assert_eq!(article.author.name, "Milo");
    }

    #[test]
    fn inserts_an_article_for_a_known_author() {
        let store = store();
        let article = store
            .insert("Title".to_string(), "Body".to_string(), Uuid::from_u128(3))
            .expect("the article is inserted for a known author");

        assert_eq!(article.author.name, "Milo");
        assert!(store.find_article_by_id(article.id).is_some());
    }

    #[test]
    fn rejects_an_article_for_an_unknown_author() {
        assert!(
            store()
                .insert(
                    "Title".to_string(),
                    "Body".to_string(),
                    Uuid::from_u128(999)
                )
                .is_err()
        );
    }

    #[test]
    fn removes_an_article() {
        let store = store();
        store.remove(FEATURED_ARTICLE_ID);

        assert!(store.find_article_by_id(FEATURED_ARTICLE_ID).is_none());
    }

    #[test]
    fn lists_the_seeded_articles_in_id_order() {
        let all = store().all();

        assert_eq!(all.len(), 3);
        assert_eq!(all[0].id, FEATURED_ARTICLE_ID);
    }

    #[tokio::test]
    async fn binds_an_article_by_its_uuid() {
        let store = store();

        assert!(
            store
                .bind(FEATURED_ARTICLE_ID.to_string())
                .await
                .is_some()
        );
        assert!(store.bind("not-a-uuid".to_string()).await.is_none());
    }
}
