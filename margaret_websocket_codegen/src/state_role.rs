#[derive(Debug)]
pub enum StateRole {
    Entry { server: String, path: String },
    Intermediate,
    Terminal,
}

impl StateRole {
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal)
    }
}
