#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TablePrivilege {
    Delete,
    Insert,
    Select,
    Update,
}

impl TablePrivilege {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Insert => "INSERT",
            Self::Select => "SELECT",
            Self::Update => "UPDATE",
        }
    }
}
