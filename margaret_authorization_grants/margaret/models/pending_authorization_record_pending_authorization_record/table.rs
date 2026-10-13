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
            column_type: margaret::framework::model::column_type::ColumnType::Text,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "state",
            nullable: true,
        },
    ],
    foreign_keys: &[],
    indexes: &[
        margaret::framework::model::index::Index {
            columns: &["expires_at"],
            name: "pending_authorizations_expires_at_index",
        },
    ],
    name: "pending_authorizations",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["id"],
    unique_constraints: &[],
};
