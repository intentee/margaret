use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_input_weaving::input_value::InputValue;

use crate::console_argument::ConsoleArgument;

fn valued_registration(
    id: &str,
    long: &TokenStream,
    InputValue {
        required,
        value_type,
        ..
    }: &InputValue,
) -> TokenStream {
    let value_type = path_tokens(value_type);

    quote! {
        .arg(
            clap::Arg::new(#id)
                #long
                .required(#required)
                .value_parser(clap::value_parser!(#value_type)),
        )
    }
}

#[must_use]
pub fn console_argument_registration(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! {
            .arg(clap::Arg::new(#name).long(#name).action(clap::ArgAction::SetTrue))
        },
        ConsoleArgument::Named { name, value } => {
            valued_registration(name, &quote! { .long(#name) }, value)
        }
        ConsoleArgument::Positional { id, value } => {
            valued_registration(id, &TokenStream::new(), value)
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::console_argument::ConsoleArgument;

    use super::console_argument_registration;

    fn value(required: bool, segments: &[&str]) -> InputValue {
        InputValue {
            required,
            value_type: CanonicalPath::new(segments.iter().map(ToString::to_string).collect()),
            weaving: WeavingKind::Cloned,
        }
    }

    fn collapsed(argument: &ConsoleArgument) -> String {
        console_argument_registration(argument)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_flag_registers_a_set_true_action() {
        assert_eq!(
            collapsed(&ConsoleArgument::Flag {
                name: "loud".to_string(),
            }),
            r#".arg(clap::Arg::new("loud").long("loud").action(clap::ArgAction::SetTrue))"#
        );
    }

    #[test]
    fn a_required_named_argument_registers_a_long_flag_with_a_value_parser() {
        assert_eq!(
            collapsed(&ConsoleArgument::Named {
                name: "config".to_string(),
                value: value(true, &["std", "path", "PathBuf"]),
            }),
            r#".arg(clap::Arg::new("config").long("config").required(true).value_parser(clap::value_parser!(std::path::PathBuf)),)"#
        );
    }

    #[test]
    fn an_optional_named_argument_is_not_required() {
        assert!(
            collapsed(&ConsoleArgument::Named {
                name: "note".to_string(),
                value: value(false, &["std", "string", "String"]),
            })
            .contains(".required(false)")
        );
    }

    #[test]
    fn a_positional_argument_registers_without_a_long_flag() {
        assert_eq!(
            collapsed(&ConsoleArgument::Positional {
                id: "name".to_string(),
                value: value(true, &["std", "string", "String"]),
            }),
            r#".arg(clap::Arg::new("name").required(true).value_parser(clap::value_parser!(std::string::String)),)"#
        );
    }
}
