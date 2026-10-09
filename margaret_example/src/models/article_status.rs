#[derive(Clone)]
pub enum ArticleStatus {
    Draft,
    Published,
}

impl ArticleStatus {
    #[must_use]
    pub fn from_stored(stored: &str) -> Option<Self> {
        match stored {
            "Draft" => Some(Self::Draft),
            "Published" => Some(Self::Published),
            _ => None,
        }
    }

    #[must_use]
    pub fn stored(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Published => "Published",
        }
    }
}
