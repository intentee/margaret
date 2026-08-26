use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;

use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;

pub(crate) fn console_argument_arguments(
    arguments: &AttributeArgs,
    site: &ConstructorParameter,
) -> Result<ConsoleArgumentForm, ConsoleArgumentCodegenError> {
    arguments.interpret(|reader| {
        let named = reader.take_string("from")?;
        let positional = reader.take_flag("positional");

        match (named, positional) {
            (Some(key), false) => Ok(ConsoleArgumentForm::Named { key }),
            (None, true) => Ok(ConsoleArgumentForm::Positional),
            (Some(_), true) => {
                Err(ConsoleArgumentCodegenError::NamedAndPositional { site: site.clone() })
            }
            (None, false) => {
                Err(ConsoleArgumentCodegenError::NeitherNamedNorPositional { site: site.clone() })
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::constructor_parameter::ConstructorParameter;

    use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
    use crate::console_argument_form::ConsoleArgumentForm;

    use super::console_argument_arguments;

    fn describe(attribute: &Attribute) -> String {
        match read(attribute) {
            Ok(ConsoleArgumentForm::Named { key }) => format!("named:{key}"),
            Ok(ConsoleArgumentForm::Positional) => "positional".to_string(),
            Err(error) => error.to_string(),
        }
    }

    fn read(attribute: &Attribute) -> Result<ConsoleArgumentForm, ConsoleArgumentCodegenError> {
        console_argument_arguments(
            &AttributeArgs::from_attribute(attribute).expect("the arguments parse"),
            &ConstructorParameter {
                owner: CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
                parameter: "label".to_string(),
            },
        )
    }

    #[test]
    fn reads_a_named_form() {
        assert_eq!(
            describe(&parse_quote!(#[console_argument(from = "label")])),
            "named:label"
        );
    }

    #[test]
    fn reads_a_positional_form() {
        assert_eq!(
            describe(&parse_quote!(#[console_argument(positional)])),
            "positional"
        );
    }

    #[test]
    fn rejects_a_form_that_is_both_named_and_positional() {
        assert!(
            describe(&parse_quote!(#[console_argument(from = "label", positional)]))
                .contains("an argument is either named or positional, never both")
        );
    }

    #[test]
    fn rejects_a_form_that_is_neither_named_nor_positional() {
        assert!(
            describe(&parse_quote!(#[console_argument])).contains("it must be exactly one of them")
        );
    }

    #[test]
    fn rejects_a_name_that_is_not_a_string_literal() {
        assert!(
            describe(&parse_quote!(#[console_argument(from = 5)]))
                .contains("failed to read the attribute arguments")
        );
    }
}
