use syn::Type;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;
use margaret_input_weaving::input_value::InputValue;

use crate::environment_variable::EnvironmentVariable;
use crate::environment_variable_arguments::environment_variable_arguments;
use crate::environment_variable_codegen_error::EnvironmentVariableCodegenError;

/// # Errors
///
/// Returns `EnvironmentVariableCodegenError` propagated from the work it performs.
pub fn read_environment_variable(
    index: &AttributeIndex,
    item: &IndexedItem,
    arguments: &AttributeArgs,
    declared: &Type,
    site: &ConstructorParameter,
) -> Result<EnvironmentVariable, EnvironmentVariableCodegenError> {
    Ok(EnvironmentVariable {
        name: environment_variable_arguments(arguments, site)?,
        value: InputValue::from_declared(index, item, declared, site)?,
    })
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

    use crate::environment_variable::EnvironmentVariable;
    use crate::environment_variable_codegen_error::EnvironmentVariableCodegenError;

    use super::read_environment_variable;

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

    fn describe(attribute: &Attribute, declared: &Type) -> String {
        match read(attribute, declared) {
            Ok(EnvironmentVariable { name, value }) => format!(
                "{name}:{}:{}:{:?}",
                requiredness(value.required),
                value.value_type,
                value.weaving
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
    ) -> Result<EnvironmentVariable, EnvironmentVariableCodegenError> {
        let index = indexed_crate();
        let item = index
            .item(&config_path())
            .expect("the indexed struct is present");

        read_environment_variable(
            &index,
            item,
            &AttributeArgs::from_attribute(attribute).expect("the arguments parse"),
            declared,
            &ConstructorParameter {
                owner: config_path(),
                parameter: "value".to_string(),
            },
        )
    }

    #[test]
    fn reads_a_required_variable() {
        assert_eq!(
            describe(
                &parse_quote!(#[environment_variable(from = "DATABASE_URL")]),
                &parse_quote!(String),
            ),
            "DATABASE_URL:required:std::string::String:BorrowedStr"
        );
    }

    #[test]
    fn reads_an_optional_variable() {
        assert_eq!(
            describe(
                &parse_quote!(#[environment_variable(from = "WORKER_COUNT")]),
                &parse_quote!(Option<u16>),
            ),
            "WORKER_COUNT:optional:u16:Copy"
        );
    }

    #[test]
    fn reads_a_boolean_as_an_ordinary_parsed_value() {
        assert_eq!(
            describe(
                &parse_quote!(#[environment_variable(from = "WORKER_DEBUG")]),
                &parse_quote!(bool),
            ),
            "WORKER_DEBUG:required:bool:Copy"
        );
    }

    #[test]
    fn propagates_a_malformed_name() {
        assert!(
            describe(
                &parse_quote!(#[environment_variable(from = "DATABASE URL")]),
                &parse_quote!(String),
            )
            .contains("is not a usable environment variable name")
        );
    }

    #[test]
    fn propagates_an_unusable_value_type() {
        assert!(
            describe(
                &parse_quote!(#[environment_variable(from = "TAGS")]),
                &parse_quote!(Vec<String>),
            )
            .contains("an injected input value type must be a single concrete type")
        );
    }
}
