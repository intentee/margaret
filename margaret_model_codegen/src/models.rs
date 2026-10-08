use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::assemble_model::assemble_model;
use crate::assembled_model::AssembledModel;
use crate::collect_model::collect_model;
use crate::collected_model::CollectedModel;
use crate::enum_variants::EnumVariants;
use crate::link_relations::link_relations;
use crate::model::Model;
use crate::model_codegen_error::ModelCodegenError;
use crate::order_by_dependencies::order_by_dependencies;
use crate::primary_key_columns::PrimaryKeyColumns;
use crate::relation_namespace::RelationNamespace;

/// # Errors
///
/// Returns `ModelCodegenError` when a `#[model]` declaration of the indexed crate is malformed,
/// ambiguous or inconsistent with the other models.
pub fn models(
    index: &AttributeIndex,
    namespace: TableNamespace,
) -> Result<Vec<Model>, ModelCodegenError> {
    let mut enum_variants = EnumVariants::new();
    let mut relation_namespace = RelationNamespace::new();
    let mut collected: Vec<CollectedModel> = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::Model) {
        let model = collect_model(index, &matched, &mut enum_variants)?;

        relation_namespace.register_table(&model.table, &model.path.to_string())?;
        collected.push(model);
    }

    let mut registry: HashMap<CanonicalPath, &CollectedModel> = HashMap::new();

    for model in &collected {
        registry.insert(model.path.clone(), model);
    }

    let mut key_columns = PrimaryKeyColumns::new(index, &registry);
    let assembled = collected
        .iter()
        .map(|model| assemble_model(model, &mut key_columns, namespace))
        .collect::<Result<Vec<AssembledModel>, ModelCodegenError>>()?;
    let linked = link_relations(&assembled)?;

    relation_namespace.register_indexes(&linked)?;

    order_by_dependencies(linked)
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model::check_predicate::CheckPredicate;
    use margaret_model::column_default::ColumnDefault;
    use margaret_model::column_type::ColumnType;
    use margaret_model::on_delete::OnDelete;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::models;
    use crate::field_value::FieldValue;
    use crate::index_kind::IndexKind;
    use crate::model::Model;
    use crate::model_codegen_error::ModelCodegenError;
    use crate::model_index::ModelIndex;
    use crate::relation_kind::RelationKind;
    use crate::resolved_check::ResolvedCheck;
    use crate::resolved_column::ResolvedColumn;
    use crate::resolved_rust_type::ResolvedRustType;

    const KEY: &str = "use margaret::framework::active_record::key::Key;\n";

    const AUTHOR_MODEL: &str = "\
use margaret::framework::active_record::key::Key;

#[model(table = \"authors\")]
struct Author {
    #[column(primary_key)]
    id: uuid::Uuid,
}
";

    const FRAGMENT_METADATA_MODEL: &str = "\
use margaret::framework::active_record::key::Key;

