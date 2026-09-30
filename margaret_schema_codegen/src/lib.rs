pub mod framework_tables;
pub mod render_schema;

mod column_check_tokens;
mod column_default_tokens;
mod column_type_tokens;
mod on_delete_tokens;
mod render;

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model_codegen::models::models;

    use crate::framework_tables::FrameworkTables;
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

    fn schema_source(lib_source: &str) -> String {
        let indexed = IndexedSource::new(lib_source);
        let models = models(&indexed.index).expect("the models resolve");

        render_schema(&models, FrameworkTables::Unused)
            .to_source()
            .split_whitespace()
            .collect()
    }

    fn with_author(referencing: &str) -> String {
        format!("{AUTHOR_MODEL}\n{referencing}")
    }

    fn schema_source_with_provider_state(lib_source: &str) -> String {
        let indexed = IndexedSource::new(lib_source);
        let models = models(&indexed.index).expect("the models resolve");

        render_schema(&models, FrameworkTables::OidcProviderState)
            .to_source()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn appends_the_provider_state_tables_to_the_model_tables() {
        assert!(schema_source_with_provider_state(ARTICLE).contains(
            ".into_iter().chain(margaret::framework::provider_state_postgres::provider_state_tables::provider_state_tables()).collect()"
        ));
    }

    #[test]
    fn declares_only_the_provider_state_tables_without_models() {
        assert!(schema_source_with_provider_state("").contains(
            "tables:margaret::framework::provider_state_postgres::provider_state_tables::provider_state_tables(),"
        ));
    }

    #[test]
    fn generates_a_schema_command_from_a_model() {
        let source = schema_source(ARTICLE);

        assert!(source.contains("pubfnschema()"));
        assert!(source.contains("margaret::framework::model::schema::Schema"));
        assert!(source.contains("margaret::framework::model::table::Table"));
        assert!(source.contains("\"articles\""));
        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Uuid"));
        assert!(
            source.contains("margaret::framework::model::column_default::ColumnDefault::UuidV7")
        );
        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Text"));
        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Boolean"));
        assert!(
            source.contains("margaret::framework::model::column_default::ColumnDefault::NotSet")
        );
        assert!(source.contains("\"is_published\""));
        assert!(source.contains("\"note\""));
        assert!(source.contains("nullable:true"));
        assert!(source.contains("nullable:false"));
        assert!(source.contains("primary_key:vec![\"id\".to_string()]"));
    }

    #[test]
    fn accepts_a_positional_column_with_an_explicit_name() {
        let source =
            schema_source("#[model(table = \"t\")]\nstruct S(#[column(name = \"value\")] i64);\n");

        assert!(source.contains("\"value\""));
        assert!(source.contains("margaret::framework::model::column_type::ColumnType::BigInt"));
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

        assert!(source.contains("margaret::framework::model::foreign_key::ForeignKey"));
        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(
            source.contains("on_delete:margaret::framework::model::on_delete::OnDelete::NoAction")
        );
        assert!(source.contains("references_columns:vec![\"id\".to_string()]"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
        assert!(source.contains(
            "column_type:margaret::framework::model::column_type::ColumnType::Uuid,default:margaret::framework::model::column_default::ColumnDefault::NotSet,name:\"author_id\".to_string(),nullable:false,"
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
            "column_type:margaret::framework::model::column_type::ColumnType::Text,default:margaret::framework::model::column_default::ColumnDefault::NotSet,name:\"tag_slug\".to_string(),nullable:false,"
        ));
    }

    #[test]
    fn resolves_a_foreign_key_to_a_model_declared_later() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n\n#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(source.contains("references_table:\"authors\".to_string()"));
    }

    #[test]
    fn resolves_a_self_referential_foreign_key() {
        let source = schema_source(
            "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    parent: Option<Box<Node>>,\n}\n",
        );

        assert!(source.contains("name:\"parent_id\".to_string(),nullable:true,"));
        assert!(source.contains("references_table:\"nodes\".to_string()"));
    }

    #[test]
    fn derives_distinct_columns_for_two_foreign_keys_to_the_same_model() {
        let source = schema_source(
            "#[model(table = \"users\")]\nstruct User {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"docs\")]\nstruct Doc {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: User,\n    #[column]\n    #[foreign_key]\n    editor: User,\n}\n",
        );

        assert!(source.contains("columns:vec![\"author_id\".to_string()]"));
        assert!(source.contains("columns:vec![\"editor_id\".to_string()]"));
    }

    #[test]
    fn generates_a_bytea_column_from_a_byte_vector() {
        let source = schema_source(
            "#[model(table = \"files\")]\nstruct File {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    data: Vec<u8>,\n}\n",
        );

        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Bytea"));
    }

    #[test]
    fn generates_a_numeric_column_from_a_decimal() {
        let source = schema_source(
            "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(precision = 12, scale = 2)]\n    unit_price: rust_decimal::Decimal,\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::model::column_type::ColumnType::Numeric{precision:12u32,scale:2u32,}"
        ));
    }

    #[test]
    fn generates_a_real_column_from_an_f32() {
        let source = schema_source(
            "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    reputation: f32,\n}\n",
        );

        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Real"));
    }

    #[test]
    fn generates_a_double_precision_column_from_an_f64() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    reading_minutes: f64,\n}\n",
        );

        assert!(
            source.contains("margaret::framework::model::column_type::ColumnType::DoublePrecision")
        );
    }

    #[test]
    fn generates_a_text_column_from_an_enum_field() {
        let source = schema_source(
            "enum ArticleStatus {\n    Draft,\n    Published,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: ArticleStatus,\n}\n",
        );

        assert!(source.contains(
            "column_type:margaret::framework::model::column_type::ColumnType::Text,default:margaret::framework::model::column_default::ColumnDefault::NotSet,name:\"status\".to_string(),nullable:false,"
        ));
    }

    #[test]
    fn generates_a_unique_constraint_from_a_scalar_column() {
        let source = schema_source(
            "#[model(table = \"users\")]\nstruct User {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n}\n",
        );

        assert!(source.contains(
            "unique_constraints:vec![margaret::framework::model::unique_constraint::UniqueConstraint{columns:vec![\"email\".to_string()],}]"
        ));
    }

    #[test]
    fn generates_a_unique_constraint_over_a_foreign_key() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[foreign_key]\n    author: Author,\n}\n",
        ));

        assert!(source.contains(
            "unique_constraints:vec![margaret::framework::model::unique_constraint::UniqueConstraint{columns:vec![\"author_id\".to_string()],}]"
        ));
    }

    #[test]
    fn generates_a_foreign_key_on_delete_action() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = cascade)]\n    author: Author,\n}\n",
        ));

        assert!(
            source.contains("on_delete:margaret::framework::model::on_delete::OnDelete::Cascade")
        );
    }

    #[test]
    fn generates_a_scalar_unique_constraint_before_a_foreign_key_unique_constraint() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    slug: String,\n    #[column(unique)]\n    #[foreign_key]\n    author: Author,\n}\n",
        ));

        assert!(source.contains(
            "unique_constraints:vec![margaret::framework::model::unique_constraint::UniqueConstraint{columns:vec![\"slug\".to_string()],},margaret::framework::model::unique_constraint::UniqueConstraint{columns:vec![\"author_id\".to_string()],}]"
        ));
    }

    #[test]
    fn generates_an_index_from_a_scalar_column() {
        let source = schema_source(
            "#[model(table = \"posts\")]\nstruct Post {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    slug: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"slug\".to_string()],name:\"posts_slug_index\".to_string(),}]"
        ));
    }

    #[test]
    fn generates_an_index_over_a_foreign_key() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[index]\n    author: Author,\n}\n",
        ));

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"author_id\".to_string()],name:\"articles_author_id_index\".to_string(),}]"
        ));
    }

    #[test]
    fn generates_a_composite_index_from_a_model_attribute() {
        let source = schema_source(
            "#[model(table = \"events\")]\n#[index(name = \"events_kind_label\", columns = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"kind\".to_string(),\"label\".to_string()],name:\"events_kind_label\".to_string(),}]"
        ));
    }

    #[test]
    fn orders_composite_index_columns_as_declared() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\n#[index(name = \"articles_author_title\", columns = [author_id, title])]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    author: Author,\n    #[column]\n    title: String,\n}\n",
        ));

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"author_id\".to_string(),\"title\".to_string()],name:\"articles_author_title\".to_string(),}]"
        ));
    }

    #[test]
    fn allows_a_unique_column_inside_a_composite_index() {
        let source = schema_source(
            "#[model(table = \"members\")]\n#[index(name = \"members_email_label\", columns = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n    #[column]\n    label: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"email\".to_string(),\"label\".to_string()],name:\"members_email_label\".to_string(),}]"
        ));
        assert!(source.contains(
            "unique_constraints:vec![margaret::framework::model::unique_constraint::UniqueConstraint{columns:vec![\"email\".to_string()],}]"
        ));
    }

    #[test]
    fn orders_multiple_indexes_by_name() {
        let source = schema_source(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    slug: String,\n    #[column]\n    #[index]\n    created_at: String,\n}\n",
        );

        assert!(source.contains(
            "indexes:vec![margaret::framework::model::index::Index{columns:vec![\"created_at\".to_string()],name:\"articles_created_at_index\".to_string(),},margaret::framework::model::index::Index{columns:vec![\"slug\".to_string()],name:\"articles_slug_index\".to_string(),}]"
        ));
    }

    #[test]
    fn generates_a_byte_length_check_from_a_column_attribute() {
        let source = schema_source(
            "#[model(table = \"fragment_metadata\")]\nstruct FragmentMetadata {\n    #[column(primary_key, byte_length = 32)]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(source.contains(
            "checks:vec![margaret::framework::model::column_check::ColumnCheck{name:\"fragment_metadata_hash_byte_length\".to_string(),predicate:margaret::framework::model::check_predicate::CheckPredicate::ByteLength{length:32u32,},}]"
        ));
    }

    #[test]
    fn generates_a_minimum_check_from_a_column_attribute() {
        let source = schema_source(
            "#[model(table = \"fragment_metadata\")]\nstruct FragmentMetadata {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n    #[column(minimum = 0)]\n    size_payload: i64,\n}\n",
        );

        assert!(source.contains(
            "checks:vec![margaret::framework::model::column_check::ColumnCheck{name:\"fragment_metadata_size_payload_minimum\".to_string(),predicate:margaret::framework::model::check_predicate::CheckPredicate::Minimum{minimum:0u32,},}]"
        ));
    }

    #[test]
    fn generates_an_empty_check_list_for_a_column_without_checks() {
        let source = schema_source(
            "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert!(source.contains("Column{checks:vec![],"));
    }

    #[test]
    fn generates_a_composite_foreign_key_from_a_model_attribute() {
        let source = schema_source(
            "#[model(table = \"fragment_metadata\")]\n#[primary_key(columns = [partition, hash])]\nstruct FragmentMetadata {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n\n#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, hash], references = crate::FragmentMetadata, on_delete = cascade)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(source.contains(
            "columns:vec![\"partition\".to_string(),\"hash\".to_string()],on_delete:margaret::framework::model::on_delete::OnDelete::Cascade,references_columns:vec![\"partition\".to_string(),\"hash\".to_string()],references_table:\"fragment_metadata\".to_string(),"
        ));
    }

    #[test]
    fn generates_an_integer_column_from_an_i32() {
        let source = schema_source(
            "#[model(table = \"counters\")]\nstruct Counter {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    hits: i32,\n}\n",
        );

        assert!(source.contains("margaret::framework::model::column_type::ColumnType::Integer"));
    }

    #[test]
    fn generates_a_timestamptz_column_from_a_datetime() {
        let source = schema_source(
            "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    occurred_at: chrono::DateTime<chrono::Utc>,\n}\n",
        );

        assert!(
            source.contains("margaret::framework::model::column_type::ColumnType::Timestamptz")
        );
    }

    #[test]
    fn generates_on_delete_restrict() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = restrict)]\n    author: Author,\n}\n",
        ));

        assert!(
            source.contains("on_delete:margaret::framework::model::on_delete::OnDelete::Restrict")
        );
    }

    #[test]
    fn generates_on_delete_set_null() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = set_null)]\n    author: Option<Author>,\n}\n",
        ));

        assert!(
            source.contains("on_delete:margaret::framework::model::on_delete::OnDelete::SetNull")
        );
    }

    #[test]
    fn generates_on_delete_set_default() {
        let source = schema_source(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = set_default)]\n    author: Author,\n}\n",
        ));

        assert!(
            source
                .contains("on_delete:margaret::framework::model::on_delete::OnDelete::SetDefault")
        );
    }
}
