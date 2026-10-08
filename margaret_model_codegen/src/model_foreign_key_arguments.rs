use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_model::on_delete::OnDelete;

use crate::column_list_arity::ColumnListArity;
use crate::model_codegen_error::ModelCodegenError;
use crate::model_column_list::ModelColumnList;
use crate::on_delete_argument::OnDeleteArgument;

#[derive(Debug)]
pub(crate) struct ModelForeignKeyArguments {
    pub(crate) columns: Vec<String>,
    pub(crate) on_delete: OnDelete,
    pub(crate) references: Path,
}

impl ModelForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        item: &IndexedItem,
        model: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let ModelColumnList { columns } = ModelColumnList::read(
                reader,
                FrameworkAttribute::ForeignKey,
                ColumnListArity::OneOrMore,
                model,
            )?;

            let references = reader
                .take_path(ItemNamingArgument::ForeignKeyReferences.key())?
                .ok_or_else(|| ModelCodegenError::ModelForeignKeyRequiresReferences {
                    model: model.to_string(),
                })?;

            let on_delete = match OnDeleteArgument::of(
                index,
                item,
                reader.take_path(ItemNamingArgument::OnDelete.key())?,
            ) {
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
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model::on_delete::OnDelete;

    use crate::model_codegen_error::ModelCodegenError;
    use crate::model_foreign_key_arguments::ModelForeignKeyArguments;

    fn parse(attribute: &Attribute) -> Result<ModelForeignKeyArguments, ModelCodegenError> {
        let indexed = IndexedSource::new(
            "use margaret::framework::model::on_delete::OnDelete;\n\nstruct Model;\n",
        );
        let arguments = AttributeArgs::from_attribute(attribute).expect("the arguments parse");

        ModelForeignKeyArguments::parse(
            &arguments,
            &indexed.index,
            indexed.item("Model"),
            "crate::Model",
        )
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
                    on_delete = OnDelete::Cascade
                )]
            ))
            .expect("the foreign key arguments resolve")
            .on_delete,
            OnDelete::Cascade
        );
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
                    on_delete = OnDelete::Detonate
                )]
            ))
            .expect_err("an unknown action is rejected"),
            ModelCodegenError::UnknownModelForeignKeyOnDeleteAction { ref action, .. }
                if action == "OnDelete::Detonate"
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
