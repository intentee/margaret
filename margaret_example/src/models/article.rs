#[derive(Clone)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author_id: String,
    pub body: String,
    pub published: bool,
}

impl Article {
    pub fn is_owned_by(&self, user_id: &str) -> bool {
        self.author_id == user_id
    }
}
