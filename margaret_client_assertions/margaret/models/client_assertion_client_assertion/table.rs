pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[
                margaret::framework::model::column_check::ColumnCheck {
                    name: "client_assertions_key_byte_length",
                    predicate: margaret::framework::model::check_predicate::CheckPredicate::ByteLength {
                        length: 32u32,
                    },
                },
            ],
            column_type: margaret::framework::model::column_type::ColumnType::Bytea,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "key",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::BigInt,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "expires_at",
            nullable: false,
        },
    ],
    foreign_keys: &[],
    indexes: &[
        margaret::framework::model::index::Index {
            columns: &["expires_at"],
            name: "client_assertions_expires_at_index",
        },
    ],
    name: "client_assertions",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["key"],
    unique_constraints: &[],
};