#[model(table = \"fragment_metadata\")]
#[primary_key(fields = [partition, hash])]
struct FragmentMetadata {
    #[column]
    partition: uuid::Uuid,
    #[column(byte_length = 32)]
    hash: Vec<u8>,
    #[column(minimum = 0)]
    size_payload: i64,
}
";

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    fn resolved(lib_source: &str) -> Vec<Model> {
        models(
            &IndexedSource::new(lib_source).index,
            TableNamespace::Application,
        )
        .expect("the models resolve")
    }

    fn rejection(lib_source: &str) -> ModelCodegenError {
        models(
            &IndexedSource::new(lib_source).index,
            TableNamespace::Application,
        )
        .expect_err("the models fail to resolve")
    }

    fn indexing_rejection(lib_source: &str) -> String {
        IndexedSource::try_new(lib_source)
            .err()
            .expect("the repeated attribute is rejected while indexing")
            .to_string()
    }

    fn model(lib_source: &str, table: &str) -> Model {
        resolved(lib_source)
            .into_iter()
            .find(|model| model.table == table)
            .expect("the model resolves")
    }

    fn column(lib_source: &str, table: &str, name: &str) -> ResolvedColumn {
        model(lib_source, table)
            .columns()
            .find(|column| column.name == name)
            .expect("the column resolves")
            .clone()
    }

    fn value_model(prelude: &str, value_type: &str) -> String {
        format!(
            "{prelude}\n#[model(table = \"values\")]\nstruct Values {{\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    value: {value_type},\n}}\n"
        )
    }

    fn value_column(prelude: &str, value_type: &str) -> ResolvedColumn {
        column(&value_model(prelude, value_type), "values", "value")
    }

    fn value_rejection(prelude: &str, value_type: &str) -> ModelCodegenError {
        rejection(&value_model(prelude, value_type))
    }

    fn with_author(referencing: &str) -> String {
        format!("{AUTHOR_MODEL}\n{referencing}")
    }

    fn with_fragment_metadata(referencing: &str) -> String {
        format!("{FRAGMENT_METADATA_MODEL}\n{referencing}")
    }

    fn index_columns(lib_source: &str, table: &str, kind: IndexKind) -> Vec<Vec<String>> {
        model(lib_source, table)
            .indexes
            .of_kind(kind)
            .map(ModelIndex::column_names)
            .collect()
    }

    fn tables(lib_source: &str) -> Vec<String> {
        resolved(lib_source)
            .into_iter()
            .map(|model| model.table)
            .collect()
    }

    #[test]
    fn rejects_a_model_that_is_not_a_struct() {
        assert!(matches!(
                rejection("#[model(table = \"e\")] enum E {}\n"),
                ModelCodegenError::ModelNotAStruct { ref model, .. } if model == "crate::E"
        ));
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
        assert!(matches!(
                rejection("#[model]\nstruct S;\n"),
                ModelCodegenError::MissingTable { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_malformed_model_arguments() {
        assert!(matches!(
                rejection("#[model(= 5)]\nstruct S;\n"),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "model"
        ));
    }

    #[test]
    fn rejects_a_non_string_table_name() {
        assert!(matches!(
            rejection("#[model(table = 5)]\nstruct S;\n"),
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
        assert!(matches!(
                rejection("#[model(table = \"Articles\")]\nstruct S;\n"),
                ModelCodegenError::InvalidTableName { ref table, .. } if table == "Articles"
        ));
    }

    #[test]
    fn rejects_a_table_name_that_is_too_long() {
        let table = "a".repeat(64);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n"
                )),
                ModelCodegenError::TableNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_two_models_with_the_same_table() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct A {\n    #[column(primary_key)]\n    id: i64,\n}\n\n#[model(table = \"t\")]\nstruct B {\n    #[column(primary_key)]\n    id: i64,\n}\n"
                ),
                ModelCodegenError::DuplicateTableName { ref table, .. } if table == "t"
        ));
    }

    #[test]
    fn rejects_a_model_without_a_primary_key() {
        assert!(matches!(
                rejection("#[model(table = \"t\")]\nstruct S {\n    #[column]\n    id: i64,\n}\n"),
                ModelCodegenError::ModelRequiresPrimaryKey { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_field_without_a_column_attribute() {
        assert!(matches!(
                rejection("#[model(table = \"t\")]\nstruct S {\n    id: i64,\n}\n"),
                ModelCodegenError::UnattributedField { ref field, .. } if field == "id"
        ));
    }

    #[test]
    fn rejects_a_positional_field() {
        assert!(matches!(
            rejection("#[model(table = \"t\")]\nstruct S(#[column(primary_key)] i64);\n"),
            ModelCodegenError::ModelRequiresNamedFields { position: 0, ref model } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_field_name_that_is_not_snake_case() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    myId: i64,\n}\n"
                ),
                ModelCodegenError::FieldNameNotSnakeCase { ref field, .. } if field == "myId"
        ));
    }

    #[test]
    fn rejects_malformed_column_arguments() {
        assert!(matches!(
                rejection("#[model(table = \"t\")]\nstruct S {\n    #[column(= 5)]\n    id: i64,\n}\n"),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "column"
        ));
    }

    #[test]
    fn rejects_a_non_string_column_name() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(name = 5)]\n    id: i64,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "name"
        ));
    }

    #[test]
    fn rejects_a_non_integer_precision() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key, precision = \"12\", scale = 2)]\n    value: rust_decimal::Decimal,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "precision"
        ));
    }

    #[test]
    fn rejects_a_non_integer_scale() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key, precision = 12, scale = \"2\")]\n    value: rust_decimal::Decimal,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "scale"
        ));
    }

    #[test]
    fn rejects_a_column_name_that_is_not_snake_case() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key, name = \"Id\")]\n    id: i64,\n}\n"
                ),
                ModelCodegenError::InvalidColumnName { ref column, .. } if column == "Id"
        ));
    }

    #[test]
    fn rejects_a_column_name_that_is_too_long() {
        let name = "a".repeat(64);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"{name}\")]\n    value: String,\n}}\n"
                )),
                ModelCodegenError::ColumnNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_duplicate_column_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key, name = \"value\")]\n    a: i64,\n    #[column(name = \"value\")]\n    b: i64,\n}\n"
                ),
                ModelCodegenError::DuplicateColumnName { ref column, .. } if column == "value"
        ));
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
    fn rejects_an_uninferrable_column_type() {
        assert!(matches!(
                value_rejection("", "u64"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "u64"
        ));
    }

    #[test]
    fn rejects_a_foreign_type_named_after_a_known_column_type() {
        assert!(matches!(
                value_rejection("", "foreign::Uuid"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "foreign :: Uuid"
        ));
    }

    #[test]
    fn infers_a_uuid_column() {
        let resolved = value_column("", "uuid::Uuid");

        assert_eq!(resolved.column_type, ColumnType::Uuid);
        assert!(!resolved.nullable);
    }

    #[test]
    fn infers_a_uuid_column_from_an_imported_uuid() {
        assert_eq!(
            value_column("use uuid::Uuid;", "Uuid").column_type,
            ColumnType::Uuid
        );
    }

    #[test]
    fn infers_text_from_string() {
        assert_eq!(value_column("", "String").column_type, ColumnType::Text);
    }

    #[test]
    fn infers_text_from_secret_text() {
        assert_eq!(
            value_column(
                "use margaret::framework::active_record::secret_text::SecretText;\n",
                "SecretText"
            )
            .column_type,
            ColumnType::Text
        );
    }

    #[test]
    fn infers_boolean_from_bool() {
        assert_eq!(value_column("", "bool").column_type, ColumnType::Boolean);
    }

    #[test]
    fn infers_integer_from_i32() {
        assert_eq!(value_column("", "i32").column_type, ColumnType::Integer);
    }

    #[test]
    fn infers_big_int_from_i64() {
        assert_eq!(value_column("", "i64").column_type, ColumnType::BigInt);
    }

    #[test]
    fn infers_real_from_f32() {
        assert_eq!(value_column("", "f32").column_type, ColumnType::Real);
    }

    #[test]
    fn infers_double_precision_from_f64() {
        assert_eq!(
            value_column("", "f64").column_type,
            ColumnType::DoublePrecision
        );
    }

    #[test]
    fn infers_timestamptz_from_an_imported_utc_datetime() {
        let resolved = value_column("use chrono::DateTime;\nuse chrono::Utc;", "DateTime<Utc>");

        assert_eq!(resolved.column_type, ColumnType::Timestamptz);
        assert!(!resolved.nullable);
    }

    #[test]
    fn rejects_a_datetime_outside_utc() {
        assert!(matches!(
                value_rejection(
                    "use chrono::DateTime;\nuse chrono::FixedOffset;",
                    "DateTime<FixedOffset>"
                ),
                ModelCodegenError::TimestamptzRequiresUtc { ref rust_type, .. } if rust_type == "DateTime < FixedOffset >"
        ));
    }

    #[test]
    fn treats_an_option_as_a_nullable_column() {
        let resolved = value_column("", "Option<String>");

        assert!(resolved.nullable);
        assert_eq!(resolved.column_type, ColumnType::Text);
    }

    #[test]
    fn infers_bytea_from_a_byte_vector() {
        let resolved = value_column("", "Vec<u8>");

        assert_eq!(resolved.column_type, ColumnType::Bytea);
        assert!(!resolved.nullable);
    }

    #[test]
    fn treats_an_optional_byte_vector_as_a_nullable_bytea_column() {
        let resolved = value_column("", "Option<Vec<u8>>");

        assert!(resolved.nullable);
        assert_eq!(resolved.column_type, ColumnType::Bytea);
    }

    #[test]
    fn rejects_a_vector_of_non_bytes() {
        assert!(matches!(
                value_rejection("", "Vec<i32>"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "Vec < i32 >"
        ));
    }

    #[test]
    fn rejects_a_vector_of_a_non_path_element() {
        assert!(matches!(
                value_rejection("", "Vec<[u8; 4]>"),
                ModelCodegenError::UnresolvableFieldType { ref rust_type, .. } if rust_type == "[u8 ; 4]"
        ));
    }

    #[test]
    fn rejects_a_standard_library_type_that_is_not_a_column_type() {
        assert!(matches!(
                value_rejection("use std::sync::Arc;", "Arc<String>"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "Arc < String >"
        ));
    }

    #[test]
    fn rejects_a_non_path_type() {
        assert!(matches!(
                value_rejection("", "[u8; 4]"),
                ModelCodegenError::UnresolvableFieldType { ref rust_type, .. } if rust_type == "[u8 ; 4]"
        ));
    }

    #[test]
    fn rejects_a_bare_option() {
        assert!(matches!(
                value_rejection("", "Option"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "Option"
        ));
    }

    #[test]
    fn rejects_an_option_with_multiple_arguments() {
        assert!(matches!(
                value_rejection("", "Option<i32, i64>"),
                ModelCodegenError::UninferrableColumnType { ref rust_type, .. } if rust_type == "Option < i32 , i64 >"
        ));
    }

    #[test]
    fn rejects_a_lifetime_type_argument() {
        assert!(matches!(
                value_rejection("", "Vec<'static>"),
                ModelCodegenError::UnresolvableFieldType { ref rust_type, .. } if rust_type == "Vec < 'static >"
        ));
    }

    #[test]
    fn records_the_rust_type_of_a_scalar_field() {
        assert_eq!(
            model(&value_model("", "Option<Vec<u8>>"), "values").fields[1].value,
            FieldValue::Scalar {
                rust_type: ResolvedRustType {
                    arguments: vec![ResolvedRustType {
                        arguments: Vec::new(),
                        path: path(&["u8"]),
                    }],
                    path: path(&["std", "vec", "Vec"]),
                },
            }
        );
    }

    #[test]
    fn defaults_a_uuid_primary_key_to_a_v7_identifier() {
        assert_eq!(
            column(
                "#[model(table = \"values\")]\nstruct Values {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n",
                "values",
                "id"
            )
            .default,
            ColumnDefault::UuidV7
        );
    }

    #[test]
    fn leaves_a_uuid_column_outside_the_primary_key_without_a_default() {
        assert_eq!(
            value_column("", "uuid::Uuid").default,
            ColumnDefault::NotSet
        );
    }

    #[test]
    fn leaves_a_uuid_column_of_a_composite_primary_key_without_a_default() {
        let source = "#[model(table = \"values\")]\n#[primary_key(fields = [left, right])]\nstruct Values {\n    #[column]\n    left: uuid::Uuid,\n    #[column]\n    right: uuid::Uuid,\n}\n";

        assert!(
            model(source, "values")
                .columns()
                .all(|column| column.default == ColumnDefault::NotSet)
        );
    }

    #[test]
    fn infers_a_text_column_from_a_unit_enum() {
        let source = "enum ArticleStatus {\n    Draft,\n    Published,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: ArticleStatus,\n}\n";
        let article = model(source, "articles");

        assert_eq!(article.fields[1].columns[0].column_type, ColumnType::Text);
        assert!(!article.fields[1].nullable);
        assert_eq!(
            article.fields[1].value,
            FieldValue::Enum {
                path: path(&["crate", "ArticleStatus"]),
                variants: vec!["Draft".to_string(), "Published".to_string()],
            }
        );
    }

    #[test]
    fn treats_an_optional_enum_as_a_nullable_text_column() {
        let source = "enum ArticleStatus {\n    Draft,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: Option<ArticleStatus>,\n}\n";

        assert!(column(source, "articles", "status").nullable);
    }

    #[test]
    fn validates_an_enum_shared_by_two_columns_only_once() {
        let source = "enum ArticleStatus {\n    Draft,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    status: ArticleStatus,\n    #[column]\n    previous_status: ArticleStatus,\n}\n";

        assert_eq!(
            model(source, "articles").fields[2].value,
            FieldValue::Enum {
                path: path(&["crate", "ArticleStatus"]),
                variants: vec!["Draft".to_string()],
            }
        );
    }

    #[test]
    fn rejects_an_enum_column_with_a_data_carrying_variant() {
        assert!(matches!(
            value_rejection("enum ArticleStatus {\n    Draft,\n    Rejected(String),\n}\n", "ArticleStatus"),
            ModelCodegenError::EnumColumnVariantNotUnit { ref variant, .. } if variant == "Rejected"
        ));
    }

    #[test]
    fn rejects_an_empty_enum_column() {
        assert!(matches!(
                value_rejection("enum ArticleStatus {}\n", "ArticleStatus"),
                ModelCodegenError::EmptyEnumColumn { ref enum_type, .. } if enum_type == "crate::ArticleStatus"
        ));
    }

    #[test]
    fn rejects_a_crate_struct_column() {
        assert!(matches!(
            value_rejection("struct Author {}\n", "Author"),
            ModelCodegenError::LocalColumnTypeIsNotAnEnum { ref resolved_type, .. } if resolved_type == "crate::Author"
        ));
    }

    #[test]
    fn rejects_a_field_typed_as_another_model() {
        assert!(matches!(
            rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    author: Author,\n}\n"
            )),
            ModelCodegenError::ModelFieldRequiresKey { ref target, .. } if target == "crate::Author"
        ));
    }

    #[test]
    fn infers_a_numeric_column_from_a_fully_qualified_decimal() {
        assert_eq!(
            column(
                "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    unit_price: rust_decimal::Decimal,\n}\n",
                "line_items",
                "unit_price"
            )
            .column_type,
            ColumnType::Numeric {
                precision: 12,
                scale: 2
            }
        );
    }

    #[test]
    fn infers_a_numeric_column_from_an_imported_decimal() {
        assert_eq!(
            column(
                "use rust_decimal::Decimal;\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 4, scale = 4)]\n    rate: Decimal,\n}\n",
                "line_items",
                "rate"
            )
            .column_type,
            ColumnType::Numeric {
                precision: 4,
                scale: 4
            }
        );
    }

    #[test]
    fn treats_an_optional_decimal_as_a_nullable_numeric_column() {
        assert!(
            column(
                "#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    discount: Option<rust_decimal::Decimal>,\n}\n",
                "line_items",
                "discount"
            )
            .nullable
        );
    }

    #[test]
    fn rejects_a_decimal_column_without_a_precision_and_scale() {
        assert!(matches!(
                value_rejection("", "rust_decimal::Decimal"),
                ModelCodegenError::NumericColumnRequiresDigits { ref field, .. } if field == "value"
        ));
    }

    #[test]
    fn rejects_a_precision_and_scale_on_a_crate_local_decimal() {
        assert!(matches!(
                rejection(
                    "struct Decimal {}\n\n#[model(table = \"line_items\")]\nstruct LineItem {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    unit_price: Decimal,\n}\n"
                ),
                ModelCodegenError::NumericDigitsOnNonNumericColumn { ref rust_type, .. } if rust_type == "Decimal"
        ));
    }

    #[test]
    fn rejects_a_precision_and_scale_on_a_text_column() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    title: String,\n}\n"
                ),
                ModelCodegenError::NumericDigitsOnNonNumericColumn { ref rust_type, .. } if rust_type == "String"
        ));
    }

    #[test]
    fn rejects_a_precision_and_scale_on_an_enum_column() {
        assert!(matches!(
                rejection(
                    "enum ArticleStatus {\n    Draft,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    status: ArticleStatus,\n}\n"
                ),
                ModelCodegenError::NumericDigitsOnNonNumericColumn { ref rust_type, .. } if rust_type == "ArticleStatus"
        ));
    }

    #[test]
    fn rejects_a_precision_and_scale_on_a_key_field() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: i64,\n    #[column(precision = 12, scale = 2)]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::NumericDigitsOnNonNumericColumn { ref rust_type, .. } if rust_type == "Key < Author >"
        ));
    }

    #[test]
    fn resolves_a_byte_length_check_from_a_column_attribute() {
        assert_eq!(
            column(FRAGMENT_METADATA_MODEL, "fragment_metadata", "hash").checks,
            vec![ResolvedCheck {
                name: "fragment_metadata_hash_byte_length".to_string(),
                predicate: CheckPredicate::ByteLength { length: 32 },
            }]
        );
    }

    #[test]
    fn rejects_a_byte_length_that_is_not_an_unsigned_integer() {
        assert!(matches!(
            rejection(
                "#[model(table = \"fragments\")]\nstruct Fragment {\n    #[column(primary_key, byte_length = \"thirty two\")]\n    hash: Vec<u8>,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "byte_length"
        ));
    }

    #[test]
    fn rejects_a_minimum_that_is_not_an_unsigned_integer() {
        assert!(matches!(
            rejection(
                "#[model(table = \"fragments\")]\nstruct Fragment {\n    #[column(primary_key, minimum = \"zero\")]\n    size_payload: i64,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "minimum"
        ));
    }

    #[test]
    fn records_the_payload_of_a_json_field() {
        let values = model(
            &value_model(
                "use margaret::framework::active_record::json::Json;\nuse std::collections::BTreeSet;",
                "Json<BTreeSet<String>>",
            ),
            "values",
        );

        assert_eq!(values.fields[1].columns[0].column_type, ColumnType::Text);
        assert_eq!(
            values.fields[1].value,
            FieldValue::Json {
                payload: ResolvedRustType {
                    arguments: vec![ResolvedRustType {
                        arguments: Vec::new(),
                        path: path(&["std", "string", "String"]),
                    }],
                    path: path(&["std", "collections", "BTreeSet"]),
                },
            }
        );
    }

    #[test]
    fn rejects_a_json_field_without_its_payload() {
        assert!(matches!(
                value_rejection(
                    "use margaret::framework::active_record::json::Json;",
                    "Json"
                ),
                ModelCodegenError::WrapperRequiresOneArgument { ref wrapper, .. } if wrapper == "margaret::framework::active_record::json::Json"
        ));
    }

    #[test]
    fn derives_one_column_per_target_key_column_of_a_composite_key() {
        let source = format!(
            "{KEY}#[model(table = \"orders\")]\n#[primary_key(fields = [region, number])]\nstruct Order {{\n    #[column]\n    region: String,\n    #[column]\n    number: i64,\n}}\n\n#[model(table = \"line_items\")]\nstruct LineItem {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    order: Key<Order>,\n}}\n"
        );
        let line_items = model(&source, "line_items");

        assert_eq!(line_items.foreign_keys.len(), 1);
        assert_eq!(
            line_items.foreign_keys[0].columns,
            vec!["order_region".to_string(), "order_number".to_string()]
        );
        assert_eq!(
            line_items.foreign_keys[0].references_columns,
            vec!["region".to_string(), "number".to_string()]
        );
        assert_eq!(line_items.foreign_keys[0].references_table, "orders");
        assert_eq!(line_items.foreign_keys[0].on_delete, OnDelete::NoAction);
        assert_eq!(
            line_items.fields[1].value,
            FieldValue::Key {
                on_delete: OnDelete::NoAction,
                target: path(&["crate", "Order"]),
            }
        );
    }

    #[test]
    fn derives_the_columns_of_a_key_through_a_nested_key() {
        let source = format!(
            "{AUTHOR_MODEL}\n#[model(table = \"translations\")]\n#[primary_key(fields = [author, locale])]\nstruct Translation {{\n    #[column]\n    author: Key<Author>,\n    #[column]\n    locale: String,\n}}\n\n#[model(table = \"notes\")]\nstruct Note {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::Cascade)]\n    #[index]\n    source: Key<Translation>,\n}}\n"
        );
        let notes = model(&source, "notes");

        assert_eq!(
            notes.foreign_keys[0].columns,
            vec!["source_author_id".to_string(), "source_locale".to_string()]
        );
        assert_eq!(
            notes.foreign_keys[0].references_columns,
            vec!["author_id".to_string(), "locale".to_string()]
        );
        assert_eq!(notes.foreign_keys[0].on_delete, OnDelete::Cascade);
        assert_eq!(
            index_columns(&source, "translations", IndexKind::PrimaryKey),
            vec![vec!["author_id".to_string(), "locale".to_string()]]
        );
    }

    #[test]
    fn accepts_a_key_field_as_the_primary_key() {
        let source = with_author(
            "#[model(table = \"profiles\")]\nstruct Profile {\n    #[column(primary_key)]\n    author: Key<Author>,\n    #[column]\n    bio: String,\n}\n",
        );

        assert_eq!(
            index_columns(&source, "profiles", IndexKind::PrimaryKey),
            vec![vec!["author_id".to_string()]]
        );
        assert_eq!(
            column(&source, "profiles", "author_id").default,
            ColumnDefault::NotSet
        );
    }

    #[test]
    fn treats_an_optional_composite_key_as_nullable_columns() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    metadata: Option<Key<FragmentMetadata>>,\n}\n",
        );

        assert!(
            model(&source, "fragment").fields[1]
                .columns
                .iter()
                .all(|column| column.nullable)
        );
    }

    #[test]
    fn copies_each_target_key_column_type_onto_its_derived_column() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    metadata: Key<FragmentMetadata>,\n}\n",
        );
        let derived = &model(&source, "fragment").fields[1].columns;

        assert_eq!(
            derived
                .iter()
                .map(|column| (column.name.as_str(), column.column_type))
                .collect::<Vec<_>>(),
            vec![
                ("metadata_partition", ColumnType::Uuid),
                ("metadata_hash", ColumnType::Bytea)
            ]
        );
    }

    #[test]
    fn indexes_every_column_of_a_composite_key_field() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    metadata: Key<FragmentMetadata>,\n}\n",
        );

        assert_eq!(
            index_columns(&source, "fragment", IndexKind::Plain),
            vec![vec![
                "metadata_partition".to_string(),
                "metadata_hash".to_string()
            ]]
        );
    }

    #[test]
    fn constrains_every_column_of_a_unique_composite_key_field() {
        let source = with_fragment_metadata(
            "#[model(table = \"fragment\")]\nstruct Fragment {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    metadata: Key<FragmentMetadata>,\n}\n",
        );

        assert_eq!(
            index_columns(&source, "fragment", IndexKind::Unique),
            vec![vec![
                "metadata_partition".to_string(),
                "metadata_hash".to_string()
            ]]
        );
    }

    #[test]
    fn resolves_a_key_target_imported_through_a_reexport() {
        let source = "use margaret::framework::active_record::key::Key;\n\nmod metadata {\n    mod fragment {\n        #[model(table = \"fragment_metadata\")]\n        pub struct FragmentMetadata {\n            #[column(primary_key)]\n            pub hash: Vec<u8>,\n        }\n    }\n\n    pub use fragment::FragmentMetadata;\n}\n\nuse crate::metadata::FragmentMetadata;\n\n#[model(table = \"fragment\")]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    metadata: Key<FragmentMetadata>,\n}\n";

        assert_eq!(
            model(source, "fragment").foreign_keys[0].references_table,
            "fragment_metadata"
        );
    }

    #[test]
    fn rejects_a_struct_level_foreign_key() {
        assert!(matches!(
                rejection(&with_fragment_metadata(
                    "#[model(table = \"fragment\")]\n#[foreign_key(columns = [hash], references = crate::FragmentMetadata)]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    hash: Vec<u8>,\n}\n"
                )),
                ModelCodegenError::ForeignKeyIsNotAModelAttribute { ref model, .. } if model == "crate::FragmentAssociation"
        ));
    }

    #[test]
    fn rejects_a_key_field_without_a_column() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::Cascade)]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::UnattributedField { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_a_key_field_that_sets_a_column_name() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"writer\")]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::ForeignKeyColumnNameIsDerived { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_a_check_constraint_on_a_key_field() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(byte_length = 32)]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::ForeignKeyCannotDeclareCheckConstraint { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_a_foreign_key_attribute_on_a_field_that_is_not_a_key() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::Cascade)]\n    author_id: uuid::Uuid,\n}\n"
                ),
                ModelCodegenError::ForeignKeyRequiresKeyField { ref field, .. } if field == "author_id"
        ));
    }

    #[test]
    fn rejects_a_foreign_key_attribute_without_an_on_delete_action() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::ForeignKeyRequiresOnDelete { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_an_unknown_on_delete_action() {
        assert!(matches!(
            rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::Purge)]\n    #[index]\n    author: Key<Author>,\n}\n"
            )),
            ModelCodegenError::UnknownOnDeleteAction { ref action, .. } if action == "margaret::framework::model::on_delete::OnDelete::Purge"
        ));
    }

    #[test]
    fn rejects_set_null_on_a_required_key() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::SetNull)]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::SetNullRequiresNullableKey { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn accepts_set_null_on_an_optional_key() {
        let source = with_author(
            "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = margaret::framework::model::on_delete::OnDelete::SetNull)]\n    #[index]\n    author: Option<Key<Author>>,\n}\n",
        );

        assert_eq!(
            model(&source, "articles").foreign_keys[0].on_delete,
            OnDelete::SetNull
        );
    }

    #[test]
    fn rejects_malformed_foreign_key_arguments() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(= 5)]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "foreign_key"
        ));
    }

    #[test]
    fn rejects_a_repeated_foreign_key_attribute() {
        assert_eq!(
            indexing_rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key]\n    #[foreign_key]\n    author: Key<Author>,\n}\n",
            )),
            "attribute 'foreign_key' is repeated on 'crate::Article::author' but a single occurrence was expected"
        );
    }

    #[test]
    fn rejects_a_key_to_an_unresolvable_type() {
        assert!(matches!(
                value_rejection(KEY, "Key<DoesNotExist>"),
                ModelCodegenError::UnresolvableFieldType { ref rust_type, .. } if rust_type == "DoesNotExist"
        ));
    }

    #[test]
    fn rejects_a_key_to_a_struct_that_is_not_a_model() {
        assert!(matches!(
            rejection(&format!(
                "{KEY}struct Plain;\n\n#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    other: Key<Plain>,\n}}\n"
            )),
            ModelCodegenError::KeyTargetNotAModel { ref target, .. } if target == "crate::Plain"
        ));
    }

    #[test]
    fn rejects_a_key_to_a_model_of_another_crate() {
        assert!(matches!(
            value_rejection(KEY, "Key<other::Account>"),
            ModelCodegenError::KeyTargetOutsideCrate { ref target, .. } if target == "other::Account"
        ));
    }

    #[test]
    fn rejects_a_key_with_several_type_arguments() {
        assert!(matches!(
                value_rejection(KEY, "Key<String, String>"),
                ModelCodegenError::WrapperRequiresOneArgument { ref wrapper, .. } if wrapper == "margaret::framework::active_record::key::Key"
        ));
    }

    #[test]
    fn rejects_a_key_to_a_model_without_a_primary_key() {
        assert!(matches!(
                rejection(&format!(
                    "{KEY}#[model(table = \"authors\")]\nstruct Author {{\n    #[column]\n    name: String,\n}}\n\n#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    author: Key<Author>,\n}}\n"
                )),
                ModelCodegenError::ModelRequiresPrimaryKey { ref model, .. } if model == "crate::Author"
        ));
    }

    #[test]
    fn rejects_a_key_that_collides_with_a_scalar_column() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"author_id\")]\n    writer: String,\n    #[column]\n    #[index]\n    author: Key<Author>,\n}\n"
                )),
                ModelCodegenError::DuplicateColumnName { ref column, .. } if column == "author_id"
        ));
    }

    #[test]
    fn rejects_a_key_whose_derived_column_name_is_too_long() {
        let field = "a".repeat(62);

        assert!(matches!(
                rejection(&with_author(&format!(
                    "#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    {field}: Key<Author>,\n}}\n"
                ))),
                ModelCodegenError::ForeignKeyColumnNameTooLong { ref model, .. } if model == "crate::Article"
        ));
    }

    #[test]
    fn rejects_a_nested_key_column_name_that_is_too_long() {
        let field = "a".repeat(62);

        assert!(matches!(
            rejection(&format!(
                "{AUTHOR_MODEL}\n#[model(table = \"notes\")]\nstruct Note {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    source: Key<Translation>,\n}}\n\n#[model(table = \"translations\")]\n#[primary_key(fields = [{field}, locale])]\nstruct Translation {{\n    #[column]\n    {field}: Key<Author>,\n    #[column]\n    locale: String,\n}}\n"
            )),
            ModelCodegenError::ForeignKeyColumnNameTooLong { ref model, .. } if model == "crate::Translation"
        ));
    }

    #[test]
    fn rejects_a_nested_key_to_a_struct_that_is_not_a_model() {
        assert!(matches!(
            rejection(&format!(
                "{KEY}struct Plain;\n\n#[model(table = \"notes\")]\nstruct Note {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    source: Key<Translation>,\n}}\n\n#[model(table = \"translations\")]\n#[primary_key(fields = [plain, locale])]\nstruct Translation {{\n    #[column]\n    plain: Key<Plain>,\n    #[column]\n    locale: String,\n}}\n"
            )),
            ModelCodegenError::KeyTargetNotAModel { ref model, .. } if model == "crate::Translation"
        ));
    }

    #[test]
    fn rejects_a_key_that_no_index_leads_with() {
        assert!(matches!(
            rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    author: Key<Author>,\n}\n"
            )),
            ModelCodegenError::UnindexedForeignKey { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_primary_keys_that_reference_each_other() {
        assert!(matches!(
                rejection(&format!(
                    "{KEY}#[model(table = \"alpha\")]\nstruct Alpha {{\n    #[column(primary_key)]\n    beta: Key<Beta>,\n}}\n\n#[model(table = \"beta\")]\nstruct Beta {{\n    #[column(primary_key)]\n    alpha: Key<Alpha>,\n}}\n"
                )),
                ModelCodegenError::PrimaryKeyReferenceCycle { ref model, .. } if model == "crate::Beta"
        ));
    }

    #[test]
    fn orders_referenced_tables_before_referencing_tables() {
        assert_eq!(
            tables(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    author: Key<Author>,\n}\n"
            )),
            ["authors", "articles"]
        );
    }

    #[test]
    fn orders_a_composite_key_target_before_the_model_that_references_it() {
        assert_eq!(
            tables(&with_fragment_metadata(
                "#[model(table = \"fragment\")]\nstruct FragmentAssociation {\n    #[column(primary_key)]\n    metadata: Key<FragmentMetadata>,\n}\n"
            )),
            ["fragment_metadata", "fragment"]
        );
    }

    #[test]
    fn orders_a_self_referencing_key_without_a_cycle() {
        assert_eq!(
            tables(&format!(
                "{KEY}#[model(table = \"nodes\")]\nstruct Node {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    parent: Option<Key<Node>>,\n}}\n"
            )),
            ["nodes"]
        );
    }

    #[test]
    fn rejects_a_foreign_key_cycle() {
        assert!(matches!(
                rejection(&format!(
                    "{KEY}#[model(table = \"alpha\")]\nstruct Alpha {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    beta: Key<Beta>,\n}}\n\n#[model(table = \"beta\")]\nstruct Beta {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    alpha: Key<Alpha>,\n}}\n"
                )),
                ModelCodegenError::ForeignKeyCycle { ref path, .. } if path == "alpha -> beta -> alpha"
        ));
    }

    #[test]
    fn resolves_a_composite_primary_key_in_the_declared_order() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(fields = [branch, repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";
        let locks = model(source, "locks");

        assert_eq!(
            locks.indexes.primary_key.field_names(),
            vec!["branch".to_string(), "repository".to_string()]
        );
        assert_eq!(
            index_columns(source, "locks", IndexKind::PrimaryKey),
            vec![vec!["branch".to_string(), "repository".to_string()]]
        );
    }

    #[test]
    fn names_the_primary_key_index_after_the_table() {
        assert_eq!(
            model(&value_model("", "String"), "values")
                .indexes
                .primary_key
                .name,
            "values_pkey"
        );
    }

    #[test]
    fn rejects_two_fields_flagged_as_the_primary_key() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\nstruct Lock {\n    #[column(primary_key)]\n    repository: String,\n    #[column(primary_key)]\n    branch: String,\n}\n"
                ),
                ModelCodegenError::CompositePrimaryKeyRequiresModelDeclaration { ref model, .. } if model == "crate::Lock"
        ));
    }

    #[test]
    fn rejects_a_primary_key_declared_on_both_a_column_and_the_model() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\nstruct Lock {\n    #[column(primary_key)]\n    repository: String,\n    #[column]\n    branch: String,\n}\n"
                ),
                ModelCodegenError::ConflictingPrimaryKeyDeclarations { ref model, .. } if model == "crate::Lock"
        ));
    }

    #[test]
    fn rejects_more_than_one_model_primary_key() {
        assert_eq!(
            indexing_rejection(
                "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\n#[primary_key(fields = [branch, repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n"
            ),
            "attribute 'primary_key' is repeated on 'crate::Lock' but a single occurrence was expected"
        );
    }

    #[test]
    fn rejects_a_model_primary_key_field_the_model_does_not_declare() {
        assert!(matches!(
            rejection(
                "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, missing])]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
            ),
            ModelCodegenError::ModelDeclarationFieldNotDeclared { ref field, .. } if field == "missing"
        ));
    }

    #[test]
    fn rejects_an_optional_primary_key_field() {
        assert!(matches!(
            rejection(
                "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: Option<String>,\n}\n"
            ),
            ModelCodegenError::NullablePrimaryKeyField { ref field, .. } if field == "branch"
        ));
    }

    #[test]
    fn rejects_a_model_primary_key_over_a_single_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationRequiresSeveralFields { ref declaration, .. } if declaration == "primary_key"
        ));
    }

    #[test]
    fn rejects_a_model_primary_key_without_fields() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationRequiresFields { ref declaration, .. } if declaration == "primary_key"
        ));
    }

    #[test]
    fn rejects_a_model_primary_key_that_repeats_a_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, repository])]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationRepeatsField { ref field, .. } if field == "repository"
        ));
    }

    #[test]
    fn rejects_a_model_primary_key_naming_a_path() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [self::repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationFieldIsNotAnIdentifier { ref field, .. } if field == "self::repository"
        ));
    }

    #[test]
    fn rejects_malformed_model_primary_key_arguments() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(= 5)]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "primary_key"
        ));
    }

    #[test]
    fn rejects_a_primary_key_attribute_on_a_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\nstruct Lock {\n    #[column]\n    #[primary_key(fields = [repository, branch])]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::PrimaryKeyIsNotAFieldAttribute { ref field, .. } if field == "repository"
        ));
    }

    #[test]
    fn rejects_a_unique_attribute_on_a_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\nstruct Lock {\n    #[column(primary_key)]\n    #[unique(fields = [repository, branch])]\n    repository: String,\n}\n"
                ),
                ModelCodegenError::UniqueIsNotAFieldAttribute { ref field, .. } if field == "repository"
        ));
    }

    #[test]
    fn resolves_a_unique_column() {
        let source = "#[model(table = \"members\")]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n}\n";
        let members = model(source, "members");

        assert_eq!(members.indexes.secondary[0].kind, IndexKind::Unique);
        assert_eq!(members.indexes.secondary[0].name, "members_email_key");
        assert_eq!(
            members.indexes.secondary[0].field_names(),
            vec!["email".to_string()]
        );
    }

    #[test]
    fn resolves_a_composite_unique_constraint_from_a_model_attribute() {
        assert_eq!(
            index_columns(
                "#[model(table = \"members\")]\n#[unique(fields = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n",
                "members",
                IndexKind::Unique
            ),
            vec![vec!["email".to_string(), "label".to_string()]]
        );
    }

    #[test]
    fn rejects_an_optional_unique_column() {
        assert!(matches!(
            rejection(
                "#[model(table = \"members\")]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: Option<String>,\n}\n"
            ),
            ModelCodegenError::NullableUniqueField { ref field, .. } if field == "email"
        ));
    }

    #[test]
    fn rejects_an_optional_field_of_a_composite_unique_constraint() {
        assert!(matches!(
            rejection(
                "#[model(table = \"members\")]\n#[unique(fields = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: Option<String>,\n}\n"
            ),
            ModelCodegenError::NullableUniqueField { ref field, .. } if field == "label"
        ));
    }

    #[test]
    fn rejects_a_model_unique_field_the_model_does_not_declare() {
        assert!(matches!(
            rejection(
                "#[model(table = \"members\")]\n#[unique(fields = [email, missing])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n}\n"
            ),
            ModelCodegenError::ModelDeclarationFieldNotDeclared { ref field, .. } if field == "missing"
        ));
    }

    #[test]
    fn rejects_two_model_unique_constraints_over_the_same_fields() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"members\")]\n#[unique(fields = [email, label])]\n#[unique(fields = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n"
                ),
                ModelCodegenError::DuplicateModelUniqueConstraint { ref fields, .. } if fields == "email, label"
        ));
    }

    #[test]
    fn rejects_a_model_unique_over_a_single_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"members\")]\n#[unique(fields = [email])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationRequiresSeveralFields { ref declaration, .. } if declaration == "unique"
        ));
    }

    #[test]
    fn rejects_malformed_model_unique_arguments() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"members\")]\n#[unique(= 5)]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n"
                ),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "unique"
        ));
    }

    #[test]
    fn derives_the_name_of_a_field_index() {
        let source = "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    title: String,\n}\n";

        assert_eq!(
            model(source, "articles").indexes.secondary[0].name,
            "articles_title_index"
        );
    }

    #[test]
    fn keeps_the_explicit_name_of_a_field_index() {
        let source = "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"articles_by_title\")]\n    title: String,\n}\n";

        assert_eq!(
            model(source, "articles").indexes.secondary[0].name,
            "articles_by_title"
        );
    }

    #[test]
    fn resolves_a_composite_index_from_a_model_attribute() {
        assert_eq!(
            index_columns(
                "#[model(table = \"events\")]\n#[index(name = \"events_kind_label\", fields = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n",
                "events",
                IndexKind::Plain
            ),
            vec![vec!["kind".to_string(), "label".to_string()]]
        );
    }

    #[test]
    fn orders_the_plain_indexes_by_name() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_a\", fields = [label, kind])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"events_b\")]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert_eq!(
            model(source, "events")
                .indexes
                .of_kind(IndexKind::Plain)
                .map(|index| index.name.as_str())
                .collect::<Vec<&str>>(),
            ["events_a", "events_b"]
        );
    }

    #[test]
    fn rejects_a_model_index_without_a_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\n#[index(fields = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n"
                ),
                ModelCodegenError::ModelIndexRequiresName { ref model, .. } if model == "crate::Event"
        ));
    }

    #[test]
    fn rejects_a_model_index_field_the_model_does_not_declare() {
        assert!(matches!(
            rejection(
                "#[model(table = \"events\")]\n#[index(name = \"events_kind_label\", fields = [kind, missing])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n}\n"
            ),
            ModelCodegenError::ModelDeclarationFieldNotDeclared { ref field, .. } if field == "missing"
        ));
    }

    #[test]
    fn rejects_a_model_index_over_a_single_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\n#[index(name = \"events_kind\", fields = [kind])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n}\n"
                ),
                ModelCodegenError::ModelDeclarationRequiresSeveralFields { ref declaration, .. } if declaration == "index"
        ));
    }

    #[test]
    fn rejects_a_model_index_name_that_is_not_a_string() {
        assert!(matches!(
            rejection(
                "#[model(table = \"events\")]\n#[index(name = 5, fields = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "name"
        ));
    }

    #[test]
    fn rejects_a_model_index_name_that_is_not_snake_case() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\n#[index(name = \"Events\", fields = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n"
                ),
                ModelCodegenError::InvalidIndexName { ref index, .. } if index == "Events"
        ));
    }

    #[test]
    fn rejects_malformed_model_index_arguments() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\n#[index(= 5)]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n"
                ),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "index"
        ));
    }

    #[test]
    fn rejects_malformed_field_index_arguments() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index(= 5)]\n    value: String,\n}\n"
                ),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "index"
        ));
    }

    #[test]
    fn rejects_a_non_string_field_index_name() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index(name = 5)]\n    value: String,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "name"
        ));
    }

    #[test]
    fn rejects_a_field_index_that_names_fields() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(fields = [kind])]\n    kind: String,\n}\n"
                ),
                ModelCodegenError::FieldIndexCannotDeclareFields { ref field, .. } if field == "kind"
        ));
    }

    #[test]
    fn rejects_an_explicit_index_name_that_is_too_long() {
        let name = "a".repeat(64);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index(name = \"{name}\")]\n    value: String,\n}}\n"
                )),
                ModelCodegenError::ExplicitIndexNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_derived_index_name_that_is_too_long() {
        let field = "a".repeat(56);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index]\n    {field}: String,\n}}\n"
                )),
                ModelCodegenError::IndexNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_derived_primary_key_index_name_that_is_too_long() {
        let table = "a".repeat(60);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"{table}\")]\nstruct S {{\n    #[column(primary_key)]\n    id: i64,\n}}\n"
                )),
                ModelCodegenError::PrimaryKeyIndexNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_a_derived_unique_index_name_that_is_too_long() {
        let field = "a".repeat(58);

        assert!(matches!(
                rejection(&format!(
                    "#[model(table = \"t\")]\nstruct S {{\n    #[column(primary_key)]\n    id: i64,\n    #[column(unique)]\n    {field}: String,\n}}\n"
                )),
                ModelCodegenError::UniqueIndexNameTooLong { ref model, .. } if model == "crate::S"
        ));
    }

    #[test]
    fn rejects_an_index_on_the_primary_key_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    #[index]\n    id: i64,\n}\n"
                ),
                ModelCodegenError::RedundantIndexOnPrimaryKeyColumns { ref columns, .. } if columns == "id"
        ));
    }

    #[test]
    fn rejects_an_index_on_the_leading_field_of_a_composite_primary_key() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\nstruct Lock {\n    #[column]\n    #[index]\n    repository: String,\n    #[column]\n    branch: String,\n}\n"
                ),
                ModelCodegenError::RedundantIndexOnPrimaryKeyColumns { ref columns, .. } if columns == "repository"
        ));
    }

    #[test]
    fn accepts_an_index_on_a_trailing_field_of_a_composite_primary_key() {
        let source = "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    #[index]\n    branch: String,\n}\n";

        assert_eq!(
            index_columns(source, "locks", IndexKind::Plain),
            vec![vec!["branch".to_string()]]
        );
    }

    #[test]
    fn rejects_a_model_index_that_duplicates_the_primary_key() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\n#[index(name = \"locks_repository_branch\", fields = [repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n"
                ),
                ModelCodegenError::RedundantIndexOnPrimaryKeyColumns { ref columns, .. } if columns == "repository, branch"
        ));
    }

    #[test]
    fn rejects_an_index_on_a_unique_field() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    #[index(name = \"solo\")]\n    email: String,\n}\n"
                ),
                ModelCodegenError::RedundantIndexOnUniqueColumns { ref columns, .. } if columns == "email"
        ));
    }

    #[test]
    fn rejects_a_model_index_that_duplicates_a_unique_constraint() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"members\")]\n#[unique(fields = [email, label])]\n#[index(name = \"members_email_label\", fields = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    email: String,\n    #[column]\n    label: String,\n}\n"
                ),
                ModelCodegenError::RedundantIndexOnUniqueColumns { ref columns, .. } if columns == "email, label"
        ));
    }

    #[test]
    fn rejects_an_index_that_extends_the_primary_key() {
        assert!(matches!(
            rejection(
                "#[model(table = \"events\")]\n#[index(name = \"events_id_kind\", fields = [id, kind])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n}\n"
            ),
            ModelCodegenError::IndexExtendsUniqueKey { ref unique, .. } if unique == "id"
        ));
    }

    #[test]
    fn rejects_a_unique_constraint_that_extends_a_unique_field() {
        assert!(matches!(
            rejection(
                "#[model(table = \"members\")]\n#[unique(fields = [email, label])]\nstruct Member {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    email: String,\n    #[column]\n    label: String,\n}\n"
            ),
            ModelCodegenError::IndexExtendsUniqueKey { ref unique, .. } if unique == "email"
        ));
    }

    #[test]
    fn accepts_an_ordering_index_that_ends_with_the_primary_key() {
        let source = "#[model(table = \"messages\")]\n#[index(name = \"messages_posted\", fields = [posted_at, id])]\nstruct Message {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    posted_at: i64,\n}\n";

        assert_eq!(
            index_columns(source, "messages", IndexKind::Plain),
            vec![vec!["posted_at".to_string(), "id".to_string()]]
        );
    }

    #[test]
    fn rejects_two_fields_that_share_an_index_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"events_pair\")]\n    kind: String,\n    #[column]\n    #[index(name = \"events_pair\")]\n    label: String,\n}\n"
                ),
                ModelCodegenError::DuplicateIndexDeclaration { ref index, .. } if index == "events_pair"
        ));
    }

    #[test]
    fn rejects_an_explicit_index_name_that_collides_with_a_derived_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    c: String,\n    #[column]\n    #[index(name = \"t_c_index\")]\n    d: String,\n}\n"
                ),
                ModelCodegenError::DuplicateIndexDeclaration { ref index, .. } if index == "t_c_index"
        ));
    }

    #[test]
    fn rejects_a_duplicate_index_name_across_tables() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"a\")]\nstruct A {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"shared\")]\n    x: String,\n}\n\n#[model(table = \"b\")]\nstruct B {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"shared\")]\n    y: String,\n}\n"
                ),
                ModelCodegenError::DuplicateIndexName { ref name, .. } if name == "shared"
        ));
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_table_name() {
        assert!(matches!(
                rejection(&with_author(
                    "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"authors\")]\n    title: String,\n}\n"
                )),
                ModelCodegenError::IndexNameCollidesWithTableName { ref name, .. } if name == "authors"
        ));
    }

    #[test]
    fn rejects_a_derived_index_name_that_collides_with_a_table_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"articles_title_index\")]\nstruct Reserved {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    title: String,\n}\n"
                ),
                ModelCodegenError::IndexNameCollidesWithTableName { ref name, .. } if name == "articles_title_index"
        ));
    }

    #[test]
    fn rejects_a_constraint_index_name_that_collides_with_a_table_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"articles_pkey\")]\nstruct Reserved {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n"
                ),
                ModelCodegenError::ConstraintIndexCollidesWithTableName { ref name, .. } if name == "articles_pkey"
        ));
    }

    #[test]
    fn rejects_two_tables_that_derive_the_same_constraint_index_name() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"a\")]\nstruct First {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    b_c: String,\n}\n\n#[model(table = \"a_b\")]\nstruct Second {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(unique)]\n    c: String,\n}\n"
                ),
                ModelCodegenError::DuplicateConstraintIndexName { ref name, .. } if name == "a_b_c_key"
        ));
    }

    #[test]
    fn rejects_an_index_name_that_collides_with_a_constraint_index() {
        assert!(matches!(
                rejection(
                    "#[model(table = \"author\")]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"book\")]\nstruct Book {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(name = \"author_pkey\")]\n    title: String,\n}\n"
                ),
                ModelCodegenError::IndexNameCollidesWithConstraintIndex { ref name, .. } if name == "author_pkey"
        ));
    }

    const ARTICLE_MODEL: &str = "\
use margaret::framework::active_record::key::Key;

