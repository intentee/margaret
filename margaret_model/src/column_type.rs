pub enum ColumnType {
    BigInt,
    Boolean,
    Integer,
    Text,
    Uuid,
}

impl ColumnType {
    #[must_use]
    pub fn render(&self) -> &'static str {
        match self {
            ColumnType::BigInt => "BIGINT",
            ColumnType::Boolean => "BOOLEAN",
            ColumnType::Integer => "INTEGER",
            ColumnType::Text => "TEXT",
            ColumnType::Uuid => "UUID",
        }
    }
}
