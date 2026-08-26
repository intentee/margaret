use syn::Type;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;
use margaret_input_weaving::input_value::InputValue;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_arguments::console_argument_arguments;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;
use crate::console_argument_scope::ConsoleArgumentScope;

fn bool_path() -> CanonicalPath {
    CanonicalPath::new(vec!["bool".to_string()])
}

fn is_flag(value: &InputValue) -> bool {
    value.required && value.value_type == bool_path()
}

/// # Errors
///
/// Returns `ConsoleArgumentCodegenError` propagated from the work it performs.
pub fn read_console_argument(
    index: &AttributeIndex,
    item: &IndexedItem,
    arguments: &AttributeArgs,
    declared: &Type,
    site: &ConstructorParameter,
    scope: ConsoleArgumentScope,
) -> Result<ConsoleArgument, ConsoleArgumentCodegenError> {
    let form = console_argument_arguments(arguments, site)?;
    let value = InputValue::from_declared(index, item, declared, site)?;

    match (form, is_flag(&value)) {
        (ConsoleArgumentForm::Named { key }, true) => Ok(ConsoleArgument::Flag { name: key }),
        (ConsoleArgumentForm::Positional, true) => {
            Err(ConsoleArgumentCodegenError::BooleanPositional { site: site.clone() })
        }
        (ConsoleArgumentForm::Named { key }, false) => {
            Ok(ConsoleArgument::Named { name: key, value })
        }
        (ConsoleArgumentForm::Positional, false) => match scope {
            ConsoleArgumentScope::ConsoleCommand => Ok(ConsoleArgument::Positional {
                id: site.parameter.clone(),
                value,
            }),
            ConsoleArgumentScope::ServeConstruction => {
                Err(ConsoleArgumentCodegenError::PositionalOutsideCommand { site: site.clone() })
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use syn::Attribute;
    use syn::Type;
    use syn::parse_quote;
    use tempfile::tempdir;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_input_weaving::constructor_parameter::ConstructorParameter;

    use crate::console_argument::ConsoleArgument;
    use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
    use crate::console_argument_scope::ConsoleArgumentScope;

    use super::read_console_argument;

    fn config_path() -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()])
    }

    fn indexed_crate() -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(
            source_directory.join("lib.rs"),
            "#[singleton]\nstruct Config;\n",
        )
        .expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn describe(attribute: &Attribute, declared: &Type, scope: ConsoleArgumentScope) -> String {
        match read(attribute, declared, scope) {
            Ok(ConsoleArgument::Flag { name }) => format!("flag:{name}"),
            Ok(ConsoleArgument::Named { name, value }) => format!(
                "named:{name}:{}:{}",
                requiredness(value.required),
                value.value_type
            ),
            Ok(ConsoleArgument::Positional { id, value }) => format!(
                "positional:{id}:{}:{}",
                requiredness(value.required),
                value.value_type
            ),
            Err(error) => error.to_string(),
        }
    }

    fn requiredness(required: bool) -> &'static str {
        if required { "required" } else { "optional" }
    }

    fn read(
        attribute: &Attribute,
        declared: &Type,
        scope: ConsoleArgumentScope,
    ) -> Result<ConsoleArgument, ConsoleArgumentCodegenError> {
        let index = indexed_crate();
        let item = index
            .item(&config_path())
            .expect("the indexed struct is present");

        read_console_argument(
            &index,
            item,
            &AttributeArgs::from_attribute(attribute).expect("the arguments parse"),
            declared,
            &ConstructorParameter {
                owner: config_path(),
                parameter: "label".to_string(),
            },
            scope,
        )
    }

    #[test]
    fn reads_a_required_named_argument() {
        assert_eq!(
            describe(
                &parse_quote!(#[console_argument(from = "label")]),
                &parse_quote!(String),
                ConsoleArgumentScope::ServeConstruction,
            ),
            "named:label:required:std::string::String"
        );
    }

    #[test]
    fn reads_an_optional_named_argument() {
        assert_eq!(
            describe(
                &parse_quote!(#[console_argument(from = "label")]),
                &parse_quote!(Option<String>),
                ConsoleArgumentScope::ServeConstruction,
            ),
            "named:label:optional:std::string::String"
        );
    }

    #[test]
    fn reads_a_required_boolean_as_a_flag() {
        assert_eq!(
            describe(
                &parse_quote!(#[console_argument(from = "loud")]),
                &parse_quote!(bool),
                ConsoleArgumentScope::ServeConstruction,
            ),
            "flag:loud"
        );
    }

    #[test]
    fn reads_an_optional_boolean_as_a_valued_argument() {
        assert_eq!(
            describe(
                &parse_quote!(#[console_argument(from = "loud")]),
                &parse_quote!(Option<bool>),
                ConsoleArgumentScope::ServeConstruction,
            ),
            "named:loud:optional:bool"
        );
    }

    #[test]
    fn reads_a_positional_argument_on_a_console_command() {
        assert_eq!(
            describe(
                &parse_quote!(#[console_argument(positional)]),
                &parse_quote!(String),
                ConsoleArgumentScope::ConsoleCommand,
            ),
            "positional:label:required:std::string::String"
        );
    }

    #[test]
    fn rejects_a_positional_argument_outside_a_console_command() {
        assert!(
            describe(
                &parse_quote!(#[console_argument(positional)]),
                &parse_quote!(String),
                ConsoleArgumentScope::ServeConstruction,
            )
            .contains("positional arguments are only allowed on console commands")
        );
    }

    #[test]
    fn rejects_a_boolean_positional_argument() {
        assert!(
            describe(
                &parse_quote!(#[console_argument(positional)]),
                &parse_quote!(bool),
                ConsoleArgumentScope::ConsoleCommand,
            )
            .contains("a boolean is a named flag, never positional")
        );
    }

    #[test]
    fn propagates_an_unusable_value_type() {
        assert!(
            describe(
                &parse_quote!(#[console_argument(from = "tags")]),
                &parse_quote!(Vec<String>),
                ConsoleArgumentScope::ServeConstruction,
            )
            .contains("an injected input value type must be a single concrete type")
        );
    }

    #[test]
    fn propagates_a_malformed_form() {
        assert!(
            describe(
                &parse_quote!(#[console_argument]),
                &parse_quote!(String),
                ConsoleArgumentScope::ServeConstruction,
            )
            .contains("it must be exactly one of them")
        );
    }
}