#[model(table = \"article_translations\")]
#[primary_key(fields = [article, locale])]
struct ArticleTranslation {
    #[column]
    article: Key<Article>,
    #[column]
    locale: String,
}

#[model(table = \"article_covers\")]
struct ArticleCover {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column(unique)]
    article: Key<Article>,
}

#[model(table = \"article_comments\")]
#[index(name = \"article_comments_posted\", fields = [article, posted_at, id])]
struct ArticleComment {
    #[column(primary_key)]
    id: uuid::Uuid,
    #[column]
    article: Key<Article>,
    #[column]
    posted_at: i64,
}
";

    fn article(relations: &str) -> String {
        format!(
            "{ARTICLE_MODEL}\n{relations}\n#[model(table = \"articles\")]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    title: String,\n}}\n"
        )
    }

    fn article_rejection(relations: &str) -> ModelCodegenError {
        rejection(&article(relations))
    }

    #[test]
    fn links_a_has_many_relation_through_the_primary_key_of_the_related_model() {
        let relation = model(
            &article(
                "#[has_many(name = \"translations\", model = ArticleTranslation, key = article)]",
            ),
            "articles",
        )
        .relations
        .remove(0);

        assert_eq!(relation.covering_index.name, "article_translations_pkey");
        assert_eq!(relation.key.name, "article");
        assert_eq!(relation.kind, RelationKind::HasMany);
        assert_eq!(relation.name, "translations");
        assert_eq!(relation.related, path(&["crate", "ArticleTranslation"]));
    }

    #[test]
    fn links_a_has_many_relation_through_an_ordering_index() {
        assert_eq!(
            model(
                &article("#[has_many(name = \"comments\", model = ArticleComment, key = article)]"),
                "articles"
            )
            .relations[0]
                .covering_index
                .name,
            "article_comments_posted"
        );
    }

    #[test]
    fn links_a_has_one_relation_through_a_unique_key() {
        let relation = model(
            &article("#[has_one(name = \"cover\", model = ArticleCover, key = article)]"),
            "articles",
        )
        .relations
        .remove(0);

        assert_eq!(
            relation.covering_index.name,
            "article_covers_article_id_key"
        );
        assert_eq!(relation.kind, RelationKind::HasOne);
        assert_eq!(relation.related, path(&["crate", "ArticleCover"]));
    }

    #[test]
    fn rejects_a_has_one_relation_over_a_key_that_is_not_unique() {
        assert!(matches!(
                article_rejection(
                    "#[has_one(name = \"comment\", model = ArticleComment, key = article)]"
                ),
                ModelCodegenError::HasOneRequiresUniqueKey { ref relation, .. } if relation == "comment"
        ));
    }

    #[test]
    fn rejects_a_has_many_relation_over_a_unique_key() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"covers\", model = ArticleCover, key = article)]"
                ),
                ModelCodegenError::HasManyOverUniqueKey { ref relation, .. } if relation == "covers"
        ));
    }

    #[test]
    fn rejects_a_has_many_relation_without_an_ordering_index() {
        let source = format!(
            "{KEY}#[model(table = \"articles\")]\n#[has_many(name = \"tags\", model = Tag, key = article)]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n\n#[model(table = \"tags\")]\nstruct Tag {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    article: Key<Article>,\n}}\n"
        );

        assert!(matches!(
                rejection(&source),
                ModelCodegenError::HasManyRequiresOrderedIndex { ref relation, .. } if relation == "tags"
        ));
    }

    #[test]
    fn rejects_a_has_many_relation_with_several_ordering_indexes() {
        let source = format!(
            "{KEY}#[model(table = \"articles\")]\n#[has_many(name = \"tags\", model = Tag, key = article)]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n\n#[model(table = \"tags\")]\n#[index(name = \"tags_by_label\", fields = [article, label, id])]\n#[index(name = \"tags_by_id\", fields = [article, id])]\nstruct Tag {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    article: Key<Article>,\n    #[column]\n    label: String,\n}}\n"
        );

        assert!(matches!(
                rejection(&source),
                ModelCodegenError::AmbiguousHasManyIndex { ref relation, .. } if relation == "tags"
        ));
    }

    #[test]
    fn rejects_a_relation_without_a_name() {
        assert!(matches!(
            article_rejection("#[has_many(model = ArticleTranslation, key = article)]"),
            ModelCodegenError::RelationRequiresName { ref declaration, .. } if declaration == "has_many"
        ));
    }

    #[test]
    fn rejects_a_relation_name_that_is_not_snake_case() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"Translations\", model = ArticleTranslation, key = article)]"
                ),
                ModelCodegenError::InvalidRelationName { ref relation, .. } if relation == "Translations"
        ));
    }

    #[test]
    fn rejects_a_relation_without_a_model() {
        assert!(matches!(
                article_rejection("#[has_many(name = \"translations\", key = article)]"),
                ModelCodegenError::RelationRequiresModel { ref relation, .. } if relation == "translations"
        ));
    }

    #[test]
    fn rejects_a_relation_to_an_unresolvable_item() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"translations\", model = Missing, key = article)]"
                ),
                ModelCodegenError::RelatedItemNotAModel { ref related, .. } if related == "Missing"
        ));
    }

    #[test]
    fn rejects_a_relation_to_an_item_that_is_not_a_model() {
        assert!(matches!(
            article_rejection(
                "struct Plain;\n\n#[has_many(name = \"translations\", model = Plain, key = article)]"
            ),
            ModelCodegenError::RelatedItemNotAModel { ref related, .. } if related == "crate::Plain"
        ));
    }

    #[test]
    fn rejects_a_relation_without_a_key() {
        assert!(matches!(
                article_rejection("#[has_many(name = \"translations\", model = ArticleTranslation)]"),
                ModelCodegenError::RelationRequiresKey { ref relation, .. } if relation == "translations"
        ));
    }

    #[test]
    fn rejects_a_relation_key_that_is_a_path() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"translations\", model = ArticleTranslation, key = self::article)]"
                ),
                ModelCodegenError::RelationKeyIsNotAnIdentifier { ref key, .. } if key == "self::article"
        ));
    }

    #[test]
    fn rejects_a_relation_key_the_related_model_does_not_declare() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"translations\", model = ArticleTranslation, key = missing)]"
                ),
                ModelCodegenError::RelationKeyFieldNotDeclared { ref key, .. } if key == "missing"
        ));
    }

    #[test]
    fn rejects_a_relation_key_that_is_not_a_key() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"translations\", model = ArticleTranslation, key = locale)]"
                ),
                ModelCodegenError::RelationKeyFieldNotAKey { ref key, .. } if key == "locale"
        ));
    }

    #[test]
    fn rejects_a_relation_key_that_references_another_model() {
        assert!(matches!(
            rejection(&with_author(
                "#[model(table = \"profiles\")]\nstruct Profile {\n    #[column(primary_key)]\n    author: Key<Author>,\n}\n\n#[model(table = \"articles\")]\n#[has_one(name = \"profile\", model = Profile, key = author)]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n"
            )),
            ModelCodegenError::RelationKeyTargetsAnotherModel { ref target, .. } if target == "crate::Author"
        ));
    }

    #[test]
    fn rejects_two_relations_with_the_same_name() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"items\", model = ArticleTranslation, key = article)]\n#[has_many(name = \"items\", model = ArticleComment, key = article)]"
                ),
                ModelCodegenError::DuplicateRelationName { ref relation, .. } if relation == "items"
        ));
    }

    #[test]
    fn rejects_a_relation_named_after_a_key_field() {
        let source = format!(
            "{KEY}#[model(table = \"nodes\")]\n#[has_many(name = \"parent\", model = Node, key = parent)]\n#[index(name = \"nodes_children\", fields = [parent, id])]\nstruct Node {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    parent: Option<Key<Node>>,\n}}\n"
        );

        assert!(matches!(
            rejection(&source),
            ModelCodegenError::DuplicateRelationName { ref relation, .. } if relation == "parent"
        ));
    }

    #[test]
    fn rejects_two_relations_through_the_same_key() {
        assert!(matches!(
                article_rejection(
                    "#[has_many(name = \"translations\", model = ArticleTranslation, key = article)]\n#[has_many(name = \"localized\", model = ArticleTranslation, key = article)]"
                ),
                ModelCodegenError::DuplicateInverseRelation { ref key, .. } if key == "article"
        ));
    }

    #[test]
    fn rejects_malformed_relation_arguments() {
        assert!(matches!(
                article_rejection("#[has_many(= 5)]"),
                ModelCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                }),
            } if attribute_path == "has_many"
        ));
    }

    #[test]
    fn places_the_models_in_the_requested_namespace() {
        let indexed = IndexedSource::new(&value_model("", "String"));

        assert_eq!(
            models(&indexed.index, TableNamespace::Framework).expect("the models resolve")[0]
                .namespace,
            TableNamespace::Framework
        );
    }

    #[test]
    fn records_the_canonical_path_of_the_model() {
        assert_eq!(
            model(
                "mod blog {\n    #[model(table = \"posts\")]\n    pub struct Post {\n        #[column(primary_key)]\n        pub id: i64,\n    }\n}\n",
                "posts"
            )
            .path,
            path(&["crate", "blog", "Post"])
        );
    }

    #[test]
    fn rejects_a_datetime_without_its_timezone() {
        assert!(matches!(
            value_rejection("use chrono::DateTime;", "DateTime"),
            ModelCodegenError::WrapperRequiresOneArgument { ref wrapper, .. } if wrapper == "chrono::DateTime"
        ));
    }

    #[test]
    fn rejects_a_byte_length_on_a_text_column() {
        assert!(matches!(
            rejection(
                "#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key, byte_length = 32)]\n    name: String,\n}\n"
            ),
            ModelCodegenError::ByteLengthOnNonByteaColumn { ref column, .. } if column == "name"
        ));
    }

    #[test]
    fn rejects_both_a_byte_length_and_a_minimum() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key, byte_length = 4, minimum = 0)]\n    value: i64,\n}\n"
            ),
            ModelCodegenError::ConflictingColumnChecks { ref field, .. } if field == "value"
        ));
    }

    #[test]
    fn rejects_a_scale_without_a_precision() {
        assert!(matches!(
            rejection(
                "#[model(table = \"t\")]\nstruct S {\n    #[column(primary_key)]\n    id: i64,\n    #[column(scale = 2)]\n    value: rust_decimal::Decimal,\n}\n"
            ),
            ModelCodegenError::NumericPrecisionMissing { ref field, .. } if field == "value"
        ));
    }

    #[test]
    fn rejects_field_index_fields_that_are_not_an_array() {
        assert!(matches!(
            rejection(
                "#[model(table = \"events\")]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index(fields = \"kind\")]\n    kind: String,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "fields"
        ));
    }

    #[test]
    fn rejects_model_declaration_fields_that_are_not_an_array() {
        assert!(matches!(
            rejection(
                "#[model(table = \"locks\")]\n#[primary_key(fields = \"repository\")]\nstruct Lock {\n    #[column]\n    repository: String,\n}\n"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "fields"
        ));
    }

    #[test]
    fn rejects_an_on_delete_action_that_is_not_a_path() {
        assert!(matches!(
            rejection(&with_author(
                "#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[foreign_key(on_delete = \"Cascade\")]\n    #[index]\n    author: Key<Author>,\n}\n"
            )),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "on_delete"
        ));
    }

    #[test]
    fn rejects_a_relation_name_that_is_not_a_string() {
        assert!(matches!(
            article_rejection("#[has_many(name = 5, model = ArticleTranslation, key = article)]"),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "name"
        ));
    }

    #[test]
    fn rejects_a_relation_model_that_is_not_a_path() {
        assert!(matches!(
            article_rejection(
                "#[has_many(name = \"translations\", model = \"ArticleTranslation\", key = article)]"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "model"
        ));
    }

    #[test]
    fn rejects_a_relation_key_that_is_not_a_path() {
        assert!(matches!(
            article_rejection(
                "#[has_many(name = \"translations\", model = ArticleTranslation, key = \"article\")]"
            ),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "key"
        ));
    }

    #[test]
    fn names_the_generated_module_after_the_canonical_path() {
        assert_eq!(
            model(
                "mod blog {\n    #[model(table = \"posts\")]\n    pub struct Post {\n        #[column(primary_key)]\n        pub id: i64,\n    }\n}\n",
                "posts"
            )
            .module,
            "blog_post"
        );
    }
}
