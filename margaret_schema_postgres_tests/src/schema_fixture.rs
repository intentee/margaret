use margaret_model::column::Column;
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
        column_type,
        default,
        name: name.to_string(),
        nullable,
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
            column: "author_id".to_string(),
            on_delete: OnDelete::Cascade,
            references_column: "id".to_string(),
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

#[must_use]
pub fn schema_fixture() -> Schema {
    Schema {
        tables: vec![author_table(), article_table()],
    }
}
