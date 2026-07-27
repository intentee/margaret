pub mod messages;
pub mod responders;

use std::sync::Arc;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;
use tokio::sync::Mutex;

use crate::english_greeter::EnglishGreeter;
use crate::forms::get_articles_form::GetArticlesForm;
use crate::margaret::routes::Routes;
use crate::models::article::Article;
use crate::models::user::User;

#[websocket_session(path = "/storyboard/{topic}/{article}", server = "public")]
pub struct StoryboardSession {
    article_title: String,
    board_url: String,
    greeter: Arc<EnglishGreeter>,
    topic: String,
    turns: Mutex<Vec<String>>,
    viewer: Option<User>,
}

impl StoryboardSession {
    #[build_for_session]
    pub fn build_for_session(
        greeter: Arc<EnglishGreeter>,
        #[route_parameter(from = "topic")] topic: String,
        #[route_parameter(from = "article")] article: Article,
        #[form_request(from = Query)] filters: GetArticlesForm,
        #[authenticated_user] viewer: Option<User>,
        routes: &Routes,
    ) -> anyhow::Result<Self> {
        Ok({
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
        })
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
