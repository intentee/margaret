use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryStreamExt as _;
use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::creation::Creation;
use margaret::framework::active_record::detached::Detached;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::constants::article_batch::ARTICLE_BATCH;
use crate::enums::article_status::ArticleStatus;
use crate::margaret::models::models_article_article::draft::Draft;
use crate::models::article::Article;
use crate::models::author::Author;

#[singleton]
pub struct ArticleRepository {
    database: Arc<Database>,
}

impl ArticleRepository {
    #[must_use]
    #[constructor]
    pub fn create(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the draft cannot be stored.
    pub async fn draft(
        &self,
        title: String,
        body: String,
        author: Uuid,
        created_at: DateTime<Utc>,
    ) -> Result<Creation<Article>, ActiveRecordError> {
        Article::create(Draft {
            title,
            body,
            cover: None,
            published: false,
            price: Decimal::ZERO,
            reading_minutes: 0.0,
            status: ArticleStatus::Draft,
            created_at,
            author: Key::new(author),
        })
        .when(Author::query().id.eq(author).exists::<Detached>())
        .run(self.database.as_ref())
        .await
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the articles cannot be read.
    pub async fn listed(&self, author: Option<String>) -> Result<Vec<Article>, ActiveRecordError> {
        let database = self.database.as_ref();

        match author {
            Some(name) => match Author::query().name.eq(name).find(database).await? {
                Lookup::Found(author) => {
                    Article::query()
                        .author
                        .eq(Key::of(&author))
                        .id
                        .ascending()
                        .stream::<ARTICLE_BATCH, _>(database)
                        .try_collect()
                        .await
                }
                Lookup::Missing => Ok(Vec::new()),
            },
            None => {
                Article::query()
                    .id
                    .ascending()
                    .stream::<ARTICLE_BATCH, _>(database)
                    .try_collect()
                    .await
            }
        }
    }
}
