const ARTICLE_SELECTION: &str = "articles.id, articles.title, articles.body, articles.cover, \
     articles.published, articles.price, articles.reading_minutes, articles.status, \
     articles.created_at, authors.id AS author_id, authors.name AS author_name, \
     authors.is_active AS author_is_active, authors.joined_at AS author_joined_at, \
     authors.bio AS author_bio, authors.reputation AS author_reputation";

pub(crate) struct ArticleStatements {
    pub(crate) all: String,
    pub(crate) find: String,
    pub(crate) insert: String,
    pub(crate) remove: &'static str,
    pub(crate) save: &'static str,
}

impl ArticleStatements {
    pub(crate) fn new() -> Self {
        Self {
            all: format!(
                "SELECT {ARTICLE_SELECTION} FROM articles \
                 JOIN authors ON authors.id = articles.author_id ORDER BY articles.id"
            ),
            find: format!(
                "SELECT {ARTICLE_SELECTION} FROM articles \
                 JOIN authors ON authors.id = articles.author_id WHERE articles.id = $1"
            ),
            insert: format!(
                "WITH inserted AS (\
                 INSERT INTO articles \
                 (title, body, published, price, reading_minutes, status, created_at, author_id) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, authors.id FROM authors WHERE authors.id = $8 \
                 RETURNING *) \
                 SELECT {ARTICLE_SELECTION} FROM inserted AS articles \
                 JOIN authors ON authors.id = articles.author_id"
            ),
            remove: "DELETE FROM articles WHERE id = $1",
            save: "UPDATE articles SET title = $2, body = $3, cover = $4, published = $5, \
                   price = $6, reading_minutes = $7, status = $8, created_at = $9, \
                   author_id = $10 WHERE id = $1",
        }
    }
}
