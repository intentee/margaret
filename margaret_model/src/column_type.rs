#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColumnType {
    BigInt,
    Boolean,
    Bytea,
    DoublePrecision,
    Integer,
    Numeric { precision: u32, scale: u32 },
    Real,
    Text,
    Timestamptz,
    Uuid,
}

impl ColumnType {
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            ColumnType::BigInt => "BIGINT".to_string(),
            ColumnType::Boolean => "BOOLEAN".to_string(),
            ColumnType::Bytea => "BYTEA".to_string(),
            ColumnType::DoublePrecision => "DOUBLE PRECISION".to_string(),
            ColumnType::Integer => "INTEGER".to_string(),
            ColumnType::Numeric { precision, scale } => format!("NUMERIC({precision}, {scale})"),
            ColumnType::Real => "REAL".to_string(),
            ColumnType::Text => "TEXT".to_string(),
            ColumnType::Timestamptz => "TIMESTAMPTZ".to_string(),
            ColumnType::Uuid => "UUID".to_string(),
        }
    }
}
