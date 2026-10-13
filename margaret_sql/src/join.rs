use crate::condition::Condition;
use crate::select::Select;
use crate::table_alias::TableAlias;
use crate::table_source::TableSource;

#[derive(Clone)]
pub enum Join {
    Inner {
        on: Condition,
        source: TableSource,
    },
    Lateral {
        alias: TableAlias,
        select: Box<Select>,
    },
    Left {
        on: Condition,
        source: TableSource,
    },
}
