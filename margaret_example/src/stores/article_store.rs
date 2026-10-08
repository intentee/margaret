use std::sync::Arc;

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::models::article::Article;
use crate::models::article_status::ArticleStatus;
use crate::stores::article_insertion::ArticleInsertion;
use crate::stores::article_row::article_row;
use crate::stores::article_statements::ArticleStatements;
use crate::stores::author_not_found::AuthorNotFound;
use crate::stores::blog_store_error::BlogStoreError;
use crate::system_clock::SystemClock;

#[singleton]
#[provides_route_parameter]
pub struct ArticleStore {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
    statements: ArticleStatements,
}

impl ArticleStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            database,
            statements: ArticleStatements::new(),
        })
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the articles cannot be read.
    pub async fn all(&self) -> Result<Vec<Article>, BlogStoreError> {
        self.database
            .client()
            .await
            .map_err(BlogStoreError::Unavailable)?
            .query(&self.statements.all, &[])
            .await
            .map_err(BlogStoreError::ListArticles)?
            .iter()
            .map(article_row)
            .collect()
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the article cannot be read.
    pub async fn find_article_by_id(&self, id: Uuid) -> Result<Option<Article>, BlogStoreError> {
        self.database
            .client()
            .await
            .map_err(BlogStoreError::Unavailable)?
            .query_opt(&self.statements.find, &[&id])
            .await
            .map_err(BlogStoreError::FindArticle)?
            .as_ref()
            .map(article_row)
            .transpose()
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the article cannot be inserted.
    pub async fn insert(
        &self,
        title: String,
        body: String,
        author_id: Uuid,
    ) -> Result<ArticleInsertion, BlogStoreError> {
        Ok(
            match self
                .database
                .client()
                .await
                .map_err(BlogStoreError::Unavailable)?
                .query_opt(
                    &self.statements.insert,
                    &[
                        &title,
                        &body,
                        &false,
                        &Decimal::ZERO,
                        &0.0_f64,
                        &ArticleStatus::Draft.stored(),
                        &self.clock.now(),
                        &author_id,
                    ],
                )
                .await
                .map_err(BlogStoreError::InsertArticle)?
            {
                Some(row) => ArticleInsertion::Inserted(article_row(&row)?),
                None => ArticleInsertion::AuthorNotFound(AuthorNotFound { author_id }),
            },
        )
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the article cannot be removed.
    pub async fn remove(&self, id: Uuid) -> Result<(), BlogStoreError> {
        self.database
            .client()
            .await
            .map_err(BlogStoreError::Unavailable)?
            .execute(self.statements.remove, &[&id])
            .await
            .map_err(BlogStoreError::RemoveArticle)
            .map(|_removed| ())
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the article cannot be saved.
    pub async fn save(
        &self,
        Article {
            id,
            title,
            body,
            cover,
            published,
            price,
            reading_minutes,
            status,
            created_at,
            author,
        }: &Article,
    ) -> Result<(), BlogStoreError> {
        self.database
            .client()
            .await
            .map_err(BlogStoreError::Unavailable)?
            .execute(
                self.statements.save,
                &[
                    id,
                    title,
                    body,
                    cover,
                    published,
                    price,
                    reading_minutes,
                    &status.stored(),
                    created_at,
                    &author.id,
                ],
            )
            .await
            .map_err(BlogStoreError::SaveArticle)
            .map(|_saved| ())
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ArticleStore {
    type Model = Article;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {
        Ok(match Uuid::parse_str(&value) {
            Ok(id) => match self.find_article_by_id(id).await? {
                Some(article) => RouteParameterBindingOutcome::Bound(article),
                None => RouteParameterBindingOutcome::NotFound,
            },
            Err(_malformed) => RouteParameterBindingOutcome::NotFound,
        })
    }
}
