use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_input_weaving::input_value::InputValue;
use margaret_input_weaving::weaving_kind::WeavingKind;

use crate::console_argument::ConsoleArgument;
use crate::required_flag_read::required_flag_read;

fn failed_outcome() -> TokenStream {
    quote! { return margaret::framework::console::command_outcome::CommandOutcome::Failed }
}

fn valued_read(
    id: &str,
    InputValue {
        required,
        value_type,
        weaving,
    }: &InputValue,
) -> TokenStream {
    let is_copy = weaving == &WeavingKind::Copy;
    let value_type = path_tokens(value_type);

    if *required {
        let present = if is_copy {
            quote! { *value }
        } else {
            quote! { value.clone() }
        };

        required_flag_read(&value_type, id, &present, &failed_outcome())
    } else if is_copy {
        quote! { matches.get_one::<#value_type>(#id).copied() }
    } else {
        quote! { matches.get_one::<#value_type>(#id).cloned() }
    }
}

#[must_use]
pub fn console_argument_read(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! { matches.get_flag(#name) },
        ConsoleArgument::Named { name, value } => valued_read(name, value),
        ConsoleArgument::Positional { id, value } => valued_read(id, value),
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::console_argument::ConsoleArgument;

    use super::console_argument_read;

    fn value(required: bool, weaving: WeavingKind, segments: &[&str]) -> InputValue {
        InputValue {
            required,
            value_type: CanonicalPath::new(
                segments
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            ),
            weaving,
        }
    }

    fn collapsed(argument: &ConsoleArgument) -> String {
        console_argument_read(argument)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_flag_reads_from_get_flag() {
        assert_eq!(
            collapsed(&ConsoleArgument::Flag {
                name: "loud".to_string(),
            }),
            r#"matches.get_flag("loud")"#
        );
    }

    #[test]
    fn a_required_copy_argument_dereferences_the_read_value() {
        assert!(
            collapsed(&ConsoleArgument::Named {
                name: "retries".to_string(),
                value: value(true, WeavingKind::Copy, &["u16"]),
            })
            .contains("matches.get_one::<u16>(\"retries\"){Some(value)=>*value")
        );
    }

    #[test]
    fn a_required_non_copy_argument_clones_the_qualified_read_value() {
        assert!(
            collapsed(&ConsoleArgument::Named {
                name: "label".to_string(),
                value: value(true, WeavingKind::BorrowedStr, &["std", "string", "String"]),
            })
            .contains(
                "matches.get_one::<std::string::String>(\"label\"){Some(value)=>value.clone()"
            )
        );
    }

    #[test]
    fn an_optional_argument_reads_a_cloned_qualified_option() {
        assert_eq!(
            collapsed(&ConsoleArgument::Named {
                name: "note".to_string(),
                value: value(false, WeavingKind::Cloned, &["std", "string", "String"]),
            }),
            r#"matches.get_one::<std::string::String>("note").cloned()"#
        );
    }

    #[test]
    fn an_optional_copy_argument_reads_a_copied_qualified_option() {
        assert_eq!(
            collapsed(&ConsoleArgument::Named {
                name: "retries".to_string(),
                value: value(false, WeavingKind::Copy, &["u16"]),
            }),
            r#"matches.get_one::<u16>("retries").copied()"#
        );
    }

    #[test]
    fn a_positional_argument_reads_by_its_id() {
        assert!(
            collapsed(&ConsoleArgument::Positional {
                id: "point".to_string(),
                value: value(true, WeavingKind::Cloned, &["crate", "geometry", "Point"]),
            })
            .contains(r#"matches.get_one::<crate::geometry::Point>("point")"#)
        );
    }
}
