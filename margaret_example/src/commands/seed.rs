use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;
use margaret::framework::sessions::started_session::StartedSession;

use crate::featured_article_id::FEATURED_ARTICLE_ID;
use crate::margaret::sessions::IssuedSessions;
use crate::models::article::Article;
use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;
use crate::models::user_account::UserAccount;
use crate::system_clock::SystemClock;

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

fn articles(milo: &Author, mona: &Author) -> Vec<Article> {
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
            author: Key::of(milo),
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
            author: Key::of(milo),
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
            author: Key::of(mona),
        },
    ]
}

#[singleton]
#[console_command(
    name = "seed",
    description = "Seeds the blog database with its sample authors, articles, and reader, and prints the session cookies of the reader"
)]
pub struct Seed {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
    sessions: Arc<IssuedSessions>,
}

impl Seed {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        database: Arc<Database>,
        sessions: Arc<IssuedSessions>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            database,
            sessions,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the sample data cannot be stored or the session of the reader cannot
    /// start.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let reader = self.stored_sample_data().await?;
        let StartedSession { cookie_changes, .. } =
            self.sessions.start(reader.id, self.clock.now()).await?;

        for cookie in &cookie_changes.cookies {
            println!("{}={}", cookie.name(), cookie.value());
        }

        Ok(CommandOutcome::Succeeded)
    }

    async fn stored_sample_data(&self) -> anyhow::Result<UserAccount> {
        let milo = milo();
        let mona = mona();
        let reader = UserAccount {
            id: milo.id,
            name: milo.name.clone(),
        };
        let mut connection = self.database.connection().await?;
        let transaction = connection.transaction(Isolation::ReadCommitted).await?;

        for author in [&milo, &mona] {
            author.insert().or_ignore(&transaction).await?;
        }

        for article in &articles(&milo, &mona) {
            article.insert().or_ignore(&transaction).await?;
        }

        reader.insert().or_ignore(&transaction).await?;
        transaction.commit().await?;

        Ok(reader)
    }
}
