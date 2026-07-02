#[derive(Clone)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author_id: String,
    pub body: String,
    pub published: bool,
}
