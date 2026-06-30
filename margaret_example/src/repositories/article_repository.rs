use async_trait::async_trait;

use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_macros::provides_route_parameter;
use margaret_macros::singleton;

use crate::models::article::Article;

fn published_member_article() -> Article {
    Article {
        id: "100".to_string(),
        title: "Shipping Margaret".to_string(),
        author_id: "3".to_string(),
        body: "A public note from Milo.".to_string(),
        published: true,
    }
}

fn member_draft() -> Article {
    Article {
        id: "101".to_string(),
        title: "Milo's draft".to_string(),
        author_id: "3".to_string(),
        body: "An unpublished draft from Milo.".to_string(),
        published: false,
    }
}

fn moderator_draft() -> Article {
    Article {
        id: "102".to_string(),
        title: "Mona's draft".to_string(),
        author_id: "2".to_string(),
        body: "An unpublished draft from Mona.".to_string(),
        published: false,
    }
}

#[singleton]
#[provides_route_parameter]
pub struct ArticleRepository;

impl ArticleRepository {
    pub fn find_article_by_id(&self, id: &str) -> Option<Article> {
        match id {
            "100" => Some(published_member_article()),
            "101" => Some(member_draft()),
            "102" => Some(moderator_draft()),
            _ => None,
        }
    }

    pub fn all(&self) -> Vec<Article> {
        vec![
            published_member_article(),
            member_draft(),
            moderator_draft(),
        ]
    }
}

#[async_trait]
impl HttpRouteParameterBinder for ArticleRepository {
    type Model = Article;

    async fn bind(&self, value: String) -> Option<Article> {
        self.find_article_by_id(&value)
    }
}
