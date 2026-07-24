use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

#[derive(Clone, Copy, Debug)]
pub enum ObjectKind {
    ForeignKeyConstraint,
    Index,
    PrimaryKeyConstraint,
    PrimaryKeyIndex,
    Table,
    UniqueConstraint,
    UniqueIndex,
}

impl Display for ObjectKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        let label = match self {
            ObjectKind::ForeignKeyConstraint => "foreign key constraint",
            ObjectKind::Index => "index",
            ObjectKind::PrimaryKeyConstraint => "primary key constraint",
            ObjectKind::PrimaryKeyIndex => "primary key index",
            ObjectKind::Table => "table",
            ObjectKind::UniqueConstraint => "unique constraint",
            ObjectKind::UniqueIndex => "unique index",
        };

        write!(formatter, "{label}")
    }
}
