pub enum ColumnType {
    BigInt,
    Boolean,
    Integer,
    Text,
    Timestamptz,
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
            ColumnType::Timestamptz => "TIMESTAMPTZ",
            ColumnType::Uuid => "UUID",
        }
    }
}
