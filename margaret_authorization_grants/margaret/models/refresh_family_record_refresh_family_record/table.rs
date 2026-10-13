pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Uuid,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "id",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Timestamptz,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "auth_time",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Text,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "client_id",
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
            name: "scopes",
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
            name: "refresh_families_expires_at_index",
        },
    ],
    name: "refresh_families",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["id"],
    unique_constraints: &[],
};
