use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;
use margaret::framework::tokio_postgres::Transaction;

use crate::models::article::Article;
use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;
use crate::stores::blog_store_error::BlogStoreError;
use crate::stores::featured_article_id::FEATURED_ARTICLE_ID;
use crate::stores::milo_session::MILO_SESSION;
use crate::system_clock::SystemClock;

const SEED_ARTICLE: &str = "INSERT INTO articles \
     (id, title, body, cover, published, price, reading_minutes, status, created_at, author_id) \
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (id) DO NOTHING";

const SEED_AUTHOR: &str = "INSERT INTO authors (id, name, is_active, joined_at, bio, reputation) \
     VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING";

const SEED_SESSION: &str = "INSERT INTO user_sessions (id, authenticated_at, user_id) \
     VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING";

const SEED_USER: &str = "INSERT INTO users (id, name) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING";

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
        reputation: 4.5,
    }
}

fn mona() -> Author {
    Author {
        id: Uuid::from_u128(2),
        name: "Mona".to_string(),
        active: true,
        joined_at: at_epoch_seconds(1_610_000_000),
        bio: None,
        reputation: 3.75,
    }
}

fn articles() -> Vec<Article> {
    vec![
        Article {
            id: FEATURED_ARTICLE_ID,
            title: "Shipping Margaret".to_string(),
            body: "A public note from Milo.".to_string(),
            cover: None,
            published: true,
            price: Decimal::new(499, 2),
            reading_minutes: 7.5,
            status: ArticleStatus::Published,
            created_at: at_epoch_seconds(1_704_067_200),
            author: milo(),
        },
        Article {
            id: Uuid::from_u128(101),
            title: "Milo's draft".to_string(),
            body: "An unpublished draft from Milo.".to_string(),
            cover: None,
            published: false,
            price: Decimal::ZERO,
            reading_minutes: 2.25,
            status: ArticleStatus::Draft,
            created_at: at_epoch_seconds(1_704_153_600),
            author: milo(),
        },
        Article {
            id: Uuid::from_u128(102),
            title: "Mona's draft".to_string(),
            body: "An unpublished draft from Mona.".to_string(),
            cover: None,
            published: false,
            price: Decimal::ZERO,
            reading_minutes: 3.0,
            status: ArticleStatus::Draft,
            created_at: at_epoch_seconds(1_704_240_000),
            author: mona(),
        },
    ]
}

async fn seed_author(
    transaction: &Transaction<'_>,
    Author {
        id,
        name,
        active,
        joined_at,
        bio,
        reputation,
    }: &Author,
) -> Result<(), BlogStoreError> {
    transaction
        .execute(SEED_AUTHOR, &[id, name, active, joined_at, bio, reputation])
        .await
        .map_err(BlogStoreError::Seed)
        .map(|_seeded| ())
}

async fn seed_article(
    transaction: &Transaction<'_>,
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
    transaction
        .execute(
            SEED_ARTICLE,
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
        .map_err(BlogStoreError::Seed)
        .map(|_seeded| ())
}

#[singleton]
#[console_command(
    name = "seed",
    description = "Seeds the blog database with its sample authors, articles, and reader session"
)]
pub struct Seed {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl Seed {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns `BlogStoreError` when the sample data cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let milo = milo();
        let mut client = self
            .database
            .client()
            .await
            .map_err(BlogStoreError::Unavailable)?;
        let transaction = client.transaction().await.map_err(BlogStoreError::Seed)?;

        for author in [&milo, &mona()] {
            seed_author(&transaction, author).await?;
        }

        for article in &articles() {
            seed_article(&transaction, article).await?;
        }

        transaction
            .execute(SEED_USER, &[&milo.id, &milo.name])
            .await
            .map_err(BlogStoreError::Seed)?;
        transaction
            .execute(SEED_SESSION, &[&MILO_SESSION, &self.clock.now(), &milo.id])
            .await
            .map_err(BlogStoreError::Seed)?;
        transaction.commit().await.map_err(BlogStoreError::Seed)?;

        Ok(CommandOutcome::Succeeded)
    }
}
