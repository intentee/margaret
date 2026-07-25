use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::serve_input_key::ServeInputKey;
use crate::weaving_kind::WeavingKind;

#[derive(Clone, Debug, PartialEq)]
pub enum ConsoleArgument {
    Flag {
        name: String,
    },
    Named {
        name: String,
        required: bool,
        weaving: WeavingKind,
        value_type: CanonicalPath,
    },
    Positional {
        id: String,
        required: bool,
        weaving: WeavingKind,
        value_type: CanonicalPath,
    },
    SpiffeHttpClient,
}

impl ConsoleArgument {
    #[must_use]
    pub fn field_type(&self) -> TokenStream {
        match self {
            ConsoleArgument::Flag { .. } => quote! { bool },
            ConsoleArgument::Named {
                required,
                value_type,
                ..
            }
            | ConsoleArgument::Positional {
                required,
                value_type,
                ..
            } => {
                let type_tokens = path_tokens(value_type);

                if *required {
                    type_tokens
                } else {
                    quote! { ::std::option::Option<#type_tokens> }
                }
            }
            ConsoleArgument::SpiffeHttpClient => quote! { reqwest::Client },
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            ConsoleArgument::Flag { name } => name,
            ConsoleArgument::Named { name, .. } => name,
            ConsoleArgument::Positional { id, .. } => id,
            ConsoleArgument::SpiffeHttpClient => "spiffe_http_client",
        }
    }

    #[must_use]
    pub fn parameter_referent(&self) -> TokenStream {
        match self.weaving() {
            WeavingKind::BorrowedStr => quote! { str },
            WeavingKind::BorrowedPath => quote! { ::std::path::Path },
            WeavingKind::Copy | WeavingKind::Cloned => self.field_type(),
        }
    }

    #[must_use]
    pub fn slot_key(&self) -> ServeInputKey {
        match self {
            ConsoleArgument::Flag { name } | ConsoleArgument::Named { name, .. } => {
                ServeInputKey::ConsoleArgument { name: name.clone() }
            }
            ConsoleArgument::Positional { id, .. } => {
                ServeInputKey::ConsoleArgument { name: id.clone() }
            }
            ConsoleArgument::SpiffeHttpClient => ServeInputKey::SpiffeHttpClient,
        }
    }

    #[must_use]
    pub fn weaving(&self) -> WeavingKind {
        match self {
            ConsoleArgument::Flag { .. } => WeavingKind::Copy,
            ConsoleArgument::Named { weaving, .. }
            | ConsoleArgument::Positional { weaving, .. } => weaving.clone(),
            ConsoleArgument::SpiffeHttpClient => WeavingKind::Cloned,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::serve_input_key::ServeInputKey;
    use crate::weaving_kind::WeavingKind;

    use super::ConsoleArgument;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn collapsed(tokens: proc_macro2::TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    #[test]
    fn a_flag_field_is_a_bool() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(flag.field_type()), "bool");
        assert_eq!(flag.name(), "loud");
    }

    #[test]
    fn a_required_field_is_the_qualified_value_type() {
        let named = ConsoleArgument::Named {
            name: "config".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedPath,
            value_type: path(&["std", "path", "PathBuf"]),
        };

        assert_eq!(collapsed(named.field_type()), "std::path::PathBuf");
        assert_eq!(named.name(), "config");
    }

    #[test]
    fn an_optional_field_is_an_option_of_the_qualified_value_type() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: false,
            weaving: WeavingKind::Cloned,
            value_type: path(&["std", "string", "String"]),
        };

        assert_eq!(
            collapsed(named.field_type()),
            "::std::option::Option<std::string::String>"
        );
    }

    #[test]
    fn a_positional_field_is_the_qualified_value_type_and_names_its_id() {
        let positional = ConsoleArgument::Positional {
            id: "point".to_string(),
            required: true,
            weaving: WeavingKind::Cloned,
            value_type: path(&["crate", "geometry", "Point"]),
        };

        assert_eq!(collapsed(positional.field_type()), "crate::geometry::Point");
        assert_eq!(positional.name(), "point");
    }

    #[test]
    fn a_flag_parameter_referent_is_its_bool_field_type() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(flag.weaving(), WeavingKind::Copy);
        assert_eq!(collapsed(flag.parameter_referent()), "bool");
    }

    #[test]
    fn a_positional_weaves_by_its_stored_kind() {
        let positional = ConsoleArgument::Positional {
            id: "count".to_string(),
            required: true,
            weaving: WeavingKind::Copy,
            value_type: path(&["u16"]),
        };

        assert_eq!(positional.weaving(), WeavingKind::Copy);
    }

    #[test]
    fn a_borrowed_str_parameter_referent_is_str() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: path(&["std", "string", "String"]),
        };

        assert_eq!(collapsed(named.parameter_referent()), "str");
    }

    #[test]
    fn a_borrowed_path_parameter_referent_is_the_path_type() {
        let named = ConsoleArgument::Named {
            name: "config".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedPath,
            value_type: path(&["std", "path", "PathBuf"]),
        };

        assert_eq!(collapsed(named.parameter_referent()), "::std::path::Path");
    }

    #[test]
    fn a_named_argument_keys_its_slot_by_its_name() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: path(&["std", "string", "String"]),
        };

        assert_eq!(
            named.slot_key(),
            ServeInputKey::ConsoleArgument {
                name: "label".to_string()
            }
        );
    }

    #[test]
    fn a_positional_argument_keys_its_slot_by_its_id() {
        let positional = ConsoleArgument::Positional {
            id: "point".to_string(),
            required: true,
            weaving: WeavingKind::Cloned,
            value_type: path(&["crate", "geometry", "Point"]),
        };

        assert_eq!(
            positional.slot_key(),
            ServeInputKey::ConsoleArgument {
                name: "point".to_string()
            }
        );
    }

    #[test]
    fn a_spiffe_http_client_keys_its_slot_in_the_framework_namespace() {
        assert_eq!(
            ConsoleArgument::SpiffeHttpClient.slot_key(),
            ServeInputKey::SpiffeHttpClient
        );
    }

    #[test]
    fn a_spiffe_http_client_field_is_a_reqwest_client() {
        let client = ConsoleArgument::SpiffeHttpClient;

        assert_eq!(collapsed(client.field_type()), "reqwest::Client");
        assert_eq!(client.name(), "spiffe_http_client");
    }

    #[test]
    fn a_spiffe_http_client_weaves_by_cloning_its_value_type() {
        let client = ConsoleArgument::SpiffeHttpClient;

        assert_eq!(client.weaving(), WeavingKind::Cloned);
        assert_eq!(collapsed(client.parameter_referent()), "reqwest::Client");
    }
}
