pub mod messages;
pub mod responders;

use std::sync::Arc;

use margaret_macros::build_for_session;
use margaret_macros::middleware;
use margaret_macros::websocket_session;
use tokio::sync::Mutex;

use crate::forms::get_articles_form::GetArticlesForm;
use crate::greeter::Greeter;
use crate::margaret::routes::Routes;
use crate::models::article::Article;
use crate::models::user::User;

#[middleware(logged)]
#[websocket_session(path = "/storyboard/{topic}/{article}", server = "public")]
pub struct StoryboardSession {
    article_title: String,
    board_url: String,
    greeter: Arc<dyn Greeter>,
    topic: String,
    turns: Mutex<Vec<String>>,
    viewer: Option<User>,
}

impl StoryboardSession {
    #[build_for_session]
    #[must_use]
    pub fn build_for_session(
        greeter: Arc<dyn Greeter>,
        #[route_parameter(from = "topic")] topic: String,
        #[route_parameter(from = "article")] article: Article,
        #[form_request(from = Query)] filters: GetArticlesForm,
        #[authenticated_user] viewer: Option<User>,
        routes: &Routes,
    ) -> Self {
        let board_url = routes.public.get_feed.url();
        let article_title = match filters.author {
            Some(author) => format!("{} by {author}", article.title),
            None => article.title,
        };

        Self {
            article_title,
            board_url,
            greeter,
            topic,
            turns: Mutex::new(Vec::new()),
            viewer,
        }
    }

    #[must_use]
    pub fn article_title(&self) -> &str {
        &self.article_title
    }

    #[must_use]
    pub fn board_url(&self) -> &str {
        &self.board_url
    }

    #[must_use]
    pub fn greeting(&self) -> String {
        self.greeter.greet()
    }

    pub async fn record(&self, turn: String) {
        self.turns.lock().await.push(turn);
    }

    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub async fn turn_count(&self) -> usize {
        self.turns.lock().await.len()
    }

    #[must_use]
    pub fn viewer_name(&self) -> Option<&str> {
        self.viewer.as_ref().map(|viewer| viewer.name.as_str())
    }
}
