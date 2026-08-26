use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;

use crate::environment_variable_codegen_error::EnvironmentVariableCodegenError;
use crate::environment_variable_name::EnvironmentVariableName;

pub(crate) fn environment_variable_arguments(
    arguments: &AttributeArgs,
    site: &ConstructorParameter,
) -> Result<EnvironmentVariableName, EnvironmentVariableCodegenError> {
    arguments.interpret(|reader| {
        let Some(declared) = reader.take_string("from")? else {
            return Err(EnvironmentVariableCodegenError::MissingName { site: site.clone() });
        };

        EnvironmentVariableName::new(&declared).ok_or(
            EnvironmentVariableCodegenError::MalformedName {
                name: declared,
                site: site.clone(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::constructor_parameter::ConstructorParameter;

    use crate::environment_variable_codegen_error::EnvironmentVariableCodegenError;
    use crate::environment_variable_name::EnvironmentVariableName;

    use super::environment_variable_arguments;

    fn site() -> ConstructorParameter {
        ConstructorParameter {
            owner: CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            parameter: "database_url".to_string(),
        }
    }

    fn describe(attribute: &Attribute) -> String {
        match read(attribute) {
            Ok(name) => name.as_str().to_string(),
            Err(error) => error.to_string(),
        }
    }

    fn read(
        attribute: &Attribute,
    ) -> Result<EnvironmentVariableName, EnvironmentVariableCodegenError> {
        environment_variable_arguments(
            &AttributeArgs::from_attribute(attribute).expect("the arguments parse"),
            &site(),
        )
    }

    #[test]
    fn reads_the_declared_name() {
        assert_eq!(
            describe(&parse_quote!(#[environment_variable(from = "DATABASE_URL")])),
            "DATABASE_URL"
        );
    }

    #[test]
    fn rejects_a_bare_attribute() {
        assert!(
            describe(&parse_quote!(#[environment_variable]))
                .contains("an environment variable is always named by the variable it reads")
        );
    }

    #[test]
    fn rejects_a_malformed_name() {
        assert!(
            describe(&parse_quote!(#[environment_variable(from = "DATABASE-URL")]))
                .contains("'DATABASE-URL', which is not a usable environment variable name")
        );
    }

    #[test]
    fn rejects_a_name_that_is_not_a_string_literal() {
        assert!(
            describe(&parse_quote!(#[environment_variable(from = 5)]))
                .contains("failed to read the attribute arguments")
        );
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert!(
            describe(&parse_quote!(#[environment_variable(from = "DATABASE_URL", positional)]))
                .contains("failed to read the attribute arguments")
        );
    }
}
