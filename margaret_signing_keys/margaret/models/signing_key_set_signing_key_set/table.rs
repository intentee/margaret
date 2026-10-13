pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Text,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "name",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::BigInt,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "generation",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Text,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "document",
            nullable: false,
        },
    ],
    foreign_keys: &[],
    indexes: &[],
    name: "signing_key_sets",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["name"],
    unique_constraints: &[],
};
