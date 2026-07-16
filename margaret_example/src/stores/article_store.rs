use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use dashmap::DashMap;

use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_macros::constructor;
use margaret_macros::provides_route_parameter;
use margaret_macros::singleton;

use std::time::SystemTimeError;

use crate::clock::Clock;
use crate::models::article::Article;

const FIRST_AUTHORED_ID: u64 = 103;

fn seed() -> Vec<Article> {
    vec![
        Article {
            id: "100".to_string(),
            title: "Shipping Margaret".to_string(),
            author_id: "3".to_string(),
            body: "A public note from Milo.".to_string(),
            published: true,
            created_at: 1_704_067_200,
        },
        Article {
            id: "101".to_string(),
            title: "Milo's draft".to_string(),
            author_id: "3".to_string(),
            body: "An unpublished draft from Milo.".to_string(),
            published: false,
            created_at: 1_704_153_600,
        },
        Article {
            id: "102".to_string(),
            title: "Mona's draft".to_string(),
            author_id: "2".to_string(),
            body: "An unpublished draft from Mona.".to_string(),
            published: false,
            created_at: 1_704_240_000,
        },
    ]
}

#[singleton]
#[provides_route_parameter]
pub struct ArticleStore {
    articles: DashMap<String, Article>,
    clock: Arc<dyn Clock>,
    next_id: AtomicU64,
}

impl ArticleStore {
    #[constructor]
    pub fn create(clock: Arc<dyn Clock>) -> Self {
        let articles = DashMap::new();

        for article in seed() {
            articles.insert(article.id.clone(), article);
        }

        Self {
            articles,
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

        all.sort_by(|first, second| first.id.cmp(&second.id));

        all
    }

    pub fn find_article_by_id(&self, id: &str) -> Option<Article> {
        self.articles.get(id).map(|article| article.value().clone())
    }

    pub fn insert(
        &self,
        title: String,
        body: String,
        author_id: String,
    ) -> Result<Article, SystemTimeError> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed).to_string();
        let article = Article {
            id: id.clone(),
            title,
            author_id,
            body,
            published: false,
            created_at: self.clock.now()?,
        };

        self.articles.insert(id, article.clone());

        Ok(article)
    }

    pub fn remove(&self, id: &str) {
        self.articles.remove(id);
    }

    pub fn save(&self, article: Article) {
        self.articles.insert(article.id.clone(), article);
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ArticleStore {
    type Model = Article;

    async fn bind(&self, value: String) -> Option<Article> {
        self.find_article_by_id(&value)
    }
}
