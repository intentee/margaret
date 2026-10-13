use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::constants::COLUMN_DEFAULTS;
use margaret_model_codegen::constants::ON_DELETE_ACTIONS;
use margaret_model_codegen::index_kind::IndexKind;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_foreign_key::ModelForeignKey;
use margaret_model_codegen::model_index::ModelIndex;
use margaret_model_codegen::resolved_column::ResolvedColumn;

use crate::column_check_tokens::column_check_tokens;
use crate::column_type_tokens::column_type_tokens;
use crate::constants::TABLE_MODULE_NAME;
use crate::model_module_path::model_module_path;
use crate::string_slice_tokens::string_slice_tokens;
use crate::table_namespace_tokens::table_namespace_tokens;

fn column_tokens(column: &ResolvedColumn) -> TokenStream {
    let checks = column.checks.iter().map(column_check_tokens);
    let column_type = column_type_tokens(column.column_type);
    let default = COLUMN_DEFAULTS.tokens(column.default);
    let name = Literal::string(&column.name);
    let nullable = column.nullable;

    quote! {
        margaret::framework::model::column::Column {
            checks: &[#(#checks),*],
            column_type: #column_type,
            default: #default,
            name: #name,
            nullable: #nullable,
        }
    }
}

fn foreign_key_tokens(foreign_key: &ModelForeignKey) -> TokenStream {
    let columns = string_slice_tokens(&foreign_key.columns);
    let on_delete = ON_DELETE_ACTIONS.tokens(foreign_key.on_delete);
    let references_columns = string_slice_tokens(&foreign_key.references_columns);
    let references_table = Literal::string(&foreign_key.references_table);

    quote! {
        margaret::framework::model::foreign_key::ForeignKey {
            columns: #columns,
            on_delete: #on_delete,
            references_columns: #references_columns,
            references_table: #references_table,
        }
    }
}

fn index_tokens(index: &ModelIndex) -> TokenStream {
    let columns = string_slice_tokens(&index.column_names());
    let name = Literal::string(&index.name);

    quote! {
        margaret::framework::model::index::Index {
            columns: #columns,
            name: #name,
        }
    }
}

fn unique_constraint_tokens(index: &ModelIndex) -> TokenStream {
    let columns = string_slice_tokens(&index.column_names());

    quote! {
        margaret::framework::model::unique_constraint::UniqueConstraint {
            columns: #columns,
        }
    }
}

#[must_use]
pub fn render_table(model: &Model) -> GeneratedModuleTokens {
    let columns = model.columns().map(column_tokens);
    let foreign_keys = model.foreign_keys.iter().map(foreign_key_tokens);
    let indexes = model.indexes.of_kind(IndexKind::Plain).map(index_tokens);
    let name = Literal::string(&model.table);
    let namespace = table_namespace_tokens(model.namespace);
    let primary_key = string_slice_tokens(&model.indexes.primary_key.column_names());
    let unique_constraints = model
        .indexes
        .of_kind(IndexKind::Unique)
        .map(unique_constraint_tokens);

    GeneratedModuleTokens::new(
        model_module_path(model).child(TABLE_MODULE_NAME),
        quote! {
            pub const TABLE: margaret::framework::model::table::Table =
                margaret::framework::model::table::Table {
                    columns: &[#(#columns),*],
                    foreign_keys: &[#(#foreign_keys),*],
                    indexes: &[#(#indexes),*],
                    name: #name,
                    namespace: #namespace,
                    primary_key: #primary_key,
                    unique_constraints: &[#(#unique_constraints),*],
                };
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model_codegen::models::models;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_table;

    const KEY: &str = "use margaret::framework::active_record::key::Key;\n";

    fn table_source(lib_source: &str, table: &str, namespace: TableNamespace) -> String {
        let indexed = IndexedSource::new(lib_source);
        let models = models(&indexed.index, namespace).expect("the models resolve");

        models
            .iter()
            .find(|model| model.table == table)
            .map(render_table)
            .expect("the table renders")
            .to_source()
            .split_whitespace()
            .collect()
    }

    fn application_table(lib_source: &str, table: &str) -> String {
        table_source(lib_source, table, TableNamespace::Application)
    }

    fn value_table(value_type: &str) -> String {
        application_table(
            &format!(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {{\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    value: {value_type},\n}}\n"
            ),
            "values",
        )
    }

    fn with_author(referencing: &str) -> String {
        format!(
            "use margaret::framework::macros::model;\n{KEY}#[model(table = \"authors\")]\nstruct Author {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n\n{referencing}"
        )
    }

    fn on_delete_table(action: &str, key_type: &str) -> String {
        application_table(
            &with_author(&format!(
                "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::{action})]\n    #[index]\n    author: {key_type},\n}}\n"
            )),
            "articles",
        )
    }

    #[test]
    fn places_the_table_in_the_module_of_its_model() {
        let indexed = IndexedSource::new(
            "use margaret::framework::macros::model;\nmod blog {\nuse margaret::framework::macros::model;\n\n    #[model(table = \"posts\")]\n    pub struct Post {\n        #[column(primary_key)]\n        pub id: i64,\n    }\n}\n",
        );

        assert_eq!(
            render_table(
                &models(&indexed.index, TableNamespace::Application).expect("the models resolve")
                    [0]
            )
            .path()
            .to_string(),
            "models/blog_post/table"
        );
    }

    #[test]
    fn renders_the_table_name_and_primary_key() {
        let source = value_table("String");

        assert!(source.contains("pubconstTABLE:margaret::framework::model::table::Table="));
        assert!(source.contains("name:\"values\""));
        assert!(source.contains("primary_key:&[\"id\"]"));
    }

    #[test]
    fn renders_the_application_namespace_so_the_table_is_created_outside_the_framework_schema() {
        assert!(value_table("String").contains(
            "namespace:margaret::framework::sql_identifier::table_namespace::TableNamespace::Application"
        ));
    }

    #[test]
    fn renders_the_framework_namespace_so_the_table_is_created_in_the_framework_schema() {
        assert!(
            table_source(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key)]\n    id: i64,\n}\n",
                "values",
                TableNamespace::Framework,
            )
            .contains(
                "namespace:margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework"
            )
        );
    }

    #[test]
    fn renders_a_uuid_primary_key_with_its_declared_v7_default() {
        assert!(
            application_table(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key, default = margaret::framework::model::column_default::ColumnDefault::UuidV7)]\n    id: uuid::Uuid,\n}\n",
                "values"
            )
            .contains(
                "column_type:margaret::framework::model::column_type::ColumnType::Uuid,default:margaret::framework::model::column_default::ColumnDefault::UuidV7,name:\"id\",nullable:false"
            )
        );
    }

    #[test]
    fn renders_a_column_without_a_default_as_not_set_so_inserts_must_supply_it() {
        assert!(value_table("String").contains(
            "column_type:margaret::framework::model::column_type::ColumnType::Text,default:margaret::framework::model::column_default::ColumnDefault::NotSet,name:\"value\",nullable:false"
        ));
    }

    #[test]
    fn renders_an_option_field_as_a_nullable_column_so_none_can_be_stored() {
        assert!(value_table("Option<String>").contains("name:\"value\",nullable:true"));
    }

    #[test]
    fn renders_an_i64_field_as_a_big_int_column_of_the_same_eight_byte_range() {
        assert!(
            value_table("i64").contains(
                "column_type:margaret::framework::model::column_type::ColumnType::BigInt"
            )
        );
    }

    #[test]
    fn renders_a_bool_field_as_a_boolean_column_matching_its_rust_type() {
        assert!(
            value_table("bool").contains(
                "column_type:margaret::framework::model::column_type::ColumnType::Boolean"
            )
        );
    }

    #[test]
    fn renders_a_byte_vector_field_as_a_bytea_column_to_hold_raw_bytes() {
        assert!(
            value_table("Vec<u8>")
                .contains("column_type:margaret::framework::model::column_type::ColumnType::Bytea")
        );
    }

    #[test]
    fn renders_an_f64_field_as_a_double_precision_column_of_the_same_precision() {
        assert!(value_table("f64").contains(
            "column_type:margaret::framework::model::column_type::ColumnType::DoublePrecision"
        ));
    }

    #[test]
    fn renders_an_i32_field_as_an_integer_column_of_the_same_four_byte_range() {
        assert!(
            value_table("i32").contains(
                "column_type:margaret::framework::model::column_type::ColumnType::Integer"
            )
        );
    }

    #[test]
    fn renders_an_f32_field_as_a_real_column_of_the_same_single_precision() {
        assert!(
            value_table("f32")
                .contains("column_type:margaret::framework::model::column_type::ColumnType::Real")
        );
    }

    #[test]
    fn renders_a_utc_datetime_field_as_a_timestamptz_column_storing_an_absolute_instant() {
        assert!(value_table("chrono::DateTime<chrono::Utc>").contains(
            "column_type:margaret::framework::model::column_type::ColumnType::Timestamptz"
        ));
    }

    #[test]
    fn renders_a_decimal_field_as_a_numeric_column_with_its_declared_precision_and_scale() {
        assert!(
            application_table(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    value: rust_decimal::Decimal,\n}\n",
                "values"
            )
            .contains(
                "column_type:margaret::framework::model::column_type::ColumnType::Numeric{precision:12u32,scale:2u32,}"
            )
        );
    }

    #[test]
    fn renders_a_byte_length_check_so_postgres_rejects_bytes_of_another_length() {
        assert!(
            application_table(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key, byte_length = 32)]\n    hash: Vec<u8>,\n}\n",
                "values"
            )
            .contains(
                "checks:&[margaret::framework::model::column_check::ColumnCheck{name:\"values_hash_byte_length\",predicate:margaret::framework::model::check_predicate::CheckPredicate::ByteLength{length:32u32,},}]"
            )
        );
    }

    #[test]
    fn renders_a_minimum_check_so_postgres_rejects_a_value_below_the_declared_minimum() {
        assert!(
            application_table(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key, minimum = 0)]\n    amount: i64,\n}\n",
                "values"
            )
            .contains(
                "checks:&[margaret::framework::model::column_check::ColumnCheck{name:\"values_amount_minimum\",predicate:margaret::framework::model::check_predicate::CheckPredicate::Minimum{minimum:0u32,},}]"
            )
        );
    }

    #[test]
    fn renders_a_foreign_key_that_takes_no_action() {
        assert!(
            application_table(
                &with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    author: Key<Author>,\n}\n"
                ),
                "articles"
            )
            .contains(
                "foreign_keys:&[margaret::framework::model::foreign_key::ForeignKey{columns:&[\"author_id\"],on_delete:margaret::framework::model::on_delete::OnDelete::NoAction,references_columns:&[\"id\"],references_table:\"authors\",}]"
            )
        );
    }

    #[test]
    fn renders_a_cascading_foreign_key_so_deleting_a_parent_deletes_its_children() {
        assert!(
            on_delete_table("Cascade", "Key<Author>")
                .contains("on_delete:margaret::framework::model::on_delete::OnDelete::Cascade")
        );
    }

    #[test]
    fn renders_a_restricting_foreign_key_so_a_referenced_parent_cannot_be_deleted() {
        assert!(
            on_delete_table("Restrict", "Key<Author>")
                .contains("on_delete:margaret::framework::model::on_delete::OnDelete::Restrict")
        );
    }

    #[test]
    fn renders_a_foreign_key_that_sets_null() {
        assert!(
            on_delete_table("SetNull", "Option<Key<Author>>")
                .contains("on_delete:margaret::framework::model::on_delete::OnDelete::SetNull")
        );
    }

    #[test]
    fn renders_an_indexed_field_as_a_plain_index_so_lookups_by_it_avoid_a_table_scan() {
        assert!(value_table_with_index().contains(
            "indexes:&[margaret::framework::model::index::Index{columns:&[\"value\"],name:\"values_value_index\",}]"
        ));
    }

    fn value_table_with_index() -> String {
        application_table(
            "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index]\n    value: String,\n}\n",
            "values",
        )
    }

    #[test]
    fn renders_a_unique_column_as_a_unique_constraint_so_postgres_rejects_duplicate_values() {
        assert!(
            application_table(
                "use margaret::framework::macros::model;\n#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key)]\n    id: i64,\n    #[column(unique)]\n    value: String,\n}\n",
                "values"
            )
            .contains(
                "unique_constraints:&[margaret::framework::model::unique_constraint::UniqueConstraint{columns:&[\"value\"],}]"
            )
        );
    }
}
