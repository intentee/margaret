use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::format_path::format_path;
use margaret_model::on_delete::OnDelete;

use crate::model_codegen_error::ModelCodegenError;
use crate::on_delete_argument::OnDeleteArgument;

pub(crate) struct ForeignKeyArguments {
    pub(crate) on_delete: OnDelete,
}

impl ForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let on_delete = match OnDeleteArgument::of(reader.take_path("on_delete")?) {
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
    use margaret_model::on_delete::OnDelete;

    use crate::foreign_key_arguments::ForeignKeyArguments;
    use crate::model_codegen_error::ModelCodegenError;

    fn parse(attribute: &Attribute) -> Result<ForeignKeyArguments, ModelCodegenError> {
        let arguments = AttributeArgs::from_attribute(attribute).expect("the arguments parse");

        ForeignKeyArguments::parse(&arguments, "crate::Model", "author")
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
    fn maps_cascade() {
        assert_eq!(
            on_delete(&parse_quote!(#[foreign_key(on_delete = cascade)])),
            OnDelete::Cascade
        );
    }

    #[test]
    fn maps_restrict() {
        assert_eq!(
            on_delete(&parse_quote!(#[foreign_key(on_delete = restrict)])),
            OnDelete::Restrict
        );
    }

    #[test]
    fn maps_set_null() {
        assert_eq!(
            on_delete(&parse_quote!(#[foreign_key(on_delete = set_null)])),
            OnDelete::SetNull
        );
    }

    #[test]
    fn maps_set_default() {
        assert_eq!(
            on_delete(&parse_quote!(#[foreign_key(on_delete = set_default)])),
            OnDelete::SetDefault
        );
    }

    #[test]
    fn rejects_no_action_as_an_explicit_value() {
        let message = parse(&parse_quote!(#[foreign_key(on_delete = no_action)]))
            .err()
            .unwrap()
            .to_string();

        assert!(message.contains("unknown ON DELETE action"));
    }

    #[test]
    fn rejects_a_string_valued_action() {
        assert!(parse(&parse_quote!(#[foreign_key(on_delete = "cascade")])).is_err());
    }
}
