use crate::console_argument::ConsoleArgument;

#[must_use]
pub fn has_spiffe_http_client(arguments: &[ConsoleArgument]) -> bool {
    arguments
        .iter()
        .any(|argument| matches!(argument, ConsoleArgument::SpiffeHttpClient))
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::console_argument::ConsoleArgument;
    use crate::weaving_kind::WeavingKind;

    use super::has_spiffe_http_client;

    #[test]
    fn detects_the_spiffe_http_client_input() {
        assert!(has_spiffe_http_client(&[ConsoleArgument::SpiffeHttpClient]));
    }

    #[test]
    fn ignores_console_arguments() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
        };

        assert!(!has_spiffe_http_client(&[named]));
    }
}
