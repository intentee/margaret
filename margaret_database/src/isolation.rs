use tokio_postgres::IsolationLevel;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Isolation {
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

impl Isolation {
    pub(crate) fn level(self) -> IsolationLevel {
        match self {
            Self::ReadCommitted => IsolationLevel::ReadCommitted,
            Self::RepeatableRead => IsolationLevel::RepeatableRead,
            Self::Serializable => IsolationLevel::Serializable,
        }
    }
}
