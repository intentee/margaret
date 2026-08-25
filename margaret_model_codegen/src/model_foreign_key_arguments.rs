use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_model::on_delete::OnDelete;

use crate::model_codegen_error::ModelCodegenError;
use crate::on_delete_argument::OnDeleteArgument;

#[derive(Debug)]
pub(crate) struct ModelForeignKeyArguments {
    pub(crate) columns: Vec<String>,
    pub(crate) on_delete: OnDelete,
    pub(crate) references: Path,
}

impl ModelForeignKeyArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, model: &str) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let declared_columns = reader.take_path_array("columns")?.unwrap_or_default();

            if declared_columns.is_empty() {
                return Err(ModelCodegenError::ModelForeignKeyRequiresColumns {
                    model: model.to_string(),
                });
            }

            let mut columns: Vec<String> = Vec::with_capacity(declared_columns.len());

            for declared_column in &declared_columns {
                let Some(identifier) = declared_column.get_ident() else {
                    return Err(ModelCodegenError::ModelForeignKeyColumnIsNotAnIdentifier {
                        column: format_path(declared_column),
                        model: model.to_string(),
                    });
                };

                columns.push(identifier.to_string());
            }

            let references = reader.take_path("references")?.ok_or_else(|| {
                ModelCodegenError::ModelForeignKeyRequiresReferences {
                    model: model.to_string(),
                }
            })?;

            let on_delete = match OnDeleteArgument::of(reader.take_path("on_delete")?) {
                OnDeleteArgument::Known(on_delete) => on_delete,
                OnDeleteArgument::Unknown(path) => {
                    return Err(ModelCodegenError::UnknownModelForeignKeyOnDeleteAction {
                        action: format_path(&path),
                        model: model.to_string(),
                    });
                }
            };

            Ok(Self {
                columns,
                on_delete,
                references,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attribute_arguments::format_path::format_path;
    use margaret_model::on_delete::OnDelete;

    use crate::model_codegen_error::ModelCodegenError;
    use crate::model_foreign_key_arguments::ModelForeignKeyArguments;

    fn parse(attribute: &Attribute) -> Result<ModelForeignKeyArguments, ModelCodegenError> {
        let arguments = AttributeArgs::from_attribute(attribute).expect("the arguments parse");

        ModelForeignKeyArguments::parse(&arguments, "crate::Model")
    }

    #[test]
    fn reads_the_constrained_columns_and_the_referenced_model() {
        let ModelForeignKeyArguments {
            columns,
            on_delete,
            references,
        } = parse(&parse_quote!(
            #[foreign_key(columns = [partition, hash], references = crate::Metadata)]
        ))
        .expect("the foreign key arguments resolve");

        assert_eq!(columns, vec!["partition".to_string(), "hash".to_string()]);
        assert_eq!(on_delete, OnDelete::NoAction);
        assert_eq!(format_path(&references), "crate::Metadata");
    }

    #[test]
    fn reads_an_explicit_on_delete_action() {
        assert_eq!(
            parse(&parse_quote!(
                #[foreign_key(
                    columns = [hash],
                    references = crate::Metadata,
                    on_delete = cascade
                )]
            ))
            .expect("the foreign key arguments resolve")
            .on_delete,
            OnDelete::Cascade
        );
    }

    #[test]
    fn rejects_absent_columns() {
        assert!(matches!(
            parse(&parse_quote!(#[foreign_key(references = crate::Metadata)]))
                .expect_err("a foreign key without columns is rejected"),
            ModelCodegenError::ModelForeignKeyRequiresColumns { ref model }
                if model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_an_empty_column_list() {
        assert!(matches!(
            parse(&parse_quote!(#[foreign_key(columns = [], references = crate::Metadata)]))
                .expect_err("an empty column list is rejected"),
            ModelCodegenError::ModelForeignKeyRequiresColumns { ref model }
                if model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_a_column_that_is_not_a_plain_identifier() {
        assert!(matches!(
            parse(&parse_quote!(
                #[foreign_key(columns = [outer::inner], references = crate::Metadata)]
            ))
            .expect_err("a multi segment column path is rejected"),
            ModelCodegenError::ModelForeignKeyColumnIsNotAnIdentifier { ref column, .. }
                if column == "outer::inner"
        ));
    }

    #[test]
    fn rejects_an_absent_references_argument() {
        assert!(matches!(
            parse(&parse_quote!(#[foreign_key(columns = [hash])]))
                .expect_err("a foreign key without a target is rejected"),
            ModelCodegenError::ModelForeignKeyRequiresReferences { ref model }
                if model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_an_unknown_on_delete_action() {
        assert!(matches!(
            parse(&parse_quote!(
                #[foreign_key(
                    columns = [hash],
                    references = crate::Metadata,
                    on_delete = detonate
                )]
            ))
            .expect_err("an unknown action is rejected"),
            ModelCodegenError::UnknownModelForeignKeyOnDeleteAction { ref action, .. }
                if action == "detonate"
        ));
    }

    #[test]
    fn rejects_columns_that_are_not_an_array() {
        assert!(matches!(
            parse(&parse_quote!(
                #[foreign_key(columns = "hash", references = crate::Metadata)]
            ))
            .expect_err("a string column list is rejected"),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "columns"
        ));
    }

    #[test]
    fn rejects_a_references_argument_that_is_not_a_path() {
        assert!(matches!(
            parse(&parse_quote!(
                #[foreign_key(columns = [hash], references = "crate::Metadata")]
            ))
            .expect_err("a string target is rejected"),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "references"
        ));
    }

    #[test]
    fn rejects_an_on_delete_action_that_is_not_a_path() {
        assert!(matches!(
            parse(&parse_quote!(
                #[foreign_key(
                    columns = [hash],
                    references = crate::Metadata,
                    on_delete = "cascade"
                )]
            ))
            .expect_err("a string action is rejected"),
            ModelCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "on_delete"
        ));
    }
}
