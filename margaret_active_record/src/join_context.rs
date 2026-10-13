use margaret_sql::condition::Condition;
use margaret_sql::join::Join;
use margaret_sql::table_source::TableSource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JoinContext {
    Optional,
    Required,
}

impl JoinContext {
    pub(crate) fn join(self, on: Condition, source: TableSource) -> Join {
        match self {
            Self::Optional => Join::Left { on, source },
            Self::Required => Join::Inner { on, source },
        }
    }

    pub(crate) fn within(self, parent: Self) -> Self {
        match parent {
            Self::Optional => Self::Optional,
            Self::Required => self,
        }
    }
}
