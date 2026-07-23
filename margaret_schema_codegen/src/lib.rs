pub mod render_schema;

mod render;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_model_codegen::models::models;

    use crate::render_schema::render_schema;

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

    fn schema_source(lib_source: &str) -> String {
        let directory = crate_with(lib_source);
        let models = models(&index_of(directory.path())).expect("the models resolve");

        render_schema(&models)
            .to_source()
            .split_whitespace()
            .collect()
    }

    fn with_author(referencing: &str) -> String {
        format!("{AUTHOR_MODEL}\n{referencing}")
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
    fn generates_a_foreign_key_column_and_constraint() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n",
        ));

        assert!(source.contains("margaret_model::foreign_key::ForeignKey"));
        assert!(source.contains("column:\"author_id\".to_string()"));
        assert!(source.contains("on_delete:margaret_model::on_delete::OnDelete::NoAction"));
        assert!(source.contains("references_column:\"id\".to_string()"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
        assert!(source.contains(
            "column_type:margaret_model::column_type::ColumnType::Uuid,default:margaret_model::column_default::ColumnDefault::NotSet,name:\"author_id\".to_string(),nullable:false,"
        ));
    }

    #[test]
    fn generates_a_nullable_foreign_key_column() {
        let source = schema_source(&with_author(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Option<Author>,\n}\n",
        ));

        assert!(source.contains("name:\"author_id\".to_string(),nullable:true,"));
    }

    #[test]
    fn copies_the_referenced_primary_key_type() {
        let source = schema_source(
            "#[model(table = \"tags\")]\nstruct Tag {\n    #[column(primary_key)]\n    slug: String,\n}\n\n#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    tag: Tag,\n}\n",
        );

        assert!(source.contains(
            "column_type:margaret_model::column_type::ColumnType::Text,default:margaret_model::column_default::ColumnDefault::NotSet,name:\"tag_slug\".to_string(),nullable:false,"
        ));
    }

    #[test]
    fn resolves_a_foreign_key_to_a_model_declared_later() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n\n#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("column:\"author_id\".to_string()"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
    }

    #[test]
    fn resolves_a_self_referential_foreign_key() {
        let source = schema_source(
            "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    parent: Option<Node>,\n}\n",
        );

        assert!(source.contains("name:\"parent_id\".to_string(),nullable:true,"));
        assert!(source.contains("references_table:\"nodes\".to_string()"));
    }

    #[test]
    fn derives_distinct_columns_for_two_foreign_keys_to_the_same_model() {
        let source = schema_source(
            "#[model(table = \"users\")]\nstruct User {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"docs\")]\nstruct Doc {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: User,\n    #[column]\n    #[foreign_key]\n    editor: User,\n}\n",
        );

        assert!(source.contains("column:\"author_id\".to_string()"));
        assert!(source.contains("column:\"editor_id\".to_string()"));
    }

    #[test]
    fn generates_a_bytea_column_from_a_byte_vector() {
        let source = schema_source(
            "#[model(table = \"files\")]\nstruct File {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    data: Vec<u8>,\n}\n",
        );

        assert!(source.contains("margaret_model::column_type::ColumnType::Bytea"));
    }

    #[test]
    fn generates_a_unique_constraint_from_a_scalar_column() {
        let source = schema_source(
            "#[model(table = \"users\")]\nstruct User {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n}\n",
        );

        assert!(source.contains(
            "unique_constraints:vec![margaret_model::unique_constraint::UniqueConstraint{columns:vec![\"email\".to_string()],}]"
        ));
    }

    #[test]
    fn generates_a_unique_constraint_over_a_foreign_key() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[foreign_key]\n    author: Author,\n}\n",
        ));

        assert!(source.contains(
            "unique_constraints:vec![margaret_model::unique_constraint::UniqueConstraint{columns:vec![\"author_id\".to_string()],}]"
        ));
    }

    #[test]
    fn generates_a_foreign_key_on_delete_action() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = cascade)]\n    author: Author,\n}\n",
        ));

        assert!(source.contains("on_delete:margaret_model::on_delete::OnDelete::Cascade"));
    }

    #[test]
    fn generates_an_index_from_a_scalar_column() {
        let source = schema_source(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    slug: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{column:\"slug\".to_string(),name:\"posts_slug_index\".to_string(),}]"
        ));
    }

    #[test]
    fn generates_an_index_over_a_foreign_key() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[index]\n    author: Author,\n}\n",
        ));

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{column:\"author_id\".to_string(),name:\"articles_author_id_index\".to_string(),}]"
        ));
    }
}
