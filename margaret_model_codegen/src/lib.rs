pub mod has_models;
pub mod inferred_column;
pub mod model;
pub mod model_codegen_error;
pub mod models;
pub mod resolved_column;
pub mod resolved_foreign_key;
pub mod resolved_index;

mod collected_model;
mod column_arguments;
mod deferred_foreign_key;
mod foreign_key_arguments;
mod foreign_key_target;
mod foreign_key_target_column;
mod infer_column_type;
mod model_arguments;
mod option_inner;
mod single_generic_argument;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::has_models::has_models;
    use crate::models::models;

    const ARTICLE: &str = "\
#[model(table = \"articles\")]
struct Article {
    #[column(primary_key, name = \"id\")]
    id: uuid::Uuid,
    #[column]
    title: String,
    #[column(name = \"is_published\")]
    published: bool,
    #[column]
    note: Option<String>,
}
";

    const AUTHOR_MODEL: &str = "\
#[model(table = \"authors\")]
struct Author {
    #[column(primary_key)]
    id: uuid::Uuid,
}
";

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source = directory.path().join("src");

        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn index_of(directory: &Path) -> AttributeIndex {
        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.join("src")))
            .expect("the crate is indexed")
            .build()
    }

    fn error_message(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        models(&index_of(directory.path()))
            .expect_err("the models fail to resolve")
            .to_string()
    }

    fn with_author(referencing: &str) -> String {
        format!("{AUTHOR_MODEL}\n{referencing}")
    }

    #[test]
    fn detects_the_presence_of_models() {
        let with_models = crate_with(ARTICLE);
        let without_models = crate_with("struct Plain;\n");

        assert!(has_models(&index_of(with_models.path())));
        assert!(!has_models(&index_of(without_models.path())));
    }

    #[test]
    fn rejects_a_model_that_is_not_a_struct() {
        assert!(error_message("#[model(table = \"e\")] enum E {}\n").contains("is not a struct"));
    }

    #[test]
    fn rejects_a_model_declared_more_than_once() {
        assert!(
            error_message("#[model(table = \"a\")]\n#[model(table = \"b\")]\nstruct S;\n")
                .contains("duplicate #[model]")
        );
    }

    #[test]
    fn rejects_a_model_without_a_table() {
        assert!(error_message("#[model]\nstruct S;\n").contains("missing the required 'table'"));
    }

    #[test]
    fn rejects_malformed_model_arguments() {
        assert!(
            error_message("#[model(= 5)]\nstruct S;\n")
                .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_non_string_table_name() {
        assert!(
            error_message("#[model(table = 5)]\nstruct S;\n")
                .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_table_name_that_is_not_snake_case() {
        assert!(
            error_message("#[model(table = \"Articles\")]\nstruct S;\n")
                .contains("invalid table name")
        );
    }

    #[test]
    fn rejects_two_models_with_the_same_table() {
        let source = "\
#[model(table = \"t\")]
struct A {
    #[column]
    id: i64,
}

#[model(table = \"t\")]
struct B {
    #[column]
    id: i64,
}
";

        assert!(error_message(source).contains("duplicate table name"));
    }

    #[test]
    fn rejects_a_field_without_a_column_attribute() {
        assert!(
            error_message("#[model(table = \"t\")]\nstruct S {\n    id: i64,\n}\n")
                .contains("has no #[column]")
        );
    }

    #[test]
    fn rejects_a_positional_field_without_a_column_attribute() {
        assert!(
            error_message("#[model(table = \"t\")]\nstruct S(i64);\n").contains("has no #[column]")
        );
    }

    #[test]
    fn rejects_a_positional_column_without_a_name() {
        assert!(
            error_message("#[model(table = \"t\")]\nstruct S(#[column] i64);\n")
                .contains("requires an explicit column name")
        );
    }

    #[test]
    fn rejects_malformed_column_arguments() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(= 5)]\n    id: i64,\n}\n"
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_non_string_column_name() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(name = 5)]\n    id: i64,\n}\n"
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_column_name_that_is_not_snake_case() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(name = \"Id\")]\n    id: i64,\n}\n"
            )
            .contains("invalid column name")
        );
    }

    #[test]
    fn rejects_a_duplicate_column_name() {
        let source = "\
#[model(table = \"t\")]
struct S {
    #[column(name = \"value\")]
    a: i64,
    #[column(name = \"value\")]
    b: i64,
}
";

        assert!(error_message(source).contains("duplicate column name"));
    }

    #[test]
    fn rejects_an_uninferrable_column_type() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    value: u64,\n}\n"
            )
            .contains("cannot be mapped to an SQL type")
        );
    }

    #[test]
    fn rejects_a_field_with_repeated_column_attributes() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[column]\n    id: i64,\n}\n"
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_foreign_key_to_a_composite_primary_key() {
        assert!(
            error_message(
                "#[model(table = \"orders\")]\nstruct Order {\n    #[column(primary_key)]\n    region: String,\n    #[column(primary_key)]\n    number: i64,\n}\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    order: Order,\n}\n",
            )
            .contains("composite primary key")
        );
    }

    #[test]
    fn rejects_a_foreign_key_without_a_column() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[foreign_key]\n    author: Author,\n}\n",
            ))
            .contains("must also carry a #[column]")
        );
    }

    #[test]
    fn rejects_a_foreign_key_that_sets_a_column_name() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"writer\")]\n    #[foreign_key]\n    author: Author,\n}\n",
            ))
            .contains("column names are derived")
        );
    }

    #[test]
    fn rejects_a_foreign_key_that_is_a_primary_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    #[foreign_key]\n    author: Author,\n}\n",
            ))
            .contains("cannot be a primary key")
        );
    }

    #[test]
    fn rejects_a_positional_foreign_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"t\")]\nstruct S(#[column] #[foreign_key] Author);\n",
            ))
            .contains("requires a named field")
        );
    }

    #[test]
    fn rejects_a_foreign_key_to_an_unresolvable_type() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    other: DoesNotExist,\n}\n",
            )
            .contains("is not a #[model]")
        );
    }

    #[test]
    fn rejects_a_foreign_key_to_a_non_model_struct() {
        assert!(
            error_message(
                "struct Plain;\n\n#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    other: Plain,\n}\n",
            )
            .contains("is not a #[model]")
        );
    }

    #[test]
    fn rejects_a_foreign_key_to_a_model_without_a_primary_key() {
        let source = "#[model(table = \"authors\")]\nstruct Author {\n    #[column]\n    name: String,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n";

        assert!(error_message(source).contains("which has no primary key"));
    }

    #[test]
    fn rejects_a_repeated_foreign_key_attribute() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[foreign_key]\n    author: Author,\n}\n",
            ))
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_malformed_foreign_key_arguments() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(= 5)]\n    author: Author,\n}\n",
            ))
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_foreign_key_that_collides_with_a_scalar_column() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"author_id\")]\n    author_id: String,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n",
            ))
            .contains("duplicate column name")
        );
    }

    #[test]
    fn rejects_a_foreign_key_whose_derived_name_is_not_snake_case() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    myAuthor: Author,\n}\n",
            ))
            .contains("invalid column name")
        );
    }

    #[test]
    fn rejects_a_table_name_that_is_too_long() {
        let table = "a".repeat(64);
        let source = format!(
            "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n"
        );

        assert!(error_message(&source).contains("exceeding the 63-byte"));
    }

    #[test]
    fn rejects_a_derived_column_name_that_is_too_long() {
        let field = "a".repeat(62);
        let source = with_author(&format!(
            "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    {field}: Author,\n}}\n"
        ));

        assert!(error_message(&source).contains("exceeding the 63-byte"));
    }

    #[test]
    fn rejects_an_unknown_on_delete_action() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = purge)]\n    author: Author,\n}\n",
            ))
            .contains("unknown ON DELETE action")
        );
    }

    #[test]
    fn rejects_a_repeated_index_attribute() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index]\n    #[index]\n    value: String,\n}\n",
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_an_index_without_a_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[index]\n    value: String,\n}\n"
            )
            .contains("has an #[index] attribute")
        );
    }

    #[test]
    fn rejects_an_index_on_a_unique_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[index]\n    email: String,\n}\n",
            )
            .contains("a unique constraint is already indexed")
        );
    }

    #[test]
    fn rejects_an_index_on_a_unique_foreign_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[foreign_key]\n    #[index]\n    author: Author,\n}\n",
            ))
            .contains("a unique constraint is already indexed")
        );
    }

    #[test]
    fn rejects_an_index_on_a_primary_key_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    #[index]\n    id: uuid::Uuid,\n}\n",
            )
            .contains("a primary key is already indexed")
        );
    }

    #[test]
    fn rejects_an_index_whose_derived_name_is_too_long() {
        let column = "a".repeat(50);
        let source = format!(
            "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    {column}: String,\n}}\n"
        );

        assert!(error_message(&source).contains("derives an index name"));
    }

    fn table_order(lib_source: &str) -> Vec<String> {
        let directory = crate_with(lib_source);

        models(&index_of(directory.path()))
            .expect("the models resolve")
            .into_iter()
            .map(|model| model.table)
            .collect()
    }

    #[test]
    fn orders_referenced_tables_before_referencing_tables() {
        let source = with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n",
        );

        assert_eq!(table_order(&source), ["authors", "articles"]);
    }

    #[test]
    fn orders_a_self_referential_foreign_key_without_a_cycle() {
        let source = "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    parent: Node,\n}\n";

        assert_eq!(table_order(source), ["nodes"]);
    }

    #[test]
    fn rejects_a_foreign_key_cycle() {
        let source = "#[model(table = \"alpha\")]\nstruct Alpha {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    beta: Beta,\n}\n\n#[model(table = \"beta\")]\nstruct Beta {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    alpha: Alpha,\n}\n";

        assert!(error_message(source).contains("foreign key dependency cycle"));
    }
}
