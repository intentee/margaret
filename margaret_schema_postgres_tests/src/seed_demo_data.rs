use chrono::DateTime;
use chrono::Utc;
use sqlx::PgPool;
use sqlx::query;
use uuid::Uuid;

use margaret_example::models::article::Article;
use margaret_example::models::article_status::ArticleStatus;
use margaret_example::models::author::Author;
use margaret_example::stores::article_store::FEATURED_ARTICLE_ID;

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

fn demo_articles() -> Vec<Article> {
    vec![
        Article {
            id: FEATURED_ARTICLE_ID,
            title: "Shipping Margaret".to_string(),
            body: "A public note from Milo.".to_string(),
            cover: None,
            published: true,
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
            status: ArticleStatus::Draft,
            created_at: at_epoch_seconds(1_704_240_000),
            author: mona(),
        },
    ]
}

async fn insert_author(pool: &PgPool, author: &Author) {
    query("INSERT INTO authors (id, name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4, $5)")
        .bind(author.id)
        .bind(&author.name)
        .bind(author.active)
        .bind(author.joined_at)
        .bind(&author.bio)
        .execute(pool)
        .await
        .expect("the demo author is seeded");
}

async fn insert_article(pool: &PgPool, article: &Article) {
    query(
        "INSERT INTO articles \
         (id, title, body, cover, published, status, created_at, author_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(article.id)
    .bind(&article.title)
    .bind(&article.body)
    .bind(&article.cover)
    .bind(article.published)
    .bind(article.status.as_text())
    .bind(article.created_at)
    .bind(article.author.id)
    .execute(pool)
    .await
    .expect("the demo article is seeded");
}

pub async fn seed_demo_data(pool: &PgPool) {
    for author in [milo(), mona()] {
        insert_author(pool, &author).await;
    }

    for article in demo_articles() {
        insert_article(pool, &article).await;
    }
}
