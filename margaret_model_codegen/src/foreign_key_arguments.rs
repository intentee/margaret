use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_model::on_delete::OnDelete;

use crate::model_codegen_error::ModelCodegenError;
use crate::on_delete_argument::OnDeleteArgument;

pub(crate) struct ForeignKeyArguments {
    pub(crate) on_delete: OnDelete,
}

impl ForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        index: &AttributeIndex,
        item: &IndexedItem,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let on_delete = match OnDeleteArgument::of(
                index,
                item,
                reader.take_path(ItemNamingArgument::OnDelete.key())?,
            ) {
                OnDeleteArgument::Known(on_delete) => on_delete,
                OnDeleteArgument::Unknown(path) => {
                    return Err(ModelCodegenError::UnknownOnDeleteAction {
                        action: format_path(&path),
                        field: field.to_string(),
                        model: model.to_string(),
                    });
                }
            };

            Ok(Self { on_delete })
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model::on_delete::OnDelete;

    use crate::foreign_key_arguments::ForeignKeyArguments;
    use crate::model_codegen_error::ModelCodegenError;

    fn parse(attribute: &Attribute) -> Result<ForeignKeyArguments, ModelCodegenError> {
        let indexed = IndexedSource::new(
            "use margaret::framework::model::on_delete::OnDelete;\n\nmod local {\n    pub enum OnDelete {\n        Cascade,\n    }\n}\n\nstruct Model;\n",
        );
        let arguments = AttributeArgs::from_attribute(attribute).expect("the arguments parse");

        ForeignKeyArguments::parse(
            &arguments,
            &indexed.index,
            indexed.item("Model"),
            "crate::Model",
            "author",
        )
    }

    fn on_delete(attribute: &Attribute) -> OnDelete {
        parse(attribute)
            .expect("the foreign key arguments resolve")
            .on_delete
    }

    #[test]
    fn defaults_to_no_action_when_absent() {
        assert_eq!(on_delete(&parse_quote!(#[foreign_key])), OnDelete::NoAction);
    }

    #[test]
    fn reads_every_declarable_action() {
        assert_eq!(
            [
                on_delete(&parse_quote!(#[foreign_key(on_delete = OnDelete::Cascade)])),
                on_delete(&parse_quote!(#[foreign_key(on_delete = OnDelete::Restrict)])),
                on_delete(&parse_quote!(#[foreign_key(on_delete = OnDelete::SetDefault)])),
                on_delete(&parse_quote!(#[foreign_key(on_delete = OnDelete::SetNull)])),
            ],
            [
                OnDelete::Cascade,
                OnDelete::Restrict,
                OnDelete::SetDefault,
                OnDelete::SetNull,
            ]
        );
    }

    #[test]
    fn rejects_no_action_as_an_explicit_value() {
        assert!(matches!(
            parse(&parse_quote!(#[foreign_key(on_delete = OnDelete::NoAction)])),
            Err(ModelCodegenError::UnknownOnDeleteAction { ref action, .. })
                if action == "OnDelete::NoAction"
        ));
    }

    #[test]
    fn rejects_an_action_of_a_foreign_enum() {
        assert!(matches!(
            parse(&parse_quote!(#[foreign_key(on_delete = local::OnDelete::Cascade)])),
            Err(ModelCodegenError::UnknownOnDeleteAction { ref action, .. })
                if action == "local::OnDelete::Cascade"
        ));
    }

    #[test]
    fn rejects_a_string_valued_action() {
        assert!(parse(&parse_quote!(#[foreign_key(on_delete = "cascade")])).is_err());
    }
}
