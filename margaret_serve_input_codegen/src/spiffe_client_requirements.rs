use crate::serve_input::ServeInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpiffeClientRequirements {
    pub http_client: bool,
    pub issuer_request_client: bool,
}

impl SpiffeClientRequirements {
    #[must_use]
    pub fn of<'inputs>(inputs: impl IntoIterator<Item = &'inputs ServeInput>) -> Self {
        inputs.into_iter().fold(
            Self {
                http_client: false,
                issuer_request_client: false,
            },
            |requirements, input| Self {
                http_client: requirements.http_client
                    || matches!(input, ServeInput::SpiffeHttpClient),
                issuer_request_client: requirements.issuer_request_client
                    || matches!(input, ServeInput::SpiffeIssuerRequestClient),
            },
        )
    }

    #[must_use]
    pub fn any(self) -> bool {
        self.http_client || self.issuer_request_client
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use super::SpiffeClientRequirements;
    use crate::serve_input::ServeInput;

    #[test]
    fn requires_the_spiffe_http_client_its_input_reads_so_serve_builds_it() {
        assert_eq!(
            SpiffeClientRequirements::of(&[ServeInput::SpiffeHttpClient]),
            SpiffeClientRequirements {
                http_client: true,
                issuer_request_client: false,
            }
        );
    }

    #[test]
    fn requires_the_spiffe_issuer_request_client_its_input_reads_so_serve_builds_it() {
        assert_eq!(
            SpiffeClientRequirements::of(&[ServeInput::SpiffeIssuerRequestClient]),
            SpiffeClientRequirements {
                http_client: false,
                issuer_request_client: true,
            }
        );
    }

    #[test]
    fn requires_no_spiffe_client_of_console_arguments_so_serve_skips_the_svid_bundle() {
        let named = ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: "label".to_string(),
            value: InputValue {
                required: true,
                value_type: CanonicalPath::new(vec![
                    "std".to_string(),
                    "string".to_string(),
                    "String".to_string(),
                ]),
                weaving: WeavingKind::BorrowedStr,
            },
        });

        assert!(!SpiffeClientRequirements::of(&[named]).any());
    }
}
