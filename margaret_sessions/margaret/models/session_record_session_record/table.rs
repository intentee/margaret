pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[
                margaret::framework::model::column_check::ColumnCheck {
                    name: "sessions_secret_byte_length",
                    predicate: margaret::framework::model::check_predicate::CheckPredicate::ByteLength {
                        length: 32u32,
                    },
                },
            ],
            column_type: margaret::framework::model::column_type::ColumnType::Bytea,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "secret",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Timestamptz,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "authenticated_at",
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
            column_type: margaret::framework::model::column_type::ColumnType::Uuid,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "id",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Uuid,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "subject",
            nullable: false,
        },
    ],
    foreign_keys: &[],
    indexes: &[
        margaret::framework::model::index::Index {
            columns: &["expires_at"],
            name: "sessions_expires_at_index",
        },
    ],
    name: "sessions",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["secret"],
    unique_constraints: &[
        margaret::framework::model::unique_constraint::UniqueConstraint {
            columns: &["id"],
        },
    ],
};
