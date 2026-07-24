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

    const AUTHOR_MODEL: &str = "\
#[model(table = \"authors\")]
struct Author {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column(unique)]
    name: String,
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
    fn generates_a_schema_function() {
        let source = schema_source(
            "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("pubfnschema()"));
        assert!(source.contains("margaret_model::schema::Schema"));
        assert!(source.contains("margaret_model::table::Table"));
    }

    #[test]
    fn generates_all_column_types_defaults_and_nullability() {
        let source = schema_source(
            "#[model(table = \"things\")]\nstruct Thing {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    count: i32,\n    #[column]\n    big: i64,\n    #[column]\n    flag: bool,\n    #[column]\n    label: String,\n    #[column]\n    created_at: chrono::DateTime<chrono::Utc>,\n    #[column]\n    data: Vec<u8>,\n    #[column]\n    note: Option<String>,\n}\n",
        );

        assert!(source.contains("margaret_model::column_type::ColumnType::Uuid"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Integer"));
        assert!(source.contains("margaret_model::column_type::ColumnType::BigInt"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Boolean"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Text"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Timestamptz"));
        assert!(source.contains("margaret_model::column_type::ColumnType::Bytea"));
        assert!(source.contains("margaret_model::column_default::ColumnDefault::UuidV7"));
        assert!(source.contains("margaret_model::column_default::ColumnDefault::NotSet"));
        assert!(source.contains("nullable:true"));
        assert!(source.contains("nullable:false"));
        assert!(source.contains("primary_key:vec![\"id\".to_string()]"));
        assert!(source.contains("\"things\""));
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
    fn generates_a_single_column_foreign_key() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        ));

        assert!(source.contains("margaret_model::foreign_key::ForeignKey"));
        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(source.contains("name:\"articles_author_fkey\".to_string()"));
        assert!(source.contains("on_delete:margaret_model::on_delete::OnDelete::NoAction"));
        assert!(source.contains("references_columns:vec![\"id\".to_string()]"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
    }

    #[test]
    fn generates_a_composite_foreign_key() {
        let source = schema_source(
            "#[model(table = \"pairs\")]\nstruct Pair {\n    #[column(primary_key)]\n    left: uuid::Uuid,\n    #[column(primary_key)]\n    right: uuid::Uuid,\n}\n\n#[model(table = \"links\")]\nstruct Link {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left)]\n    pair_left: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_pair_fkey\", references = Pair::right)]\n    pair_right: uuid::Uuid,\n}\n",
        );

        assert!(source.contains(
            "columns:vec![\"pair_left\".to_string(),\"pair_right\".to_string()]"
        ));
        assert!(source.contains(
            "references_columns:vec![\"left\".to_string(),\"right\".to_string()]"
        ));
        assert!(source.contains("references_table:\"pairs\".to_string()"));
    }

    #[test]
    fn references_a_string_primary_key() {
        let source = schema_source(
            "#[model(table = \"tags\")]\nstruct Tag {\n    #[column(primary_key)]\n    slug: String,\n}\n\n#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"posts_tag_fkey\", references = Tag::slug)]\n    tag_slug: String,\n}\n",
        );

        assert!(source.contains("references_columns:vec![\"slug\".to_string()]"));
        assert!(source.contains("references_table:\"tags\".to_string()"));
    }

    #[test]
    fn generates_a_nullable_foreign_key_column() {
        let source = schema_source(&with_author(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"posts_author_fkey\", references = Author::id)]\n    author_id: Option<uuid::Uuid>,\n}\n",
        ));

        assert!(source.contains("name:\"author_id\".to_string(),nullable:true,"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
    }

    #[test]
    fn resolves_a_foreign_key_to_a_model_declared_later() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n\n#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
    }

    #[test]
    fn resolves_a_self_referential_foreign_key() {
        let source = schema_source(
            "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"nodes_parent_fkey\", references = Node::id)]\n    parent_id: Option<uuid::Uuid>,\n}\n",
        );

        assert!(source.contains("name:\"parent_id\".to_string(),nullable:true,"));
        assert!(source.contains("references_table:\"nodes\".to_string()"));
    }

    #[test]
    fn generates_two_foreign_keys_to_the_same_model() {
        let source = schema_source(
            "#[model(table = \"users\")]\nstruct User {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"docs\")]\nstruct Doc {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"docs_author_fkey\", references = User::id)]\n    author_id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"docs_editor_fkey\", references = User::id)]\n    editor_id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("name:\"docs_author_fkey\".to_string()"));
        assert!(source.contains("name:\"docs_editor_fkey\".to_string()"));
        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(source.contains("columns:vec![\"editor_id\".to_string()]"));
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
    fn generates_a_unique_constraint_over_a_foreign_key_column() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        ));

        assert!(source.contains(
            "unique_constraints:vec![margaret_model::unique_constraint::UniqueConstraint{columns:vec![\"author_id\".to_string()],}]"
        ));
    }

    #[test]
    fn renders_every_on_delete_action() {
        let source = schema_source(&with_author(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"posts_a_fkey\", references = Author::id, on_delete = cascade)]\n    a: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"posts_b_fkey\", references = Author::id, on_delete = restrict)]\n    b: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"posts_c_fkey\", references = Author::id, on_delete = set_null)]\n    c: Option<uuid::Uuid>,\n    #[column]\n    #[foreign_key(name = \"posts_d_fkey\", references = Author::id, on_delete = set_default)]\n    d: Option<uuid::Uuid>,\n    #[column]\n    #[foreign_key(name = \"posts_e_fkey\", references = Author::id)]\n    e: uuid::Uuid,\n}\n",
        ));

        assert!(source.contains("margaret_model::on_delete::OnDelete::Cascade"));
        assert!(source.contains("margaret_model::on_delete::OnDelete::Restrict"));
        assert!(source.contains("margaret_model::on_delete::OnDelete::SetNull"));
        assert!(source.contains("margaret_model::on_delete::OnDelete::SetDefault"));
        assert!(source.contains("margaret_model::on_delete::OnDelete::NoAction"));
    }

    #[test]
    fn a_uuid_foreign_key_column_has_no_generated_default() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        ));

        assert!(source.contains(
            "column_type:margaret_model::column_type::ColumnType::Uuid,default:margaret_model::column_default::ColumnDefault::NotSet,name:\"author_id\".to_string(),nullable:false,"
        ));
    }

    #[test]
    fn generates_an_index_from_a_scalar_column() {
        let source = schema_source(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    slug: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{columns:vec![\"slug\".to_string()],name:\"posts_slug_index\".to_string(),}]"
        ));
    }

    #[test]
    fn generates_an_index_over_a_foreign_key_column() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    #[index]\n    author_id: uuid::Uuid,\n}\n",
        ));

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{columns:vec![\"author_id\".to_string()],name:\"articles_author_id_index\".to_string(),}]"
        ));
    }

    #[test]
    fn combines_columns_that_share_an_index_name_into_one_index() {
        let source = schema_source(
            "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"events_kind_label\")]\n    kind: String,\n    #[column]\n    #[index(name = \"events_kind_label\")]\n    label: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{columns:vec![\"kind\".to_string(),\"label\".to_string()],name:\"events_kind_label\".to_string(),}]"
        ));
    }

    #[test]
    fn allows_a_unique_column_inside_a_composite_index() {
        let source = schema_source(
            "#[model(table = \"members\")]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[index(name = \"members_email_label\")]\n    email: String,\n    #[column]\n    #[index(name = \"members_email_label\")]\n    label: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{columns:vec![\"email\".to_string(),\"label\".to_string()],name:\"members_email_label\".to_string(),}]"
        ));
        assert!(source.contains(
            "unique_constraints:vec![margaret_model::unique_constraint::UniqueConstraint{columns:vec![\"email\".to_string()],}]"
        ));
    }

    #[test]
    fn orders_multiple_indexes_by_name() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    slug: String,\n    #[column]\n    #[index]\n    created_at: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret_model::index::Index{columns:vec![\"created_at\".to_string()],name:\"articles_created_at_index\".to_string(),},margaret_model::index::Index{columns:vec![\"slug\".to_string()],name:\"articles_slug_index\".to_string(),}]"
        ));
    }
}
