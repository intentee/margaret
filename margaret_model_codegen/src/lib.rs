pub mod has_models;
pub mod index_membership;
pub mod inferred_column;
pub mod model;
pub mod model_codegen_error;
pub mod models;
pub mod object_kind;
pub mod resolved_column;
pub mod resolved_foreign_key;
pub mod resolved_index;
pub mod resolved_unique_constraint;

mod collected_model;
mod column_arguments;
mod column_id;
mod deferred_foreign_key_member;
mod foreign_key_arguments;
mod foreign_key_target;
mod foreign_key_target_column;
mod index_arguments;
mod infer_column_type;
mod model_arguments;
mod option_inner;
mod resolve_column_default;
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

    use margaret_model::column_default::ColumnDefault;
    use margaret_model::on_delete::OnDelete;

    use crate::has_models::has_models;
    use crate::model::Model;
    use crate::models::models;
    use crate::resolved_column::ResolvedColumn;
    use crate::resolved_foreign_key::ResolvedForeignKey;

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
    #[column(unique)]
    name: String,
    #[column]
    bio: String,
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

    fn resolve(lib_source: &str) -> Vec<Model> {
        let directory = crate_with(lib_source);

        models(&index_of(directory.path())).expect("the models resolve")
    }

    fn error_message(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        models(&index_of(directory.path()))
            .expect_err("the models fail to resolve")
            .to_string()
    }

    fn model_named<'models>(models: &'models [Model], table: &str) -> &'models Model {
        models
            .iter()
            .find(|model| model.table.as_str() == table)
            .expect("the table is resolved")
    }

    fn foreign_key_named<'model>(model: &'model Model, name: &str) -> &'model ResolvedForeignKey {
        model
            .foreign_keys
            .iter()
            .find(|foreign_key| foreign_key.name.as_str() == name)
            .expect("the foreign key is resolved")
    }

    fn column_named<'model>(model: &'model Model, name: &str) -> &'model ResolvedColumn {
        model
            .columns
            .iter()
            .find(|column| column.name == name)
            .expect("the column is resolved")
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
    fn rejects_a_table_name_that_is_too_long() {
        let table = "a".repeat(64);
        let source = format!(
            "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n"
        );

        assert!(error_message(&source).contains("table name that is too long"));
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
    fn rejects_a_scalar_column_name_that_is_too_long() {
        let column = "a".repeat(64);
        let source = format!(
            "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"{column}\")]\n    value: String,\n}}\n"
        );

        assert!(error_message(&source).contains("column name that is too long"));
    }

    #[test]
    fn rejects_a_foreign_key_without_a_column() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("must also carry a #[column]")
        );
    }

    #[test]
    fn rejects_a_foreign_key_missing_a_name() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("is missing the required 'name'")
        );
    }

    #[test]
    fn rejects_an_invalid_foreign_key_name() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"Bad\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("invalid foreign key name")
        );
    }

    #[test]
    fn rejects_a_foreign_key_name_that_is_too_long() {
        let name = "a".repeat(64);
        let source = with_author(&format!(
            "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"{name}\", references = Author::id)]\n    author_id: uuid::Uuid,\n}}\n"
        ));

        assert!(error_message(&source).contains("has a name that is too long"));
    }

    #[test]
    fn rejects_a_foreign_key_missing_references() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\")]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("is missing the required 'references'")
        );
    }

    #[test]
    fn rejects_a_reference_without_a_field_segment() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("does not name a column of the target model")
        );
    }

    #[test]
    fn rejects_a_reference_to_a_non_model() {
        assert!(
            error_message(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Missing::id)]\n    author_id: uuid::Uuid,\n}\n",
            )
            .contains("is not a #[model]")
        );
    }

    #[test]
    fn rejects_a_reference_to_a_non_model_struct() {
        assert!(
            error_message(
                "struct Plain {\n    id: uuid::Uuid,\n}\n\n#[model(table = \"t\")]\nstruct T {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"t_plain_fkey\", references = Plain::id)]\n    plain_id: uuid::Uuid,\n}\n",
            )
            .contains("is not a #[model]")
        );
    }

    #[test]
    fn rejects_a_reference_to_an_unknown_field() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::missing)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("has no such column")
        );
    }

    #[test]
    fn rejects_referenced_columns_that_are_not_a_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::bio)]\n    author_bio: String,\n}\n",
            ))
            .contains("are not a primary key or unique constraint")
        );
    }

    #[test]
    fn rejects_a_foreign_key_with_ambiguous_targets() {
        let source = with_author(
            "#[model(table = \"pairs\")]\nstruct Pair {\n    #[column(primary_key)]\n    left: uuid::Uuid,\n    #[column(primary_key)]\n    right: uuid::Uuid,\n}\n\n#[model(table = \"links\")]\nstruct Link {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_mixed_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_mixed_fkey\", references = Pair::left)]\n    pair_left: uuid::Uuid,\n}\n",
        );

        assert!(error_message(&source).contains("references more than one target model"));
    }

    #[test]
    fn rejects_a_foreign_key_column_type_mismatch() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: i64,\n}\n",
            ))
            .contains("does not match the referenced type")
        );
    }

    #[test]
    fn rejects_a_foreign_key_that_references_a_column_twice() {
        let source = "\
#[model(table = \"pairs\")]
struct Pair {
    #[column(primary_key)]
    left: uuid::Uuid,
    #[column(primary_key)]
    right: uuid::Uuid,
}

#[model(table = \"links\")]
struct Link {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left)]
    a: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left)]
    b: uuid::Uuid,
}
";

        assert!(
            error_message(source).contains("references the same target column more than once")
        );
    }

    #[test]
    fn rejects_inconsistent_on_delete_across_a_foreign_key() {
        let source = "\
#[model(table = \"pairs\")]
struct Pair {
    #[column(primary_key)]
    left: uuid::Uuid,
    #[column(primary_key)]
    right: uuid::Uuid,
}

#[model(table = \"links\")]
struct Link {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left, on_delete = cascade)]
    pair_left: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::right, on_delete = restrict)]
    pair_right: uuid::Uuid,
}
";

        assert!(error_message(source).contains("more than one distinct ON DELETE action"));
    }

    #[test]
    fn rejects_a_repeated_foreign_key_attribute() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"a\", references = Author::id)]\n    #[foreign_key(name = \"b\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_malformed_foreign_key_arguments() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(= 5)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_an_unknown_on_delete_action() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id, on_delete = purge)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("unknown ON DELETE action")
        );
    }

    #[test]
    fn a_uuid_primary_key_column_auto_generates() {
        let models = resolve(
            "#[model(table = \"t\")]\nstruct T {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
        );

        assert_eq!(column_named(model_named(&models, "t"), "id").default, ColumnDefault::UuidV7);
    }

    #[test]
    fn plain_and_foreign_key_uuid_columns_have_no_default() {
        let models = resolve(&with_author(
            "#[model(table = \"pairs\")]\nstruct Pair {\n    #[column(primary_key)]\n    left: uuid::Uuid,\n    #[column(primary_key)]\n    right: uuid::Uuid,\n}\n\n#[model(table = \"records\")]\nstruct Record {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    plain: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"records_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"records_optional_author_fkey\", references = Author::id)]\n    optional_author_id: Option<uuid::Uuid>,\n    #[column]\n    #[foreign_key(name = \"records_pair_fkey\", references = Pair::left)]\n    pair_left: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"records_pair_fkey\", references = Pair::right)]\n    pair_right: uuid::Uuid,\n}\n",
        ));
        let record = model_named(&models, "records");

        assert_eq!(column_named(record, "id").default, ColumnDefault::UuidV7);
        assert_eq!(column_named(record, "plain").default, ColumnDefault::NotSet);
        assert_eq!(column_named(record, "author_id").default, ColumnDefault::NotSet);
        assert_eq!(column_named(record, "optional_author_id").default, ColumnDefault::NotSet);
        assert_eq!(column_named(record, "pair_left").default, ColumnDefault::NotSet);
        assert_eq!(column_named(record, "pair_right").default, ColumnDefault::NotSet);
    }

    #[test]
    fn rejects_set_null_on_a_non_nullable_foreign_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id, on_delete = set_null)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("nulls its columns on delete")
        );
    }

    #[test]
    fn rejects_set_default_on_a_non_nullable_foreign_key() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id, on_delete = set_default)]\n    author_id: uuid::Uuid,\n}\n",
            ))
            .contains("nulls its columns on delete")
        );
    }

    #[test]
    fn rejects_set_null_on_a_partially_nullable_composite_foreign_key() {
        let source = "\
#[model(table = \"pairs\")]
struct Pair {
    #[column(primary_key)]
    left: uuid::Uuid,
    #[column(primary_key)]
    right: uuid::Uuid,
}

#[model(table = \"links\")]
struct Link {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left, on_delete = set_null)]
    pair_left: Option<uuid::Uuid>,
    #[column]
    #[foreign_key(name = \"links_pair_fkey\", references = Pair::right, on_delete = set_null)]
    pair_right: uuid::Uuid,
}
";

        assert!(error_message(source).contains("nulls its columns on delete"));
    }

    #[test]
    fn resolves_set_null_on_a_nullable_foreign_key() {
        let models = resolve(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id, on_delete = set_null)]\n    author_id: Option<uuid::Uuid>,\n}\n",
        ));
        let article = model_named(&models, "articles");

        assert_eq!(
            foreign_key_named(article, "articles_author_fkey").on_delete,
            OnDelete::SetNull
        );
    }

    #[test]
    fn references_a_positional_primary_key_column() {
        let models = resolve(
            "#[model(table = \"codes\")]\nstruct Code(#[column(primary_key, name = \"value\")] String);\n\n#[model(table = \"usages\")]\nstruct Usage {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"usages_code_fkey\", references = Code::value)]\n    code_value: String,\n}\n",
        );
        let usage = model_named(&models, "usages");
        let foreign_key = foreign_key_named(usage, "usages_code_fkey");

        assert_eq!(foreign_key.references_columns, ["value"]);
        assert_eq!(foreign_key.references_table, "codes");
    }

    #[test]
    fn resolves_a_single_column_foreign_key_to_a_primary_key() {
        let models = resolve(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id, on_delete = cascade)]\n    author_id: uuid::Uuid,\n}\n",
        ));
        let article = model_named(&models, "articles");
        let foreign_key = foreign_key_named(article, "articles_author_fkey");

        assert_eq!(foreign_key.columns, ["author_id"]);
        assert_eq!(foreign_key.references_columns, ["id"]);
        assert_eq!(foreign_key.references_table, "authors");
        assert_eq!(foreign_key.on_delete, OnDelete::Cascade);
    }

    #[test]
    fn resolves_a_composite_foreign_key_to_a_composite_primary_key() {
        let models = resolve(
            "#[model(table = \"pairs\")]\nstruct Pair {\n    #[column(primary_key)]\n    left: uuid::Uuid,\n    #[column(primary_key)]\n    right: uuid::Uuid,\n}\n\n#[model(table = \"links\")]\nstruct Link {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_pair_fkey\", references = Pair::left)]\n    pair_left: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"links_pair_fkey\", references = Pair::right)]\n    pair_right: uuid::Uuid,\n}\n",
        );
        let link = model_named(&models, "links");
        let foreign_key = foreign_key_named(link, "links_pair_fkey");

        assert_eq!(foreign_key.columns, ["pair_left", "pair_right"]);
        assert_eq!(foreign_key.references_columns, ["left", "right"]);
        assert_eq!(foreign_key.references_table, "pairs");
    }

    #[test]
    fn resolves_a_foreign_key_to_a_unique_key() {
        let models = resolve(&with_author(
            "#[model(table = \"aliases\")]\nstruct Alias {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"aliases_author_fkey\", references = Author::name)]\n    author_name: String,\n}\n",
        ));
        let alias = model_named(&models, "aliases");
        let foreign_key = foreign_key_named(alias, "aliases_author_fkey");

        assert_eq!(foreign_key.columns, ["author_name"]);
        assert_eq!(foreign_key.references_columns, ["name"]);
        assert_eq!(foreign_key.references_table, "authors");
    }

    #[test]
    fn resolves_a_foreign_key_to_a_unique_column_of_a_composite_key_model() {
        let models = resolve(
            "#[model(table = \"lookups\")]\nstruct Lookup {\n    #[column(primary_key)]\n    left: uuid::Uuid,\n    #[column(primary_key)]\n    right: uuid::Uuid,\n    #[column(unique)]\n    slug: String,\n}\n\n#[model(table = \"referrers\")]\nstruct Referrer {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"referrers_lookup_fkey\", references = Lookup::slug)]\n    lookup_slug: String,\n}\n",
        );
        let referrer = model_named(&models, "referrers");
        let foreign_key = foreign_key_named(referrer, "referrers_lookup_fkey");

        assert_eq!(foreign_key.references_columns, ["slug"]);
        assert_eq!(foreign_key.references_table, "lookups");
    }

    #[test]
    fn references_two_same_named_models_in_different_modules() {
        let source = "\
mod first {
    #[model(table = \"first_tenant\")]
    pub struct Tenant {
        #[column(primary_key)]
        pub id: uuid::Uuid,
    }
}

