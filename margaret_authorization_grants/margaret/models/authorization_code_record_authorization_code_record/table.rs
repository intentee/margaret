pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[
                margaret::framework::model::column_check::ColumnCheck {
                    name: "authorization_codes_code_byte_length",
                    predicate: margaret::framework::model::check_predicate::CheckPredicate::ByteLength {
                        length: 32u32,
                    },
                },
            ],
            column_type: margaret::framework::model::column_type::ColumnType::Bytea,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "code",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::BigInt,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "expires_at",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Text,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "grant",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Uuid,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "redeemed_by",
            nullable: true,
        },
    ],
    foreign_keys: &[],
    indexes: &[
        margaret::framework::model::index::Index {
            columns: &["expires_at"],
            name: "authorization_codes_expires_at_index",
        },
    ],
    name: "authorization_codes",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["code"],
    unique_constraints: &[],
};
