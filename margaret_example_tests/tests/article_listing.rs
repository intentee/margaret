use margaret_example::featured_article_id::FEATURED_ARTICLE_ID;
use margaret_example::models::article::Article;
use margaret_example_tests::seeded_blog::seeded_blog;

fn titles(articles: &[Article]) -> Vec<&str> {
    articles
        .iter()
        .map(|article| article.title.as_str())
        .collect()
}

#[tokio::test]
async fn lists_the_seeded_articles_in_id_order() {
    let blog = seeded_blog().await;
    let listed = Article::listed(&blog.database, None)
        .await
        .expect("the articles are listed");

    assert_eq!(listed[0].id, FEATURED_ARTICLE_ID);
    assert_eq!(
        titles(&listed),
        vec!["Shipping Margaret", "Milo's draft", "Mona's draft"]
    );
}

#[tokio::test]
async fn lists_the_articles_of_one_author() {
    let blog = seeded_blog().await;

    assert_eq!(
        titles(
            &Article::listed(&blog.database, Some("Milo".to_string()))
                .await
                .expect("the articles are listed")
        ),
        vec!["Shipping Margaret", "Milo's draft"]
    );
}

#[tokio::test]
async fn lists_no_articles_of_an_unknown_author() {
    let blog = seeded_blog().await;

    assert!(
        Article::listed(&blog.database, Some("Nobody".to_string()))
            .await
            .expect("the articles are listed")
            .is_empty()
    );
}
