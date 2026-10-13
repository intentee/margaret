use margaret_active_record_tests::models::article::Article;

fn attempted(article: Article) -> String {
    article.author.name
}

fn main() {
    drop(attempted);
}
