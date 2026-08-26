use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;

use crate::serve_input_codegen_error::ServeInputCodegenError;

const SERVE_INPUT_ATTRIBUTES: [FrameworkAttribute; 3] = [
    FrameworkAttribute::ConsoleArgument,
    FrameworkAttribute::EnvironmentVariable,
    FrameworkAttribute::SpiffeHttpClient,
];

pub(crate) struct DeclaredServeInputSource<'attributes> {
    pub(crate) attribute: &'attributes IndexedAttribute,
    pub(crate) declared_by: FrameworkAttribute,
}

pub(crate) fn serve_input_source<'attributes>(
    attributes: &'attributes [IndexedAttribute],
    site: &ConstructorParameter,
) -> Result<Option<DeclaredServeInputSource<'attributes>>, ServeInputCodegenError> {
    let mut declared = SERVE_INPUT_ATTRIBUTES.into_iter().filter_map(|marker| {
        attributes
            .iter()
            .find(|attribute| attribute.framework_attribute() == Some(marker))
            .map(|attribute| DeclaredServeInputSource {
                attribute,
                declared_by: marker,
            })
    });

    let Some(source) = declared.next() else {
        return Ok(None);
    };

    match declared.next() {
        None => Ok(Some(source)),
        Some(second) => Err(ServeInputCodegenError::AmbiguousServeInput {
            first: source.declared_by.name(),
            second: second.declared_by.name(),
            site: site.clone(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::framework_attribute::FrameworkAttribute;
    use margaret_attributes::indexed_attribute::IndexedAttribute;
    use margaret_input_weaving::constructor_parameter::ConstructorParameter;

    use crate::serve_input_codegen_error::ServeInputCodegenError;

    use super::serve_input_source;

    fn site() -> ConstructorParameter {
        ConstructorParameter {
            owner: CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            parameter: "value".to_string(),
        }
    }

    fn declared_by(attributes: &[IndexedAttribute]) -> Option<FrameworkAttribute> {
        describe(attributes).expect("a single source is read")
    }

    fn describe(
        attributes: &[IndexedAttribute],
    ) -> Result<Option<FrameworkAttribute>, ServeInputCodegenError> {
        serve_input_source(attributes, &site()).map(|read| read.map(|source| source.declared_by))
    }

    #[test]
    fn reports_no_source_on_a_plain_dependency() {
        assert_eq!(
            declared_by(&[IndexedAttribute::new(
                &parse_quote!(#[jwks_secret_store(server)])
            )]),
            None
        );
    }

    #[test]
    fn finds_the_console_argument_source() {
        assert_eq!(
            declared_by(&[IndexedAttribute::new(
                &parse_quote!(#[console_argument(from = "label")]),
            )]),
            Some(FrameworkAttribute::ConsoleArgument)
        );
    }

    #[test]
    fn finds_the_environment_variable_source() {
        assert_eq!(
            declared_by(&[IndexedAttribute::new(
                &parse_quote!(#[environment_variable(from = "DATABASE_URL")]),
            )]),
            Some(FrameworkAttribute::EnvironmentVariable)
        );
    }

    #[test]
    fn finds_the_spiffe_http_client_source() {
        assert_eq!(
            declared_by(&[IndexedAttribute::new(&parse_quote!(#[spiffe_http_client]),)]),
            Some(FrameworkAttribute::SpiffeHttpClient)
        );
    }

    #[test]
    fn rejects_a_parameter_that_declares_two_sources() {
        let attributes = [
            IndexedAttribute::new(&parse_quote!(#[console_argument(from = "label")])),
            IndexedAttribute::new(&parse_quote!(#[environment_variable(from = "LABEL")])),
        ];

        assert!(
            describe(&attributes)
                .expect_err("two sources at once are rejected")
                .to_string()
                .contains("declares both #[console_argument] and #[environment_variable]")
        );
    }
}
