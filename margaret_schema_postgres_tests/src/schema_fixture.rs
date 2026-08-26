use margaret_model::check_predicate::CheckPredicate;
use margaret_model::column::Column;
use margaret_model::column_check::ColumnCheck;
use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::foreign_key::ForeignKey;
use margaret_model::index::Index;
use margaret_model::on_delete::OnDelete;
use margaret_model::schema::Schema;
use margaret_model::table::Table;
use margaret_model::unique_constraint::UniqueConstraint;

fn column(name: &str, column_type: ColumnType, default: ColumnDefault, nullable: bool) -> Column {
    Column {
        checks: Vec::new(),
        column_type,
        default,
        name: name.to_string(),
        nullable,
    }
}

fn checked_column(name: &str, column_type: ColumnType, predicate: CheckPredicate) -> Column {
    Column {
        checks: vec![ColumnCheck {
            name: format!("fragment_metadata_{name}_check"),
            predicate,
        }],
        column_type,
        default: ColumnDefault::NotSet,
        name: name.to_string(),
        nullable: false,
    }
}

fn author_table() -> Table {
    Table {
        columns: vec![
            column("id", ColumnType::Uuid, ColumnDefault::UuidV7, false),
            column("name", ColumnType::Text, ColumnDefault::NotSet, false),
            column(
                "is_active",
                ColumnType::Boolean,
                ColumnDefault::NotSet,
                false,
            ),
            column(
                "joined_at",
                ColumnType::Timestamptz,
                ColumnDefault::NotSet,
                false,
            ),
            column("bio", ColumnType::Text, ColumnDefault::NotSet, true),
        ],
        foreign_keys: Vec::new(),
        indexes: vec![Index {
            columns: vec!["is_active".to_string(), "joined_at".to_string()],
            name: "authors_active_joined".to_string(),
        }],
        name: "authors".to_string(),
        primary_key: vec!["id".to_string()],
        unique_constraints: vec![UniqueConstraint {
            columns: vec!["name".to_string()],
        }],
    }
}

fn article_table() -> Table {
    Table {
        columns: vec![
            column("id", ColumnType::Uuid, ColumnDefault::UuidV7, false),
            column("title", ColumnType::Text, ColumnDefault::NotSet, false),
            column("body", ColumnType::Text, ColumnDefault::NotSet, false),
            column("cover", ColumnType::Bytea, ColumnDefault::NotSet, true),
            column(
                "published",
                ColumnType::Boolean,
                ColumnDefault::NotSet,
                false,
            ),
            column("status", ColumnType::Text, ColumnDefault::NotSet, false),
            column(
                "created_at",
                ColumnType::Timestamptz,
                ColumnDefault::NotSet,
                false,
            ),
            column("author_id", ColumnType::Uuid, ColumnDefault::NotSet, false),
        ],
        foreign_keys: vec![ForeignKey {
            columns: vec!["author_id".to_string()],
            on_delete: OnDelete::Cascade,
            references_columns: vec!["id".to_string()],
            references_table: "authors".to_string(),
        }],
        indexes: vec![
            Index {
                columns: vec!["author_id".to_string()],
                name: "articles_author_id_index".to_string(),
            },
            Index {
                columns: vec!["created_at".to_string()],
                name: "articles_created_at_index".to_string(),
            },
        ],
        name: "articles".to_string(),
        primary_key: vec!["id".to_string()],
        unique_constraints: Vec::new(),
    }
}

fn line_item_table() -> Table {
    Table {
        columns: vec![
            column("id", ColumnType::Uuid, ColumnDefault::UuidV7, false),
            column(
                "price",
                ColumnType::Numeric {
                    precision: 12,
                    scale: 2,
                },
                ColumnDefault::NotSet,
                false,
            ),
            column(
                "discount",
                ColumnType::Numeric {
                    precision: 5,
                    scale: 4,
                },
                ColumnDefault::NotSet,
                true,
            ),
            column("weight_kg", ColumnType::Real, ColumnDefault::NotSet, false),
            column(
                "volume_litres",
                ColumnType::DoublePrecision,
                ColumnDefault::NotSet,
                false,
            ),
        ],
        foreign_keys: Vec::new(),
        indexes: Vec::new(),
        name: "line_items".to_string(),
        primary_key: vec!["id".to_string()],
        unique_constraints: Vec::new(),
    }
}

fn fragment_metadata_table() -> Table {
    Table {
        columns: vec![
            column("partition", ColumnType::Uuid, ColumnDefault::NotSet, false),
            checked_column(
                "hash",
                ColumnType::Bytea,
                CheckPredicate::ByteLength { length: 32 },
            ),
            checked_column(
                "size_payload",
                ColumnType::BigInt,
                CheckPredicate::Minimum { minimum: 0 },
            ),
        ],
        foreign_keys: Vec::new(),
        indexes: Vec::new(),
        name: "fragment_metadata".to_string(),
        primary_key: vec!["partition".to_string(), "hash".to_string()],
        unique_constraints: Vec::new(),
    }
}

fn fragment_table() -> Table {
    Table {
        columns: vec![
            column("partition", ColumnType::Uuid, ColumnDefault::NotSet, false),
            column("hash", ColumnType::Bytea, ColumnDefault::NotSet, false),
            column("context", ColumnType::Uuid, ColumnDefault::NotSet, false),
        ],
        foreign_keys: vec![ForeignKey {
            columns: vec!["partition".to_string(), "hash".to_string()],
            on_delete: OnDelete::Cascade,
            references_columns: vec!["partition".to_string(), "hash".to_string()],
            references_table: "fragment_metadata".to_string(),
        }],
        indexes: Vec::new(),
        name: "fragment".to_string(),
        primary_key: vec![
            "partition".to_string(),
            "hash".to_string(),
            "context".to_string(),
        ],
        unique_constraints: vec![UniqueConstraint {
            columns: vec!["hash".to_string(), "context".to_string()],
        }],
    }
}

#[must_use]
pub fn schema_fixture() -> Schema {
    Schema {
        tables: vec![
            author_table(),
            article_table(),
            line_item_table(),
            fragment_metadata_table(),
            fragment_table(),
        ],
    }
}
