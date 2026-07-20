pub mod has_models;
pub mod model_codegen_error;
pub mod render_models;

mod column_arguments;
mod infer_column_type;
mod inferred_column;
mod model;
mod model_arguments;
mod models;
mod render;
mod resolved_column;

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
    use crate::render_models::render_models;

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

    fn schema_source(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        render_models(&index_of(directory.path()))
            .expect("the models generate")
            .to_source()
            .split_whitespace()
            .collect()
    }

    fn error_message(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        render_models(&index_of(directory.path()))
            .expect_err("the models fail to generate")
            .to_string()
    }

    #[test]
    fn detects_the_presence_of_models() {
        let with_models = crate_with(ARTICLE);
        let without_models = crate_with("struct Plain;\n");

        assert!(has_models(&index_of(with_models.path())));
        assert!(!has_models(&index_of(without_models.path())));
    }

    #[test]
    fn generates_a_schema_command_from_a_model() {
        let source = schema_source(ARTICLE);

        assert!(source.contains("pubfnschema()"));
        assert!(source.contains("margaret_model::schema::Schema"));
        assert!(source.contains("margaret_model::table::Table"));
        assert!(source.contains("\"articles\""));
        assert!(source.contains("margaret_model::column_type::ColumnType::Uuid"));
        assert!(source.contains("margaret_model::column_default::ColumnDefault::UuidV7"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Text"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Boolean"));
        assert!(source.contains("margaret_model::column_default::ColumnDefault::NotSet"));
        assert!(source.contains("\"is_published\""));
        assert!(source.contains("\"note\""));
        assert!(source.contains("nullable:true"));
        assert!(source.contains("nullable:false"));
        assert!(source.contains("primary_key:vec![\"id\".to_string()]"));
        assert!(source.contains("margaret_model::render_postgres::render_postgres"));
        assert!(source.contains("CommandOutcome::Succeeded"));
    }

    #[test]
    fn accepts_a_positional_column_with_an_explicit_name() {
        let source =
            schema_source("#[model(table = \"t\")]\nstruct S(#[column(name = \"value\")] i64);\n");

        assert!(source.contains("\"value\""));
        assert!(source.contains("margaret_model::column_type::ColumnType::BigInt"));
    }

    #[test]
    fn orders_generated_tables_by_table_name() {
        let source = "\
#[model(table = \"zebra\")]
struct First {
    #[column]
    id: i64,
}

#[model(table = \"apple\")]
struct Second {
    #[column]
    id: i64,
}
";
        let generated = schema_source(source);
        let apple = generated
            .find("\"apple\"")
            .expect("the apple table is generated");
        let zebra = generated
            .find("\"zebra\"")
            .expect("the zebra table is generated");

        assert!(apple < zebra);
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
}
