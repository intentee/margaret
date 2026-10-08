pub mod inferred_column;
pub mod model;
pub mod model_codegen_error;
pub mod models;
pub mod resolved_column;
pub mod resolved_foreign_key;
pub mod resolved_index;
pub mod resolved_unique_constraint;

mod collected_model;
mod column_arguments;
mod column_list_arity;
mod column_type_context;
mod column_type_source;
mod date_time_canonical_path;
mod decimal_canonical_path;
mod declared_column_check;
mod declared_column_type;
mod deferred_foreign_key;
mod deferred_model_foreign_key;
mod enum_column;
mod explicit_index_name;
mod field_index;
mod foreign_key_arguments;
mod foreign_key_target;
mod foreign_key_target_column;
mod index_arguments;
mod index_redundancy;
mod infer_column;
mod known_column_type;
mod model_arguments;
mod model_column_list;
mod model_foreign_key_arguments;
mod model_index_arguments;
mod model_primary_key_arguments;
mod model_unique_arguments;
mod numeric_digits;
pub mod on_delete_actions;
mod on_delete_argument;
mod redundant_index;
mod resolve_declared_columns;
mod scalar_column;
mod uuid_canonical_path;

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model::check_predicate::CheckPredicate;
    use margaret_model::column_check::ColumnCheck;
    use margaret_model::column_default::ColumnDefault;
    use margaret_model::column_type::ColumnType;
    use margaret_model::on_delete::OnDelete;

    use crate::inferred_column::InferredColumn;
    use crate::model::Model;
    use crate::model_codegen_error::ModelCodegenError;
    use crate::models::models;

    const AUTHOR_MODEL: &str = "\
#[model(table = \"authors\")]
struct Author {
    #[column(primary_key)]
    id: uuid::Uuid,
}
";

    fn rejection_for(lib_source: &str) -> ModelCodegenError {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index).expect_err("the models fail to resolve")
    }

    fn error_message(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    fn indexing_rejection(lib_source: &str) -> String {
        IndexedSource::try_new(lib_source)
            .err()
            .expect("the repeated attribute is rejected while indexing")
            .to_string()
    }

    fn with_author(referencing: &str) -> String {
        format!("{AUTHOR_MODEL}\n{referencing}")
    }

    #[test]
    fn rejects_a_model_that_is_not_a_struct() {
        assert!(error_message("#[model(table = \"e\")] enum E {}\n").contains("is not a struct"));
    }

    #[test]
    fn rejects_a_model_declared_more_than_once() {
        assert_eq!(
            indexing_rejection("#[model(table = \"a\")]\n#[model(table = \"b\")]\nstruct S;\n"),
            "attribute 'model' is repeated on 'crate::S' but a single occurrence was expected"
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
        let error = rejection_for("#[model(table = 5)]\nstruct S;\n");

        assert!(matches!(
            error,
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "table" && expected == "string literal"
        ));
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
        let error = rejection_for(
            "#[model(table = \"t\")]\nstruct S {\n    #[column(name = 5)]\n    id: i64,\n}\n",
        );

        assert!(matches!(
            error,
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn rejects_a_non_integer_precision() {
        let error = rejection_for(
            "#[model(table = \"t\")]\nstruct S {\n    #[column(precision = \"12\", scale = 2)]\n    value: rust_decimal::Decimal,\n}\n",
        );

        assert!(matches!(
            error,
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "precision" && expected == "unsigned integer literal"
        ));
    }

    #[test]
    fn rejects_a_non_integer_scale() {
        let error = rejection_for(
            "#[model(table = \"t\")]\nstruct S {\n    #[column(precision = 12, scale = \"2\")]\n    value: rust_decimal::Decimal,\n}\n",
        );

        assert!(matches!(
            error,
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "scale" && expected == "unsigned integer literal"
        ));
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
    fn rejects_a_foreign_type_named_after_a_known_column_type() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    value: foreign::Uuid,\n}\n"
            )
            .contains("cannot be mapped to an SQL type")
        );
    }

    #[test]
    fn rejects_a_field_with_repeated_column_attributes() {
        assert_eq!(
            indexing_rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[column]\n    id: i64,\n}\n"
            ),
            "attribute 'column' is repeated on 'crate::S::id' but a single occurrence was expected"
        );
    }

    #[test]
    fn derives_one_column_per_target_key_column_of_a_composite_foreign_key() {
        let source = "#[model(table = \"orders\")]\n#[primary_key(columns = [region, number])]\nstruct Order {\n    #[column]\n    region: String,\n    #[column]\n    number: i64,\n}\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    order: Order,\n}\n";

        let foreign_keys = resolved_model(source, "line_items").foreign_keys;

        assert_eq!(foreign_keys.len(), 1);
        assert_eq!(
            foreign_keys[0].columns,
            vec!["order_region".to_string(), "order_number".to_string()]
        );
        assert_eq!(
            foreign_keys[0].references_columns,
            vec!["region".to_string(), "number".to_string()]
        );
        assert_eq!(foreign_keys[0].references_table, "orders");
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
        assert_eq!(
            indexing_rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[foreign_key]\n    author: Author,\n}\n",
            )),
            "attribute 'foreign_key' is repeated on 'crate::Article::author' but a single occurrence was expected"
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

        assert!(error_message(&source).contains("table name that is too long"));
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
    fn rejects_a_primary_key_index_name_that_is_too_long() {
        let table = "a".repeat(60);
        let source = format!(
            "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n"
        );

        assert!(error_message(&source).contains("primary key index name that is too long"));
    }

    #[test]
    fn rejects_a_unique_index_name_that_is_too_long() {
        let column = "a".repeat(60);
        let source = format!(
            "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    {column}: String,\n}}\n"
        );

        assert!(error_message(&source).contains("unique index name that is too long"));
    }

    #[test]
    fn rejects_a_foreign_key_whose_derived_name_is_too_long() {
        let field = "a".repeat(62);
        let source = with_author(&format!(
            "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    {field}: Author,\n}}\n"
        ));

        assert!(error_message(&source).contains("foreign key field"));
    }

    #[test]
    fn rejects_an_unknown_on_delete_action() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::Purge)]\n    author: Author,\n}\n",
            ))
            .contains(
                "declares the ON DELETE action 'margaret::framework::model::on_delete::OnDelete::Purge'"
            )
        );
    }

    #[test]
    fn rejects_a_repeated_field_index() {
        assert_eq!(
            indexing_rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index]\n    #[index]\n    value: String,\n}\n",
            ),
            "attribute 'index' is repeated on 'crate::S::value' but a single occurrence was expected"
        );
    }

    #[test]
    fn rejects_a_repeated_named_index_on_a_column() {
        assert_eq!(
            indexing_rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = \"combo\")]\n    #[index(name = \"combo\")]\n    value: String,\n}\n",
            ),
            "attribute 'index' is repeated on 'crate::S::value' but a single occurrence was expected"
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
            .contains("leading column(s) of the primary key")
        );
    }

    #[test]
    fn rejects_an_index_on_the_leading_primary_key_column_of_a_composite_key() {
        assert!(
            error_message(
                "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, branch, hash])]\nstruct Lock {\n    #[column]\n    #[index]\n    repository: String,\n    #[column]\n    branch: String,\n    #[column]\n    hash: String,\n}\n",
            )
            .contains("leading column(s) of the primary key")
        );
    }

    #[test]
    fn accepts_an_index_on_a_non_leading_primary_key_column() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, branch, hash])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n    #[column]\n    #[index]\n    hash: String,\n}\n";

        assert_eq!(
            resolved_index_columns(source),
            vec![vec!["hash".to_string()]]
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
    fn rejects_a_single_column_named_index_on_a_unique_column() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[index(name = \"solo\")]\n    email: String,\n}\n",
            )
            .contains("is redundant")
        );
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

        assert!(error_message(source).contains("duplicate index name"));
    }

    #[test]
    fn rejects_an_explicit_index_name_that_collides_with_a_derived_name() {
        assert!(
            error_message(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    c: String,\n    #[column]\n    #[index(name = \"t_c_index\")]\n    d: String,\n}\n",
            )
            .contains("declares the index name")
        );
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_table_name() {
        assert!(
            error_message(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"authors\")]\n    title: String,\n}\n",
            ))
            .contains("collides with the table name")
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

        assert!(error_message(source).contains("collides with the table name"));
    }

    #[test]
    fn rejects_a_constraint_index_name_that_collides_with_a_table_name() {
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

        assert!(error_message(source).contains("collides with the table name"));
    }

    #[test]
    fn rejects_two_tables_that_generate_the_same_constraint_index_name() {
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

        assert!(error_message(source).contains("generated for both table"));
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_constraint_index() {
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

        assert!(error_message(source).contains("collides with the constraint-backing index"));
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
        let error = rejection_for(
            "#[model(table = \"t\")]\nstruct S {\n    #[column]\n    #[index(name = 5)]\n    value: String,\n}\n",
        );

        assert!(matches!(
            error,
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "name" && expected == "string literal"
        ));
    }

    fn table_order(lib_source: &str) -> Vec<String> {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .map(|model| model.table)
            .collect()
    }

    fn resolved_index_columns(lib_source: &str) -> Vec<Vec<String>> {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .flat_map(|model| model.indexes)
            .map(|index| index.columns)
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
        let source = "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    parent: Option<Box<Node>>,\n}\n";

        assert_eq!(table_order(source), ["nodes"]);
    }

    #[test]
    fn rejects_an_unboxed_self_referential_foreign_key() {
        let source = "#[model(table = \"nodes\")]\nstruct Node {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    parent: Option<Node>,\n}\n";

        assert!(error_message(source).contains("heap indirection"));
    }

    #[test]
    fn rejects_a_foreign_key_cycle() {
        let source = "#[model(table = \"alpha\")]\nstruct Alpha {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    beta: Beta,\n}\n\n#[model(table = \"beta\")]\nstruct Beta {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    alpha: Alpha,\n}\n";

        assert!(error_message(source).contains("foreign key dependency cycle"));
    }

    fn resolved_model(lib_source: &str, table: &str) -> Model {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .find(|model| model.table == table)
            .expect("the model resolves")
    }

    fn inferred_column(lib_source: &str, table: &str, column: &str) -> InferredColumn {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .find(|model| model.table == table)
            .expect("the model resolves")
            .columns
            .into_iter()
            .find(|resolved| resolved.name == column)
            .expect("the column resolves")
            .inferred
    }

    fn inferred_value(
        lib_prelude: &str,
        value_type: &str,
    ) -> Result<InferredColumn, ModelCodegenError> {
        let indexed = IndexedSource::new(&format!(
            "{lib_prelude}\n#[model(table = \"values\")]\nstruct Values {{\n    #[column]\n    value: {value_type},\n}}\n"
        ));

        models(&indexed.index).map(|models| {
            models
                .into_iter()
                .flat_map(|model| model.columns)
                .find(|resolved| resolved.name == "value")
                .expect("the value column resolves")
                .inferred
        })
    }

    fn value_column_type(value_type: &str) -> ColumnType {
        inferred_value("", value_type)
            .expect("the value type is inferable")
            .column_type
    }

    #[test]
    fn infers_a_uuid_column_with_a_v7_default() {
        let inferred = inferred_value("", "uuid::Uuid").expect("a uuid is inferable");

        assert_eq!(inferred.column_type, ColumnType::Uuid);
        assert_eq!(inferred.default, ColumnDefault::UuidV7);
        assert!(!inferred.nullable);
    }

    #[test]
    fn infers_a_uuid_column_from_an_imported_uuid() {
        assert_eq!(
            inferred_value("use uuid::Uuid;", "Uuid")
                .expect("an imported uuid is inferable")
                .column_type,
            ColumnType::Uuid
        );
    }

    #[test]
    fn infers_text_from_string() {
        assert_eq!(value_column_type("String"), ColumnType::Text);
    }

    #[test]
    fn infers_boolean_from_bool() {
        assert_eq!(value_column_type("bool"), ColumnType::Boolean);
    }

    #[test]
    fn infers_integer_from_i32() {
        assert_eq!(value_column_type("i32"), ColumnType::Integer);
    }

    #[test]
    fn infers_big_int_from_i64() {
        assert_eq!(value_column_type("i64"), ColumnType::BigInt);
    }

    #[test]
    fn infers_real_from_f32() {
        assert_eq!(value_column_type("f32"), ColumnType::Real);
    }

    #[test]
    fn infers_double_precision_from_f64() {
        assert_eq!(value_column_type("f64"), ColumnType::DoublePrecision);
    }

    #[test]
    fn infers_timestamptz_from_an_imported_datetime() {
        let inferred = inferred_value("use chrono::DateTime;\nuse chrono::Utc;", "DateTime<Utc>")
            .expect("a datetime is inferable");

        assert_eq!(inferred.column_type, ColumnType::Timestamptz);
        assert_eq!(inferred.default, ColumnDefault::NotSet);
        assert!(!inferred.nullable);
    }

    #[test]
    fn treats_an_option_as_a_nullable_column() {
        let inferred =
            inferred_value("", "Option<String>").expect("an optional string is inferable");

        assert!(inferred.nullable);
        assert_eq!(inferred.column_type, ColumnType::Text);
        assert_eq!(inferred.default, ColumnDefault::NotSet);
    }

    #[test]
    fn infers_bytea_from_a_byte_vector() {
        let inferred = inferred_value("", "Vec<u8>").expect("a byte vector is inferable");

        assert_eq!(inferred.column_type, ColumnType::Bytea);
        assert!(!inferred.nullable);
    }

    #[test]
    fn treats_an_optional_byte_vector_as_a_nullable_bytea_column() {
        let inferred =
            inferred_value("", "Option<Vec<u8>>").expect("an optional byte vector is inferable");

        assert!(inferred.nullable);
        assert_eq!(inferred.column_type, ColumnType::Bytea);
    }

    #[test]
    fn rejects_a_vector_of_non_bytes() {
        assert!(inferred_value("", "Vec<i32>").is_err());
    }

    #[test]
    fn rejects_a_vector_of_a_non_path_element() {
        assert!(inferred_value("", "Vec<[u8; 4]>").is_err());
    }

    #[test]
    fn rejects_a_standard_library_type_that_is_not_a_column_type() {
        assert!(inferred_value("use std::sync::Arc;", "Arc<String>").is_err());
    }

    #[test]
    fn rejects_a_non_path_type() {
        assert!(inferred_value("", "[u8; 4]").is_err());
    }

    #[test]
    fn rejects_a_bare_option() {
        assert!(inferred_value("", "Option").is_err());
    }

    #[test]
    fn rejects_an_option_with_multiple_arguments() {
        assert!(inferred_value("", "Option<i32, i64>").is_err());
    }

    #[test]
    fn infers_a_text_column_from_a_unit_enum() {
        let source = "enum ArticleStatus {\n    Draft,\n    Published,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: ArticleStatus,\n    #[column]\n    cover: Vec<u8>,\n}\n";
        let inferred = inferred_column(source, "articles", "status");

        assert_eq!(inferred.column_type, ColumnType::Text);
        assert!(!inferred.nullable);
    }

    #[test]
    fn treats_an_optional_enum_as_a_nullable_text_column() {
        let source = "enum ArticleStatus {\n    Draft,\n    Published,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: Option<ArticleStatus>,\n}\n";
        let inferred = inferred_column(source, "articles", "status");

        assert_eq!(inferred.column_type, ColumnType::Text);
        assert!(inferred.nullable);
    }

    #[test]
    fn validates_an_enum_shared_by_two_columns_only_once() {
        let source = "enum ArticleStatus {\n    Draft,\n    Published,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: ArticleStatus,\n    #[column]\n    previous_status: ArticleStatus,\n}\n";

        assert_eq!(
            inferred_column(source, "articles", "previous_status").column_type,
            ColumnType::Text
        );
    }

    #[test]
    fn rejects_an_enum_column_with_a_data_carrying_variant() {
        let source = "enum ArticleStatus {\n    Draft,\n    Rejected(String),\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column]\n    status: ArticleStatus,\n}\n";

        assert!(error_message(source).contains("variant 'Rejected' carries data"));
    }

    #[test]
    fn rejects_an_empty_enum_column() {
        let source = "enum ArticleStatus {}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column]\n    status: ArticleStatus,\n}\n";

        assert!(error_message(source).contains("which has no variants"));
    }

    #[test]
    fn rejects_a_struct_column_that_is_not_a_foreign_key() {
        let source = "struct Author {}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column]\n    author: Author,\n}\n";

        assert!(error_message(source).contains(
            "resolves to 'crate::Author' declared in this crate but is not a fieldless enum"
        ));
    }

    #[test]
    fn infers_a_numeric_column_from_a_fully_qualified_decimal() {
        let source = "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(precision = 12, scale = 2)]\n    unit_price: rust_decimal::Decimal,\n}\n";

        assert_eq!(
            inferred_column(source, "line_items", "unit_price").column_type,
            ColumnType::Numeric {
                precision: 12,
                scale: 2
            }
        );
    }

    #[test]
    fn infers_a_numeric_column_from_an_imported_decimal() {
        let source = "use rust_decimal::Decimal;\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(precision = 4, scale = 4)]\n    rate: Decimal,\n}\n";

        assert_eq!(
            inferred_column(source, "line_items", "rate").column_type,
            ColumnType::Numeric {
                precision: 4,
                scale: 4
            }
        );
    }

    #[test]
    fn treats_an_optional_decimal_as_a_nullable_numeric_column() {
        let source = "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(precision = 12, scale = 2)]\n    discount: Option<rust_decimal::Decimal>,\n}\n";

        assert!(inferred_column(source, "line_items", "discount").nullable);
    }

    #[test]
    fn rejects_a_decimal_column_without_a_precision_and_scale() {
        let source = "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column]\n    unit_price: rust_decimal::Decimal,\n}\n";

        assert!(error_message(source).contains("requires an explicit precision and scale"));
    }

    #[test]
    fn does_not_treat_a_crate_local_decimal_as_a_numeric_column() {
        let source = "struct Decimal {}\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(precision = 12, scale = 2)]\n    unit_price: Decimal,\n}\n";

        assert!(
            error_message(source)
                .contains("declares a NUMERIC precision and scale but has the type 'Decimal'")
        );
    }

    #[test]
    fn rejects_a_precision_and_scale_on_a_non_numeric_column() {
        let source = "#[model(table = \"articles\")]\nstruct Article {\n    #[column(precision = 12, scale = 2)]\n    title: String,\n}\n";

        assert!(
            error_message(source)
                .contains("declares a NUMERIC precision and scale but has the type 'String'")
        );
    }

    #[test]
    fn rejects_a_precision_and_scale_on_an_enum_column() {
        let source = "enum ArticleStatus {\n    Draft,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(precision = 12, scale = 2)]\n    status: ArticleStatus,\n}\n";

        assert!(
            error_message(source).contains(
                "declares a NUMERIC precision and scale but has the type 'ArticleStatus'"
            )
        );
    }

    #[test]
    fn rejects_a_precision_and_scale_on_a_foreign_key() {
        let source = with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(precision = 12, scale = 2)]\n    #[foreign_key]\n    author: Author,\n}\n",
        );

        assert!(error_message(&source).contains("must not declare a NUMERIC precision or scale"));
    }

    #[test]
    fn rejects_a_precision_that_a_decimal_cannot_hold() {
        let source = "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(precision = 29, scale = 2)]\n    unit_price: rust_decimal::Decimal,\n}\n";

        assert!(error_message(source).contains("the precision must be between 1 and 28"));
    }

    const FRAGMENT_METADATA_MODEL: &str = "#[model(table = \"fragment_metadata\")]
