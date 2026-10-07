use margaret_model::column::Column;
use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::foreign_key::ForeignKey;
use margaret_model::index::Index;
use margaret_model::on_delete::OnDelete;
use margaret_model::table::Table;

use crate::authorization_codes_table::AUTHORIZATION_CODES_TABLE;
use crate::client_assertions_table::CLIENT_ASSERTIONS_TABLE;
use crate::pending_authorizations_table::PENDING_AUTHORIZATIONS_TABLE;
use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;
use crate::refresh_tokens_table::REFRESH_TOKENS_TABLE;

fn column(name: &str, column_type: ColumnType, nullable: bool) -> Column {
    Column {
        checks: Vec::new(),
        column_type,
        default: ColumnDefault::NotSet,
        name: name.to_string(),
        nullable,
    }
}

fn index(table: &str, column: &str) -> Index {
    Index {
        columns: vec![column.to_string()],
        name: format!("{table}-{column}"),
    }
}

fn table(name: &str, key: &str, columns: Vec<Column>, foreign_keys: Vec<ForeignKey>) -> Table {
    Table {
        columns,
        foreign_keys,
        indexes: vec![index(name, "expires_at")],
        name: name.to_string(),
        primary_key: vec![key.to_string()],
        unique_constraints: Vec::new(),
    }
}

#[must_use]
pub fn provider_state_tables() -> Vec<Table> {
    let mut refresh_tokens = table(
        REFRESH_TOKENS_TABLE,
        "digest",
        vec![
            column("digest", ColumnType::Bytea, false),
            column("family", ColumnType::Uuid, false),
            column("superseded", ColumnType::Boolean, false),
            column("expires_at", ColumnType::Timestamptz, false),
        ],
        vec![ForeignKey {
            columns: vec!["family".to_string()],
            on_delete: OnDelete::Cascade,
            references_columns: vec!["id".to_string()],
            references_table: REFRESH_FAMILIES_TABLE.to_string(),
        }],
    );

    refresh_tokens
        .indexes
        .push(index(REFRESH_TOKENS_TABLE, "family"));

    vec![
        Table {
            columns: vec![
                column("client_id", ColumnType::Text, false),
                column("digest", ColumnType::Bytea, false),
                column("expires_at", ColumnType::Timestamptz, false),
            ],
            foreign_keys: Vec::new(),
            indexes: vec![index(CLIENT_ASSERTIONS_TABLE, "expires_at")],
            name: CLIENT_ASSERTIONS_TABLE.to_string(),
            primary_key: vec!["client_id".to_string(), "digest".to_string()],
            unique_constraints: Vec::new(),
        },
        table(
            AUTHORIZATION_CODES_TABLE,
            "digest",
            vec![
                column("digest", ColumnType::Bytea, false),
                column("grant_document", ColumnType::Text, false),
                column("redeemed_family", ColumnType::Uuid, true),
                column("expires_at", ColumnType::Timestamptz, false),
            ],
            Vec::new(),
        ),
        table(
            PENDING_AUTHORIZATIONS_TABLE,
            "id",
            vec![
                column("id", ColumnType::Uuid, false),
                column("pending_document", ColumnType::Text, false),
                column("expires_at", ColumnType::Timestamptz, false),
            ],
            Vec::new(),
        ),
        table(
            REFRESH_FAMILIES_TABLE,
            "id",
            vec![
                column("id", ColumnType::Uuid, false),
                column("grant_document", ColumnType::Text, false),
                column("expires_at", ColumnType::Timestamptz, false),
            ],
            Vec::new(),
        ),
        refresh_tokens,
    ]
}
