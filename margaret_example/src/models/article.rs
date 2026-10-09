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
use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;
use margaret::framework::model::on_delete::OnDelete;

use crate::article_batch::ARTICLE_BATCH;
use crate::margaret::models::models_article_article::draft::Draft;
use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;

#[model(table = "articles")]
#[index(name = "articles_by_author", fields = [author, id])]
#[derive(Clone)]
pub struct Article {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub title: String,
    #[column]
    pub body: String,
    #[column]
    pub cover: Option<Vec<u8>>,
    #[column]
    pub published: bool,
    #[column(precision = 12, scale = 2)]
    pub price: Decimal,
    #[column(minimum = 0)]
    pub reading_minutes: f64,
    #[column]
    pub status: ArticleStatus,
    #[column]
    #[index]
    pub created_at: DateTime<Utc>,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub author: Key<Author>,
}

impl Article {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the draft cannot be stored.
    pub async fn draft(
        database: &Database,
        title: String,
        body: String,
        author: Uuid,
        created_at: DateTime<Utc>,
    ) -> Result<Creation<Self>, ActiveRecordError> {
        Self::create(Draft {
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
        .run(database)
        .await
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the articles cannot be read.
    pub async fn listed(
        database: &Database,
        author: Option<String>,
    ) -> Result<Vec<Self>, ActiveRecordError> {
        match author {
            Some(name) => match Author::query().name.eq(name).find(database).await? {
                Lookup::Found(author) => {
                    Self::query()
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
                Self::query()
                    .id
                    .ascending()
                    .stream::<ARTICLE_BATCH, _>(database)
                    .try_collect()
                    .await
            }
        }
    }
}
