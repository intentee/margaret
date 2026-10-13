use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;

fn attempted(loaded: ArticleWithAuthor) -> usize {
    loaded.translations.records.len()
}

fn main() {
    drop(attempted);
}
