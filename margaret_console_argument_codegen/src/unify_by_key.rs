use std::collections::BTreeMap;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;

pub fn unify_by_key(
    arguments: &[ConsoleArgument],
) -> Result<Vec<ConsoleArgument>, ConsoleArgumentCodegenError> {
    let mut order: Vec<String> = Vec::new();
    let mut unified: BTreeMap<String, ConsoleArgument> = BTreeMap::new();

    for argument in arguments {
        let name = argument.name().to_string();

        match unified.get(&name) {
            Some(existing) => {
                let collides = matches!(argument, ConsoleArgument::Positional { .. })
                    || matches!(existing, ConsoleArgument::Positional { .. });

                if collides {
                    return Err(ConsoleArgumentCodegenError::ConflictingConsoleArgumentId { name });
                }
            }
            None => {
                unified.insert(name.clone(), argument.clone());
                order.push(name);
            }
        }
    }

    Ok(order
        .into_iter()
        .map(|name| unified[&name].clone())
        .collect())
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::unify_by_key;
    use crate::console_argument::ConsoleArgument;

    fn named(name: &str) -> ConsoleArgument {
        ConsoleArgument::Named {
            name: name.to_string(),
            required: true,
            value_type: parse_quote!(String),
        }
    }

    fn positional(id: &str) -> ConsoleArgument {
        ConsoleArgument::Positional {
            id: id.to_string(),
            required: true,
            value_type: parse_quote!(String),
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
}
