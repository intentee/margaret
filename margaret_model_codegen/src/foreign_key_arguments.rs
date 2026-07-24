use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::format_path::format_path;
use margaret_model::on_delete::OnDelete;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ForeignKeyArguments {
    pub(crate) name: String,
    pub(crate) on_delete: OnDelete,
    pub(crate) references: Path,
}

impl ForeignKeyArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?.ok_or_else(|| {
                ModelCodegenError::ForeignKeyMissingName {
                    field: field.to_string(),
                    model: model.to_string(),
                }
            })?;

            let references = reader.take_path("references")?.ok_or_else(|| {
                ModelCodegenError::ForeignKeyMissingReferences {
                    field: field.to_string(),
                    model: model.to_string(),
                }
            })?;

            let on_delete = match reader.take_path("on_delete")? {
                None => OnDelete::NoAction,
                Some(path) if path.is_ident("cascade") => OnDelete::Cascade,
                Some(path) if path.is_ident("restrict") => OnDelete::Restrict,
                Some(path) if path.is_ident("set_null") => OnDelete::SetNull,
                Some(path) if path.is_ident("set_default") => OnDelete::SetDefault,
                Some(path) => {
                    return Err(ModelCodegenError::UnknownOnDeleteAction {
                        action: format_path(&path),
                        field: field.to_string(),
                        model: model.to_string(),
                    });
                }
            };

            Ok(Self {
                name,
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

    use margaret_attributes::attribute_args::AttributeArgs;
    use margaret_model::on_delete::OnDelete;

    use crate::foreign_key_arguments::ForeignKeyArguments;
    use crate::model_codegen_error::ModelCodegenError;

    fn parse(attribute: Attribute) -> Result<ForeignKeyArguments, ModelCodegenError> {
        let arguments = AttributeArgs::from_attribute(&attribute).expect("the arguments parse");

        ForeignKeyArguments::parse(&arguments, "crate::Model", "author")
    }

    fn on_delete(attribute: Attribute) -> OnDelete {
        parse(attribute)
            .expect("the foreign key arguments resolve")
            .on_delete
    }

    #[test]
    fn defaults_to_no_action_when_absent() {
        assert_eq!(
            on_delete(parse_quote!(#[foreign_key(name = "fk", references = Author::id)])),
            OnDelete::NoAction
        );
    }

    #[test]
    fn maps_cascade() {
        assert_eq!(
            on_delete(
                parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = cascade)])
            ),
            OnDelete::Cascade
        );
    }

    #[test]
    fn maps_restrict() {
        assert_eq!(
            on_delete(
                parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = restrict)])
            ),
            OnDelete::Restrict
        );
    }

    #[test]
    fn maps_set_null() {
        assert_eq!(
            on_delete(
                parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = set_null)])
            ),
            OnDelete::SetNull
        );
    }

    #[test]
    fn maps_set_default() {
        assert_eq!(
            on_delete(
                parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = set_default)])
            ),
            OnDelete::SetDefault
        );
    }

    #[test]
    fn rejects_no_action_as_an_explicit_value() {
        let message = parse(
            parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = no_action)]),
        )
        .err()
        .expect("no_action is not an accepted explicit value")
        .to_string();

        assert!(message.contains("unknown ON DELETE action"));
    }

    #[test]
    fn rejects_a_string_valued_action() {
        assert!(
            parse(
                parse_quote!(#[foreign_key(name = "fk", references = Author::id, on_delete = "cascade")])
            )
            .is_err()
        );
    }

    fn error_message(attribute: Attribute) -> String {
        parse(attribute)
            .err()
            .expect("the arguments are rejected")
            .to_string()
    }

    #[test]
    fn rejects_a_missing_name() {
        assert!(
            error_message(parse_quote!(#[foreign_key(references = Author::id)]))
                .contains("is missing the required 'name'")
        );
    }

    #[test]
    fn rejects_a_missing_references() {
        assert!(
            error_message(parse_quote!(#[foreign_key(name = "fk")]))
                .contains("is missing the required 'references'")
        );
    }

    #[test]
    fn rejects_a_non_string_name() {
        assert!(
            error_message(parse_quote!(#[foreign_key(name = 5, references = Author::id)]))
                .contains("failed to read the model attributes")
        );
    }

    #[test]
    fn rejects_non_path_references() {
        assert!(
            error_message(parse_quote!(#[foreign_key(name = "fk", references = "x")]))
                .contains("failed to read the model attributes")
        );
    }
}
