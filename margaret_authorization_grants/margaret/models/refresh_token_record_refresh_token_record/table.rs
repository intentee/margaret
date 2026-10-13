pub const TABLE: margaret::framework::model::table::Table = margaret::framework::model::table::Table {
    columns: &[
        margaret::framework::model::column::Column {
            checks: &[
                margaret::framework::model::column_check::ColumnCheck {
                    name: "refresh_tokens_token_byte_length",
                    predicate: margaret::framework::model::check_predicate::CheckPredicate::ByteLength {
                        length: 32u32,
                    },
                },
            ],
            column_type: margaret::framework::model::column_type::ColumnType::Bytea,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "token",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Uuid,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "family_id",
            nullable: false,
        },
        margaret::framework::model::column::Column {
            checks: &[],
            column_type: margaret::framework::model::column_type::ColumnType::Boolean,
            default: margaret::framework::model::column_default::ColumnDefault::NotSet,
            name: "current",
            nullable: false,
        },
    ],
    foreign_keys: &[
        margaret::framework::model::foreign_key::ForeignKey {
            columns: &["family_id"],
            on_delete: margaret::framework::model::on_delete::OnDelete::Cascade,
            references_columns: &["id"],
            references_table: "refresh_families",
        },
    ],
    indexes: &[
        margaret::framework::model::index::Index {
            columns: &["family_id"],
            name: "refresh_tokens_family_id_index",
        },
    ],
    name: "refresh_tokens",
    namespace: margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework,
    primary_key: &["token"],
    unique_constraints: &[],
};