mod second {
    #[model(table = \"second_tenant\")]
    pub struct Tenant {
        #[column(primary_key)]
        pub id: uuid::Uuid,
    }
}

#[model(table = \"memberships\")]
struct Membership {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"memberships_first_fkey\", references = crate::first::Tenant::id)]
    first_id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"memberships_second_fkey\", references = crate::second::Tenant::id)]
    second_id: uuid::Uuid,
}
";
        let models = resolve(source);
        let membership = model_named(&models, "memberships");

        assert_eq!(
            foreign_key_named(membership, "memberships_first_fkey").references_table,
            "first_tenant"
        );
        assert_eq!(
            foreign_key_named(membership, "memberships_second_fkey").references_table,
            "second_tenant"
        );
    }

    #[test]
    fn rejects_a_repeated_bare_index_attribute() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index]\n    #[index]\n    value: String,\n}\n",
            )
            .contains("carries a repeated #[index]")
        );
    }

    #[test]
    fn rejects_a_repeated_named_index_on_a_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = \"combo\")]\n    #[index(name = \"combo\")]\n    value: String,\n}\n",
            )
            .contains("carries a repeated #[index]")
        );
    }

    #[test]
    fn rejects_an_index_without_a_column() {
        assert!(
            error_message("#[model(table = \"t\")]\nstruct S {\n    #[index]\n    value: String,\n}\n")
                .contains("has an #[index] attribute")
        );
    }

    #[test]
    fn rejects_an_index_on_a_unique_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[index]\n    email: String,\n}\n",
            )
            .contains("leading prefix of a unique constraint")
        );
    }

    #[test]
    fn rejects_an_index_on_the_leading_primary_key_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    #[index]\n    id: uuid::Uuid,\n}\n",
            )
            .contains("leading prefix of the primary key")
        );
    }

    #[test]
    fn rejects_a_multi_column_index_that_prefixes_the_primary_key() {
        let source = "\
#[model(table = \"lore_lock\")]
struct LoreLock {
    #[column(primary_key)]
    #[index(name = \"lore_lock_leading\")]
    repository: String,
    #[column(primary_key)]
    #[index(name = \"lore_lock_leading\")]
    branch: String,
    #[column(primary_key)]
    hash: String,
}
";

        assert!(error_message(source).contains("leading prefix of the primary key"));
    }

    #[test]
    fn indexes_a_non_leading_primary_key_column() {
        let models = resolve(
            "#[model(table = \"lore_lock\")]\nstruct LoreLock {\n    #[column(primary_key)]\n    repository: String,\n    #[column(primary_key)]\n    branch: String,\n    #[column(primary_key)]\n    #[index]\n    hash: String,\n}\n",
        );
        let lore_lock = model_named(&models, "lore_lock");
        let index_columns: Vec<&[String]> = lore_lock
            .indexes
            .iter()
            .map(|index| index.columns.as_slice())
            .collect();

        assert_eq!(index_columns, [["hash".to_string()]]);
    }

    #[test]
    fn rejects_an_index_whose_derived_name_is_too_long() {
        let column = "a".repeat(50);
        let source = format!(
            "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    {column}: String,\n}}\n"
        );

        assert!(error_message(&source).contains("derives an index name that is too long"));
    }

    #[test]
    fn rejects_an_index_name_that_is_not_snake_case() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = \"Bad\")]\n    value: String,\n}\n",
            )
            .contains("invalid index name")
        );
    }

    #[test]
    fn rejects_an_empty_index_name() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = \"\")]\n    value: String,\n}\n",
            )
            .contains("invalid index name")
        );
    }

    #[test]
    fn rejects_an_explicit_index_name_that_is_too_long() {
        let name = "a".repeat(64);
        let source = format!(
            "#[model(table = \"t\")]\nstruct S {{\n    #[column]\n    #[index(name = \"{name}\")]\n    value: String,\n}}\n"
        );

        assert!(error_message(&source).contains("declares an index name"));
    }

    #[test]
    fn rejects_malformed_index_arguments() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(= 5)]\n    value: String,\n}\n",
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_a_non_string_index_name() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = 5)]\n    value: String,\n}\n",
            )
            .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn resolves_a_composite_named_index_ordered_by_field_declaration() {
        let models = resolve(
            "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"events_kind_label\")]\n    kind: String,\n    #[column]\n    #[index(name = \"events_kind_label\")]\n    label: String,\n}\n",
        );
        let event = model_named(&models, "events");

        assert_eq!(event.indexes.len(), 1);
        assert_eq!(event.indexes[0].columns, ["kind", "label"]);
        assert_eq!(event.indexes[0].name.as_str(), "events_kind_label");
    }

    #[test]
    fn rejects_a_duplicate_index_name_across_tables() {
        let source = "\
#[model(table = \"a\")]
struct A {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[index(name = \"shared\")]
    x: String,
}

#[model(table = \"b\")]
struct B {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[index(name = \"shared\")]
    y: String,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_an_explicit_index_name_that_collides_with_a_derived_name() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    c: String,\n    #[column]\n    #[index(name = \"t_c_index\")]\n    d: String,\n}\n",
            )
            .contains("schema-wide relation names must be unique")
        );
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_table_name() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"authors\")]\n    title: String,\n}\n",
            ))
            .contains("schema-wide relation names must be unique")
        );
    }

    #[test]
    fn rejects_a_derived_index_name_that_collides_with_a_table_name() {
        let source = "\
#[model(table = \"articles_title_index\")]
struct Reserved {
    #[column(primary_key)]
    id: uuid::Uuid,
}

#[model(table = \"articles\")]
struct Article {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[index]
    title: String,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_a_primary_key_index_name_that_collides_with_a_table_name() {
        let source = "\
#[model(table = \"articles_pkey\")]
struct Reserved {
    #[column(primary_key)]
    id: uuid::Uuid,
}

#[model(table = \"articles\")]
struct Article {
    #[column(primary_key)]
    id: uuid::Uuid,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_a_primary_key_index_name_that_collides_with_an_index() {
        let source = "\
#[model(table = \"holder\")]
struct Holder {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[index(name = \"target_pkey\")]
    label: String,
}

#[model(table = \"target\")]
struct Target {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"target_holder_fkey\", references = Holder::id)]
    holder_id: uuid::Uuid,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_two_tables_that_generate_the_same_unique_index_name() {
        let source = "\
#[model(table = \"a\")]
struct First {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column(unique)]
    b_c: String,
}

#[model(table = \"a_b\")]
struct Second {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column(unique)]
    c: String,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_primary_key_index() {
        let source = "\
#[model(table = \"author\")]
struct Author {
    #[column(primary_key)]
    id: uuid::Uuid,
}

#[model(table = \"book\")]
struct Book {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[index(name = \"author_pkey\")]
    title: String,
}
";

        assert!(error_message(source).contains("schema-wide relation names must be unique"));
    }

    #[test]
    fn rejects_a_foreign_key_name_that_collides_with_a_constraint() {
        let source = with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_pkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        );

        assert!(error_message(&source).contains("constraint names must be unique per table"));
    }

    #[test]
    fn rejects_a_foreign_key_name_that_collides_with_a_unique_constraint() {
        let source = with_author(
            "#[model(table = \"t\")]\nstruct T {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n    #[column]\n    #[foreign_key(name = \"t_email_key\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        );

        assert!(error_message(&source).contains("constraint names must be unique per table"));
    }

    #[test]
    fn rejects_a_primary_key_index_name_that_is_too_long() {
        let table = "a".repeat(60);
        let source = format!(
            "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n"
        );

        assert!(error_message(&source).contains("derives a primary key index name that is too long"));
    }

    #[test]
    fn rejects_a_unique_index_name_that_is_too_long() {
        let column = "a".repeat(60);
        let source = format!(
            "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique, name = \"{column}\")]\n    value: String,\n}}\n"
        );

        assert!(error_message(&source).contains("derives a unique index name that is too long"));
    }

    #[test]
    fn orders_referenced_tables_before_referencing_tables() {
        let models = resolve(&with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"articles_author_fkey\", references = Author::id)]\n    author_id: uuid::Uuid,\n}\n",
        ));
        let order: Vec<String> = models
            .iter()
            .map(|model| model.table.as_str().to_string())
            .collect();

        assert_eq!(order, ["authors", "articles"]);
    }

    #[test]
    fn orders_a_self_referential_foreign_key_without_a_cycle() {
        let models = resolve(
            "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(name = \"nodes_parent_fkey\", references = Node::id)]\n    parent_id: uuid::Uuid,\n}\n",
        );
        let order: Vec<String> = models
            .iter()
            .map(|model| model.table.as_str().to_string())
            .collect();

        assert_eq!(order, ["nodes"]);
    }

    #[test]
    fn rejects_a_foreign_key_cycle() {
        let source = "\
#[model(table = \"alpha\")]
struct Alpha {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"alpha_beta_fkey\", references = Beta::id)]
    beta_id: uuid::Uuid,
}

#[model(table = \"beta\")]
struct Beta {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    #[foreign_key(name = \"beta_alpha_fkey\", references = Alpha::id)]
    alpha_id: uuid::Uuid,
}
";

        assert!(error_message(source).contains("foreign key dependency cycle"));
    }
}
