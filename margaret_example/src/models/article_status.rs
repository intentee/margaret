#[derive(Clone)]
pub enum ArticleStatus {
    Draft,
    Published,
}

impl ArticleStatus {
    #[must_use]
    pub fn as_text(&self) -> &'static str {
        match self {
            ArticleStatus::Draft => "Draft",
            ArticleStatus::Published => "Published",
        }
    }

    #[must_use]
    pub fn from_text(text: &str) -> Option<Self> {
        match text {
            "Draft" => Some(ArticleStatus::Draft),
            "Published" => Some(ArticleStatus::Published),
            _ => None,
        }
    }
}
