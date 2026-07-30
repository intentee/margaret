use crate::console_argument::ConsoleArgument;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::serve_input_registry::ServeInputRegistry;

/// # Errors
///
/// Returns `ConsoleArgumentCodegenError` propagated from the work it performs.
pub fn unify_by_key(
    arguments: &[ConsoleArgument],
) -> Result<Vec<ConsoleArgument>, ConsoleArgumentCodegenError> {
    let mut registry = ServeInputRegistry::empty();

    for argument in arguments {
        registry.register(argument.clone())?;
    }

    Ok(registry.into_parts().0)
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::unify_by_key;
    use crate::console_argument::ConsoleArgument;
    use crate::serve_input_key::ServeInputKey;
    use crate::weaving_kind::WeavingKind;

    fn string_type() -> CanonicalPath {
        CanonicalPath::new(vec![
            "std".to_string(),
            "string".to_string(),
            "String".to_string(),
        ])
    }

    fn named(name: &str) -> ConsoleArgument {
        ConsoleArgument::Named {
            name: name.to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: string_type(),
        }
    }

    fn positional(id: &str) -> ConsoleArgument {
        ConsoleArgument::Positional {
            id: id.to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: string_type(),
        }
    }

    #[test]
    fn keeps_distinct_keys_in_first_seen_order() {
        let unified =
            unify_by_key(&[named("first"), named("second")]).expect("distinct keys unify");

        assert_eq!(unified.len(), 2);
        assert_eq!(unified[0].name(), "first");
        assert_eq!(unified[1].name(), "second");
    }

    #[test]
    fn collapses_a_repeated_named_key() {
        let unified = unify_by_key(&[named("shared"), named("shared")]).expect("the key unifies");

        assert_eq!(unified.len(), 1);
        assert_eq!(unified[0].name(), "shared");
    }

    #[test]
    fn rejects_a_later_positional_colliding_with_a_named_key() {
        let error = unify_by_key(&[named("name"), positional("name")])
            .expect_err("the positional collision is rejected")
            .to_string();

        assert!(error.contains("declared both as a positional and as a named argument"));
    }

    #[test]
    fn rejects_a_later_named_colliding_with_a_positional() {
        let error = unify_by_key(&[positional("name"), named("name")])
            .expect_err("the named collision is rejected")
            .to_string();

        assert!(error.contains("declared both as a positional and as a named argument"));
    }

    #[test]
    fn keeps_a_console_argument_named_after_the_spiffe_http_client_alongside_the_injection() {
        let unified = unify_by_key(&[
            named("spiffe_http_client"),
            ConsoleArgument::SpiffeHttpClient,
        ])
        .expect("the framework binding lives in a separate namespace from console arguments");

        assert_eq!(unified.len(), 2);
        assert_eq!(unified[0].name(), "spiffe_http_client");
        assert_eq!(unified[1].slot_key(), ServeInputKey::SpiffeHttpClient);
    }

    #[test]
    fn collapses_two_spiffe_http_client_inputs_into_one() {
        let unified = unify_by_key(&[
            ConsoleArgument::SpiffeHttpClient,
            ConsoleArgument::SpiffeHttpClient,
        ])
        .expect("two injections of the one shared client unify");

        assert_eq!(unified.len(), 1);
    }
}
