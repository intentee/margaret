use margaret::framework::tokio_postgres::Row;
use margaret::framework::tokio_postgres::types::FromSql;

use crate::models::article::Article;
use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;
use crate::stores::blog_store_error::BlogStoreError;

fn column<'row, Value: FromSql<'row>>(row: &'row Row, name: &str) -> Result<Value, BlogStoreError> {
    row.try_get(name)
        .map_err(BlogStoreError::MalformedArticleRow)
}

pub(crate) fn article_row(row: &Row) -> Result<Article, BlogStoreError> {
    let status: String = column(row, "status")?;

    Ok(Article {
        id: column(row, "id")?,
        title: column(row, "title")?,
        body: column(row, "body")?,
        cover: column(row, "cover")?,
        published: column(row, "published")?,
        price: column(row, "price")?,
        reading_minutes: column(row, "reading_minutes")?,
        status: ArticleStatus::from_stored(&status)
            .ok_or(BlogStoreError::UnknownArticleStatus { status })?,
        created_at: column(row, "created_at")?,
        author: Author {
            id: column(row, "author_id")?,
            name: column(row, "author_name")?,
            active: column(row, "author_is_active")?,
            joined_at: column(row, "author_joined_at")?,
            bio: column(row, "author_bio")?,
            reputation: column(row, "author_reputation")?,
        },
    })
}
