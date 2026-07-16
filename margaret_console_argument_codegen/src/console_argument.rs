use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

#[derive(Clone)]
pub enum ConsoleArgument {
    Flag {
        name: String,
    },
    Named {
        name: String,
        required: bool,
        value_type: Type,
    },
    Positional {
        id: String,
        required: bool,
        value_type: Type,
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
                if *required {
                    quote! { #value_type }
                } else {
                    quote! { ::std::option::Option<#value_type> }
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
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::ConsoleArgument;

    fn collapsed(argument: &ConsoleArgument) -> String {
        argument
            .field_type()
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_flag_field_is_a_bool() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(&flag), "bool");
        assert_eq!(flag.name(), "loud");
    }

    #[test]
    fn a_required_field_is_the_value_type() {
        let named = ConsoleArgument::Named {
            name: "path".to_string(),
            required: true,
            value_type: parse_quote!(PathBuf),
        };

        assert_eq!(collapsed(&named), "PathBuf");
        assert_eq!(named.name(), "path");
    }

    #[test]
    fn an_optional_field_is_an_option() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: false,
            value_type: parse_quote!(String),
        };

        assert_eq!(collapsed(&named), "::std::option::Option<String>");
    }

    #[test]
    fn a_positional_field_is_the_value_type_and_names_its_id() {
        let positional = ConsoleArgument::Positional {
            id: "point".to_string(),
            required: true,
            value_type: parse_quote!(Point),
        };

        assert_eq!(collapsed(&positional), "Point");
        assert_eq!(positional.name(), "point");
    }
}