#[primary_key(columns = [partition, hash])]
struct FragmentMetadata {
    #[column]
    partition: uuid::Uuid,
    #[column(byte_length = 32)]
    hash: Vec<u8>,
    #[column(minimum = 0)]
    size_payload: i64,
}
";

    fn with_fragment_metadata(referencing: &str) -> String {
        format!("{FRAGMENT_METADATA_MODEL}\n{referencing}")
    }

    fn column_checks(lib_source: &str, table: &str, column: &str) -> Vec<ColumnCheck> {
        let indexed = IndexedSource::new(lib_source);

        models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .find(|model| model.table == table)
            .expect("the model resolves")
            .columns
            .into_iter()
            .find(|resolved| resolved.name == column)
            .expect("the column resolves")
            .checks
    }

    #[test]
    fn resolves_a_byte_length_check_from_a_column_attribute() {
        assert_eq!(
            column_checks(FRAGMENT_METADATA_MODEL, "fragment_metadata", "hash"),
            vec![ColumnCheck {
                name: "fragment_metadata_hash_byte_length".to_string(),
                predicate: CheckPredicate::ByteLength { length: 32 },
            }]
        );
    }

    #[test]
    fn resolves_a_minimum_check_from_a_column_attribute() {
        assert_eq!(
            column_checks(FRAGMENT_METADATA_MODEL, "fragment_metadata", "size_payload"),
            vec![ColumnCheck {
                name: "fragment_metadata_size_payload_minimum".to_string(),
                predicate: CheckPredicate::Minimum { minimum: 0 },
            }]
        );
    }

    #[test]
    fn resolves_no_checks_for_a_column_that_declares_none() {
        assert!(
            column_checks(FRAGMENT_METADATA_MODEL, "fragment_metadata", "partition").is_empty()
        );
    }

    #[test]
    fn rejects_a_byte_length_on_a_text_column() {
        let source = "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key, byte_length = 32)]\n    name: String,\n}\n";

        assert!(error_message(source).contains("only applies to a BYTEA column"));
    }

    #[test]
    fn rejects_a_minimum_on_a_text_column() {
        let source = "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key, minimum = 0)]\n    name: String,\n}\n";

        assert!(error_message(source).contains("only applies to a numeric column"));
    }

    #[test]
    fn rejects_a_check_constraint_on_a_foreign_key_field() {
        let source = with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(byte_length = 32)]\n    #[foreign_key]\n    author: Author,\n}\n",
        );

        assert!(error_message(&source).contains("cannot declare a check constraint"));
    }

    #[test]
    fn resolves_a_composite_foreign_key_from_a_model_attribute() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, hash], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n    #[column]\n    context: uuid::Uuid,\n}\n",
        );

        let foreign_keys = resolved_model(&source, "fragment").foreign_keys;

        assert_eq!(foreign_keys.len(), 1);
        assert_eq!(
            foreign_keys[0].columns,
            vec!["partition".to_string(), "hash".to_string()]
        );
        assert_eq!(
            foreign_keys[0].references_columns,
            vec!["partition".to_string(), "hash".to_string()]
        );
        assert_eq!(foreign_keys[0].references_table, "fragment_metadata");
        assert_eq!(foreign_keys[0].on_delete, OnDelete::NoAction);
    }

    #[test]
    fn resolves_a_foreign_key_target_imported_through_a_reexport() {
        let source = "mod metadata {\n    mod fragment {\n        #[model(table = \"fragment_metadata\")]\n        pub struct FragmentMetadata {\n            #[column(primary_key)]\n            pub hash: Vec<u8>,\n        }\n    }\n\n    pub use fragment::FragmentMetadata;\n}\n\nuse crate::metadata::FragmentMetadata;\n\n#[model(table = \"fragment\")]\n#[foreign_key(columns = [hash], references = FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert_eq!(
            resolved_model(source, "fragment").foreign_keys[0].references_table,
            "fragment_metadata"
        );
    }

    #[test]
    fn orders_a_composite_foreign_key_target_before_the_model_that_references_it() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, hash], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );
        let indexed = IndexedSource::new(&source);

        let tables: Vec<String> = models(&indexed.index)
            .expect("the models resolve")
            .into_iter()
            .map(|model| model.table)
            .collect();

        assert_eq!(
            tables,
            vec!["fragment_metadata".to_string(), "fragment".to_string()]
        );
    }

    #[test]
    fn rejects_a_composite_foreign_key_to_a_type_that_is_not_a_model() {
        let source = "#[model(table = \"fragment\")]\n#[foreign_key(columns = [hash], references = crate::Missing)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("which is not a model declared with #[model]"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_to_a_model_without_a_primary_key() {
        let source = "#[model(table = \"metadata\")]\nstruct Metadata {\n    #[column]\n    hash: Vec<u8>,\n}\n\n#[model(table = \"fragment\")]\n#[foreign_key(columns = [hash], references = crate::Metadata)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("which has no primary key"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_with_the_wrong_number_of_columns() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[foreign_key(columns = [hash], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(error_message(&source).contains("column(s) but"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_on_a_column_the_model_does_not_declare() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, digest], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(error_message(&source).contains("which the model does not declare"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_that_repeats_a_column() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, partition], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(error_message(&source).contains("more than once"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_whose_column_type_differs_from_the_target() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [hash, partition], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(error_message(&source).contains("whose type differs from column"));
    }

    #[test]
    fn rejects_two_composite_foreign_keys_on_the_same_columns() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [partition, hash], references = crate::FragmentMetadata)]\n#[foreign_key(columns = [partition, hash], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column]\n    partition: uuid::Uuid,\n    #[column]\n    hash: Vec<u8>,\n}\n",
        );

        assert!(error_message(&source).contains("more than one foreign key on columns"));
    }

    #[test]
    fn rejects_a_byte_length_that_is_not_an_unsigned_integer() {
        let source = "#[model(table = \"fragments\")]\nstruct Fragment {\n    #[column(primary_key, byte_length = \"thirty two\")]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("'byte_length'"));
    }

    #[test]
    fn rejects_a_minimum_that_is_not_an_unsigned_integer() {
        let source = "#[model(table = \"fragments\")]\nstruct Fragment {\n    #[column(primary_key, minimum = \"zero\")]\n    size_payload: i64,\n}\n";

        assert!(error_message(source).contains("'minimum'"));
    }

    #[test]
    fn rejects_a_zero_byte_length_on_a_column() {
        let source = "#[model(table = \"fragments\")]\nstruct Fragment {\n    #[column(primary_key, byte_length = 0)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("must require at least one byte"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_whose_target_path_does_not_resolve() {
        let source = "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(columns = [hash], references = Missing)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("which is not a model declared with #[model]"));
    }

    #[test]
    fn rejects_malformed_composite_foreign_key_arguments() {
        let source = "#[model(table = \"fragment\")]\n#[primary_key(columns = [partition, hash])]\n#[foreign_key(= 5)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("failed to read the model attributes"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_without_columns() {
        let source = "#[model(table = \"fragment\")]\n#[foreign_key(references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n";

        assert!(error_message(source).contains("requires the columns it spans"));
    }

    #[test]
    fn resolves_a_composite_primary_key_in_the_declared_order() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [branch, repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

        assert_eq!(
            resolved_model(source, "locks").primary_key,
            vec!["branch".to_string(), "repository".to_string()]
        );
    }

    #[test]
    fn rejects_two_fields_flagged_as_the_primary_key() {
        let source = "#[model(table = \"locks\")]\nstruct Lock {\n    #[column(primary_key)]\n    repository: String,\n    #[column(primary_key)]\n    branch: String,\n}\n";

        assert!(
            error_message(source).contains("marks more than one field with #[column(primary_key)]")
        );
    }

    #[test]
    fn rejects_a_primary_key_declared_on_both_a_column_and_the_model() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, branch])]\nstruct Lock {\n    #[column(primary_key)]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

        assert!(error_message(source).contains("declares a primary key both with"));
    }

    #[test]
    fn rejects_more_than_one_model_primary_key() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, branch])]\n#[primary_key(columns = [branch, repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

        assert_eq!(
            indexing_rejection(source),
            "attribute 'primary_key' is repeated on 'crate::Lock' but a single occurrence was expected"
        );
    }

    #[test]
    fn rejects_a_model_primary_key_column_the_model_does_not_declare() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, missing])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

        assert!(
            error_message(source).contains("never of a column derived from a #[foreign_key] field")
        );
    }

    #[test]
    fn rejects_a_model_primary_key_over_a_foreign_key_column() {
        let source = with_author(
            "#[model(table = \"articles\")]\n#[primary_key(columns = [author_id, title])]\nstruct Article {\n    #[column]\n    title: String,\n    #[column]\n    #[foreign_key]\n    author: Author,\n}\n",
        );

        assert!(
            error_message(&source)
                .contains("never of a column derived from a #[foreign_key] field")
        );
    }

    #[test]
    fn rejects_a_primary_key_attribute_on_a_field() {
        let source = "#[model(table = \"locks\")]\nstruct Lock {\n    #[column]\n    #[primary_key(columns = [repository, branch])]\n    repository: String,\n}\n";

        assert!(error_message(source).contains("carries #[primary_key]"));
    }

    #[test]
    fn rejects_a_unique_attribute_on_a_field() {
        let source = "#[model(table = \"locks\")]\nstruct Lock {\n    #[column(primary_key)]\n    #[unique(columns = [repository, branch])]\n    repository: String,\n}\n";

        assert!(error_message(source).contains("carries #[unique]"));
    }

    #[test]
    fn resolves_a_composite_unique_constraint_from_a_model_attribute() {
        let source = "#[model(table = \"members\")]\n#[unique(columns = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n";

        assert_eq!(
            resolved_model(source, "members")
                .unique_constraints
                .into_iter()
                .map(|constraint| constraint.columns)
                .collect::<Vec<Vec<String>>>(),
            vec![vec!["email".to_string(), "label".to_string()]]
        );
    }

    #[test]
    fn rejects_a_model_unique_column_the_model_does_not_declare() {
        let source = "#[model(table = \"members\")]\n#[unique(columns = [email, missing])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n}\n";

        assert!(error_message(source).contains("which the model does not declare"));
    }

    #[test]
    fn rejects_two_model_unique_constraints_over_the_same_columns() {
        let source = "#[model(table = \"members\")]\n#[unique(columns = [email, label])]\n#[unique(columns = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n";

        assert!(error_message(source).contains("more than one unique constraint on columns"));
    }

    #[test]
    fn resolves_a_composite_index_from_a_model_attribute() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_kind_label\", columns = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert_eq!(
            resolved_index_columns(source),
            vec![vec!["kind".to_string(), "label".to_string()]]
        );
    }

    #[test]
    fn rejects_a_model_index_without_a_name() {
        let source = "#[model(table = \"events\")]\n#[index(columns = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert!(error_message(source).contains("requires a name"));
    }

    #[test]
    fn rejects_a_model_index_column_the_model_does_not_declare() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_kind_label\", columns = [kind, missing])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n}\n";

        assert!(error_message(source).contains("which the model does not declare"));
    }

    #[test]
    fn rejects_a_field_index_that_names_columns() {
        let source = "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(columns = [kind])]\n    kind: String,\n}\n";

        assert!(error_message(source).contains("must not name columns"));
    }

    #[test]
    fn rejects_two_fields_that_share_an_index_name() {
        let source = "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"events_pair\")]\n    kind: String,\n    #[column]\n    #[index(name = \"events_pair\")]\n    label: String,\n}\n";

        assert!(error_message(source).contains("declares the index name"));
    }

    #[test]
    fn rejects_a_model_index_that_duplicates_the_primary_key() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository, branch])]\n#[index(name = \"locks_repository_branch\", columns = [repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

        assert!(error_message(source).contains("leading column(s) of the primary key"));
    }

    #[test]
    fn rejects_a_model_index_that_duplicates_a_unique_constraint() {
        let source = "#[model(table = \"members\")]\n#[unique(columns = [email, label])]\n#[index(name = \"members_email_label\", columns = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n";

        assert!(error_message(source).contains("a unique constraint is already indexed"));
    }

    #[test]
    fn treats_an_optional_composite_foreign_key_as_nullable_columns() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    metadata: Option<FragmentMetadata>,\n}\n",
        );

        let columns = resolved_model(&source, "fragment").columns;

        assert!(
            columns
                .iter()
                .filter(|column| column.name.starts_with("metadata_"))
                .all(|column| column.inferred.nullable)
        );
    }

    #[test]
    fn copies_each_target_key_column_type_onto_its_derived_column() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    metadata: FragmentMetadata,\n}\n",
        );

        let columns = resolved_model(&source, "fragment").columns;
        let derived: Vec<(String, ColumnType)> = columns
            .into_iter()
            .filter(|column| column.name.starts_with("metadata_"))
            .map(|column| (column.name, column.inferred.column_type))
            .collect();

        assert_eq!(
            derived,
            vec![
                ("metadata_partition".to_string(), ColumnType::Uuid),
                ("metadata_hash".to_string(), ColumnType::Bytea),
            ]
        );
    }

    #[test]
    fn indexes_every_column_of_a_composite_foreign_key_field() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[index]\n    metadata: FragmentMetadata,\n}\n",
        );

        assert_eq!(
            resolved_index_columns(&source),
            vec![vec![
                "metadata_partition".to_string(),
                "metadata_hash".to_string()
            ]]
        );
    }

    #[test]
    fn constrains_every_column_of_a_unique_composite_foreign_key_field() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[foreign_key]\n    metadata: FragmentMetadata,\n}\n",
        );

        assert_eq!(
            resolved_model(&source, "fragment")
                .unique_constraints
                .into_iter()
                .map(|constraint| constraint.columns)
                .collect::<Vec<Vec<String>>>(),
            vec![vec![
                "metadata_partition".to_string(),
                "metadata_hash".to_string()
            ]]
        );
    }

    #[test]
    fn rejects_a_model_index_name_that_is_not_a_string() {
        let source = "#[model(table = \"events\")]\n#[index(name = 5, columns = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert!(error_message(source).contains("is not a string literal"));
    }

    #[test]
    fn rejects_a_model_index_over_a_single_column() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_kind\", columns = [kind])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n}\n";

        assert!(error_message(source).contains("names a single column"));
    }

    #[test]
    fn rejects_a_model_index_name_that_is_not_snake_case() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"Events\", columns = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert!(error_message(source).contains("invalid index name"));
    }

    #[test]
    fn rejects_a_model_primary_key_over_a_single_column() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(columns = [repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n";

        assert!(error_message(source).contains("names a single column"));
    }

    #[test]
    fn rejects_a_model_unique_over_a_single_column() {
        let source = "#[model(table = \"members\")]\n#[unique(columns = [email])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n}\n";

        assert!(error_message(source).contains("names a single column"));
    }

    #[test]
    fn rejects_malformed_model_primary_key_arguments() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(= 5)]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n";

        assert!(error_message(source).contains("failed to read the model attributes"));
    }

    #[test]
    fn rejects_malformed_model_unique_arguments() {
        let source = "#[model(table = \"members\")]\n#[unique(= 5)]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n";

        assert!(error_message(source).contains("failed to read the model attributes"));
    }

    #[test]
    fn rejects_malformed_model_index_arguments() {
        let source = "#[model(table = \"events\")]\n#[index(= 5)]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n";

        assert!(error_message(source).contains("failed to read the model attributes"));
    }

    #[test]
    fn rejects_a_field_index_whose_columns_are_not_an_array() {
        let source = "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(columns = \"kind\")]\n    kind: String,\n}\n";

        assert!(error_message(source).contains("is not a array of paths"));
    }

    #[test]
    fn rejects_a_composite_foreign_key_field_whose_derived_index_name_is_too_long() {
        let long_field = "a".repeat(40);
        let source = with_fragment_metadata(&format!(
            "#[model(table = \"fragment\")]\nstruct Fragment {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[index]\n    {long_field}: FragmentMetadata,\n}}\n"
        ));

        assert!(error_message(&source).contains("index name that is too long"));
    }
}
