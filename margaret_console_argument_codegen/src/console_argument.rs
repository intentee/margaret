use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::threading_kind::ThreadingKind;

#[derive(Clone, Debug, PartialEq)]
pub enum ConsoleArgument {
    Flag {
        name: String,
    },
    Named {
        name: String,
        required: bool,
        threading: ThreadingKind,
        value_type: CanonicalPath,
    },
    Positional {
        id: String,
        required: bool,
        threading: ThreadingKind,
        value_type: CanonicalPath,
    },
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
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            ConsoleArgument::Flag { name } => name,
            ConsoleArgument::Named { name, .. } => name,
            ConsoleArgument::Positional { id, .. } => id,
        }
    }

    #[must_use]
    pub fn parameter_referent(&self) -> TokenStream {
        match self.threading() {
            ThreadingKind::BorrowedStr => quote! { str },
            ThreadingKind::BorrowedPath => quote! { ::std::path::Path },
            ThreadingKind::Copy | ThreadingKind::Cloned => self.field_type(),
        }
    }

    #[must_use]
    pub fn threading(&self) -> ThreadingKind {
        match self {
            ConsoleArgument::Flag { .. } => ThreadingKind::Copy,
            ConsoleArgument::Named { threading, .. }
            | ConsoleArgument::Positional { threading, .. } => threading.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::threading_kind::ThreadingKind;

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
            threading: ThreadingKind::BorrowedPath,
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
            threading: ThreadingKind::Cloned,
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
            threading: ThreadingKind::Cloned,
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

        assert_eq!(flag.threading(), ThreadingKind::Copy);
        assert_eq!(collapsed(flag.parameter_referent()), "bool");
    }

    #[test]
    fn a_positional_threads_by_its_stored_kind() {
        let positional = ConsoleArgument::Positional {
            id: "count".to_string(),
            required: true,
            threading: ThreadingKind::Copy,
            value_type: path(&["u16"]),
        };

        assert_eq!(positional.threading(), ThreadingKind::Copy);
    }

    #[test]
    fn a_borrowed_str_parameter_referent_is_str() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            threading: ThreadingKind::BorrowedStr,
            value_type: path(&["std", "string", "String"]),
        };

        assert_eq!(collapsed(named.parameter_referent()), "str");
    }

    #[test]
    fn a_borrowed_path_parameter_referent_is_the_path_type() {
        let named = ConsoleArgument::Named {
            name: "config".to_string(),
            required: true,
            threading: ThreadingKind::BorrowedPath,
            value_type: path(&["std", "path", "PathBuf"]),
        };

        assert_eq!(collapsed(named.parameter_referent()), "::std::path::Path");
    }
}
