use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::format_path::format_path;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ForeignKeyArguments {
    pub(crate) on_delete: TokenStream,
}

impl ForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let on_delete = match reader.take_path("on_delete")? {
                None => quote!(margaret_model::on_delete::OnDelete::NoAction),
                Some(path) if path.is_ident("cascade") => {
                    quote!(margaret_model::on_delete::OnDelete::Cascade)
                }
                Some(path) if path.is_ident("restrict") => {
                    quote!(margaret_model::on_delete::OnDelete::Restrict)
                }
                Some(path) if path.is_ident("set_null") => {
                    quote!(margaret_model::on_delete::OnDelete::SetNull)
                }
                Some(path) if path.is_ident("set_default") => {
                    quote!(margaret_model::on_delete::OnDelete::SetDefault)
                }
                Some(path) => {
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
    use quote::quote;
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attributes::attribute_args::AttributeArgs;

    use crate::foreign_key_arguments::ForeignKeyArguments;
    use crate::model_codegen_error::ModelCodegenError;

    fn parse(attribute: Attribute) -> Result<ForeignKeyArguments, ModelCodegenError> {
        let arguments = AttributeArgs::from_attribute(&attribute).expect("the arguments parse");

        ForeignKeyArguments::parse(&arguments, "crate::Model", "author")
    }

    fn on_delete(attribute: Attribute) -> String {
        parse(attribute)
            .expect("the foreign key arguments resolve")
            .on_delete
            .to_string()
    }

    #[test]
    fn defaults_to_no_action_when_absent() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key])),
            quote!(margaret_model::on_delete::OnDelete::NoAction).to_string()
        );
    }

    #[test]
    fn maps_cascade() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key(on_delete = cascade)])),
            quote!(margaret_model::on_delete::OnDelete::Cascade).to_string()
        );
    }

    #[test]
    fn maps_restrict() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key(on_delete = restrict)])),
            quote!(margaret_model::on_delete::OnDelete::Restrict).to_string()
        );
    }

    #[test]
    fn maps_set_null() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key(on_delete = set_null)])),
            quote!(margaret_model::on_delete::OnDelete::SetNull).to_string()
        );
    }

    #[test]
    fn maps_set_default() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key(on_delete = set_default)])),
            quote!(margaret_model::on_delete::OnDelete::SetDefault).to_string()
        );
    }

    #[test]
    fn rejects_no_action_as_an_explicit_value() {
        let message = parse(parse_quote!(#[foreign_key(on_delete = no_action)]))
            .err()
            .unwrap()
            .to_string();

        assert!(message.contains("unknown ON DELETE action"));
    }

    #[test]
    fn rejects_a_string_valued_action() {
        assert!(parse(parse_quote!(#[foreign_key(on_delete = "cascade")])).is_err());
    }
}
