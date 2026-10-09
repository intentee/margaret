use crate::serve_input::ServeInput;

#[must_use]
pub fn has_spiffe_http_client<'inputs>(
    inputs: impl IntoIterator<Item = &'inputs ServeInput>,
) -> bool {
    inputs
        .into_iter()
        .any(|input| matches!(input, ServeInput::SpiffeHttpClient))
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::serve_input::ServeInput;

    use super::has_spiffe_http_client;

    #[test]
    fn detects_the_spiffe_http_client_input() {
        assert!(has_spiffe_http_client(&[ServeInput::SpiffeHttpClient]));
    }

    #[test]
    fn ignores_console_arguments() {
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

        assert!(!has_spiffe_http_client(&[named]));
    }
}
