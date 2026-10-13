use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_input_weaving::input_value::InputValue;

use crate::console_argument::ConsoleArgument;

fn valued_read(
    id: &str,
    InputValue {
        required,
        value_type,
        ..
    }: &InputValue,
) -> TokenStream {
    let value_type = path_tokens(value_type);

    if *required {
        quote! {
            margaret::framework::console::required_argument::required_argument::<#value_type>(matches, #id)
        }
    } else {
        quote! {
            margaret::framework::console::optional_argument::optional_argument::<#value_type>(matches, #id)
        }
    }
}

#[must_use]
pub fn console_argument_read(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! {
            margaret::framework::console::flag_argument::flag_argument(matches, #name)
        },
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

    fn value(required: bool, segments: &[&str]) -> InputValue {
        InputValue {
            required,
            value_type: CanonicalPath::new(segments.iter().map(ToString::to_string).collect()),
            weaving: WeavingKind::Cloned,
        }
    }

    fn collapsed(argument: &ConsoleArgument) -> String {
        console_argument_read(argument)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn reads_a_flag_through_the_framework_flag_reader_since_a_flag_holds_a_boolean() {
        assert_eq!(
            collapsed(&ConsoleArgument::Flag {
                name: "loud".to_string(),
            }),
            r#"margaret::framework::console::flag_argument::flag_argument(matches,"loud")"#
        );
    }

    #[test]
    fn reads_a_required_argument_through_the_framework_so_no_missing_branch_is_generated() {
        assert_eq!(
            collapsed(&ConsoleArgument::Named {
                name: "label".to_string(),
                value: value(true, &["std", "string", "String"]),
            }),
            r#"margaret::framework::console::required_argument::required_argument::<::std::string::String>(matches,"label")"#
        );
    }

    #[test]
    fn reads_an_optional_argument_as_an_option_of_its_qualified_type() {
        assert_eq!(
            collapsed(&ConsoleArgument::Named {
                name: "note".to_string(),
                value: value(false, &["std", "string", "String"]),
            }),
            r#"margaret::framework::console::optional_argument::optional_argument::<::std::string::String>(matches,"note")"#
        );
    }

    #[test]
    fn reads_a_positional_argument_by_its_id() {
        assert_eq!(
            collapsed(&ConsoleArgument::Positional {
                id: "point".to_string(),
                value: value(true, &["crate", "geometry", "Point"]),
            }),
            r#"margaret::framework::console::required_argument::required_argument::<crate::geometry::Point>(matches,"point")"#
        );
    }
}
