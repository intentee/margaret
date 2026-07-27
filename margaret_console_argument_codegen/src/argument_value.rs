use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;

use crate::console_argument::ConsoleArgument;
use crate::required_flag_read::required_flag_read;
use crate::weaving_kind::WeavingKind;

fn value_expression(
    id: &str,
    required: bool,
    is_copy: bool,
    value_type: &CanonicalPath,
) -> TokenStream {
    let value_type = path_tokens(value_type);

    if required {
        let present = if is_copy {
            quote! { *value }
        } else {
            quote! { value.clone() }
        };

        required_flag_read(&value_type, id, &present)
    } else {
        if is_copy {
            quote! { matches.get_one::<#value_type>(#id).copied() }
        } else {
            quote! { matches.get_one::<#value_type>(#id).cloned() }
        }
    }
}

#[must_use]
pub fn argument_value(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! { matches.get_flag(#name) },
        ConsoleArgument::Named {
            name,
            required,
            weaving,
            value_type,
        } => value_expression(
            name,
            *required,
            matches!(weaving, WeavingKind::Copy),
            value_type,
        ),
        ConsoleArgument::Positional {
            id,
            required,
            weaving,
            value_type,
        } => value_expression(
            id,
            *required,
            matches!(weaving, WeavingKind::Copy),
            value_type,
        ),
        ConsoleArgument::SpiffeHttpClient => {
            let spiffe_http_client = spiffe_http_client_ident();

            quote! { #spiffe_http_client.clone() }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::console_argument::ConsoleArgument;
    use crate::weaving_kind::WeavingKind;

    use super::argument_value;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn collapsed(argument: &ConsoleArgument) -> String {
        argument_value(argument)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_flag_reads_from_get_flag() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(&flag), r#"matches.get_flag("loud")"#);
    }

    #[test]
    fn a_required_copy_argument_dereferences_the_read_value() {
        let named = ConsoleArgument::Named {
            name: "retries".to_string(),
            required: true,
            weaving: WeavingKind::Copy,
            value_type: path(&["u16"]),
        };

        assert!(
            collapsed(&named).contains("matches.get_one::<u16>(\"retries\"){Some(value)=>*value")
        );
    }

    #[test]
    fn a_required_non_copy_argument_clones_the_qualified_read_value() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: path(&["std", "string", "String"]),
        };

        assert!(collapsed(&named).contains(
            "matches.get_one::<std::string::String>(\"label\"){Some(value)=>value.clone()"
        ));
    }

    #[test]
    fn an_optional_argument_reads_a_cloned_qualified_option() {
        let named = ConsoleArgument::Named {
            name: "note".to_string(),
            required: false,
            weaving: WeavingKind::Cloned,
            value_type: path(&["std", "string", "String"]),
        };

        assert_eq!(
            collapsed(&named),
            r#"matches.get_one::<std::string::String>("note").cloned()"#
        );
    }

    #[test]
    fn an_optional_copy_argument_reads_a_copied_qualified_option() {
        let named = ConsoleArgument::Named {
            name: "retries".to_string(),
            required: false,
            weaving: WeavingKind::Copy,
            value_type: path(&["u16"]),
        };

        assert_eq!(
            collapsed(&named),
            r#"matches.get_one::<u16>("retries").copied()"#
        );
    }

    #[test]
    fn a_positional_argument_reads_by_its_id() {
        let positional = ConsoleArgument::Positional {
            id: "point".to_string(),
            required: true,
            weaving: WeavingKind::Cloned,
            value_type: path(&["crate", "geometry", "Point"]),
        };

        assert!(
            collapsed(&positional)
                .contains(r#"matches.get_one::<crate::geometry::Point>("point")"#)
        );
    }

    #[test]
    fn a_spiffe_http_client_clones_the_serve_local() {
        assert_eq!(
            collapsed(&ConsoleArgument::SpiffeHttpClient),
            "spiffe_http_client.clone()"
        );
    }
}
