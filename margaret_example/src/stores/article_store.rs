use std::sync::Arc;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::AssertSqlSafe;
use sqlx::query;
use sqlx::query_as;
use uuid::Uuid;

use margaret::framework::http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;

use crate::margaret::postgres_pool::PgPool;
use crate::models::article::Article;
use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;
use crate::stores::article_store_error::ArticleStoreError;
use crate::system_clock::SystemClock;

pub const FEATURED_ARTICLE_ID: Uuid = Uuid::from_u128(100);

const SELECT_ARTICLES: &str = "\
SELECT \
articles.id, articles.title, articles.body, articles.cover, articles.published, \
articles.status, articles.created_at, \
authors.id, authors.name, authors.is_active, authors.joined_at, authors.bio \
FROM articles JOIN authors ON articles.author_id = authors.id";

type ArticleRow = (
    Uuid,
    String,
    String,
    Option<Vec<u8>>,
    bool,
    String,
    DateTime<Utc>,
    Uuid,
    String,
    bool,
    DateTime<Utc>,
    Option<String>,
);

type AuthorRow = (Uuid, String, bool, DateTime<Utc>, Option<String>);

fn author_from_row(row: AuthorRow) -> Author {
    let (id, name, active, joined_at, bio) = row;

    Author {
        id,
        name,
        active,
        joined_at,
        bio,
    }
}

fn article_from_row(row: ArticleRow) -> Result<Article, ArticleStoreError> {
    let (
        id,
        title,
        body,
        cover,
        published,
        status_text,
        created_at,
        author_id,
        author_name,
        author_active,
        author_joined_at,
        author_bio,
    ) = row;
    let status = match ArticleStatus::from_text(&status_text) {
        Some(status) => status,
        None => return Err(ArticleStoreError::UnknownStatus { value: status_text }),
    };

    Ok(Article {
        id,
        title,
        body,
        cover,
        published,
        status,
        created_at,
        author: Author {
            id: author_id,
            name: author_name,
            active: author_active,
            joined_at: author_joined_at,
            bio: author_bio,
        },
    })
}

#[singleton]
#[provides_route_parameter]
pub struct ArticleStore {
    clock: Arc<SystemClock>,
    pool: sqlx::PgPool,
}

impl ArticleStore {
    #[constructor]
    #[must_use]
    pub fn create(clock: Arc<SystemClock>, pool: Arc<PgPool>) -> Self {
        Self {
            clock,
            pool: (**pool).clone(),
        }
    }

    pub async fn all(&self) -> Result<Vec<Article>, ArticleStoreError> {
        let rows: Vec<ArticleRow> =
            query_as(AssertSqlSafe(format!("{SELECT_ARTICLES} ORDER BY articles.id")))
                .fetch_all(&self.pool)
                .await?;

        rows.into_iter().map(article_from_row).collect()
    }

    pub async fn find_article_by_id(&self, id: Uuid) -> Result<Option<Article>, ArticleStoreError> {
        let row: Option<ArticleRow> =
            query_as(AssertSqlSafe(format!("{SELECT_ARTICLES} WHERE articles.id = $1")))
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        row.map(article_from_row).transpose()
    }

    pub async fn insert(
        &self,
        title: String,
        body: String,
        author_id: Uuid,
    ) -> Result<Article, ArticleStoreError> {
        let author = self
            .find_author_by_id(author_id)
            .await?
            .ok_or(ArticleStoreError::AuthorNotFound { author_id })?;
        let created_at = self.clock.now();
        let (id,): (Uuid,) = query_as(
            "INSERT INTO articles \
             (title, body, cover, published, status, created_at, author_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        )
        .bind(&title)
        .bind(&body)
        .bind(None::<Vec<u8>>)
        .bind(false)
        .bind(ArticleStatus::Draft.as_text())
        .bind(created_at)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Article {
            id,
            title,
            body,
            cover: None,
            published: false,
            status: ArticleStatus::Draft,
            created_at,
            author,
        })
    }

    pub async fn remove(&self, id: Uuid) -> Result<(), ArticleStoreError> {
        query("DELETE FROM articles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn save(&self, article: Article) -> Result<(), ArticleStoreError> {
        query(
            "UPDATE articles SET \
             title = $2, body = $3, cover = $4, published = $5, status = $6, \
             created_at = $7, author_id = $8 WHERE id = $1",
        )
        .bind(article.id)
        .bind(&article.title)
        .bind(&article.body)
        .bind(&article.cover)
        .bind(article.published)
        .bind(article.status.as_text())
        .bind(article.created_at)
        .bind(article.author.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_author_by_id(&self, id: Uuid) -> Result<Option<Author>, ArticleStoreError> {
        let row: Option<AuthorRow> =
            query_as("SELECT id, name, is_active, joined_at, bio FROM authors WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(author_from_row))
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ArticleStore {
    type Model = Article;
    type Error = ArticleStoreError;

    async fn bind(&self, value: String) -> Result<Option<Article>, ArticleStoreError> {
        match Uuid::parse_str(&value) {
            Ok(id) => self.find_article_by_id(id).await,
            Err(_) => Ok(None),
        }
    }
}
