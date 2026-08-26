use proc_macro2::TokenStream;
use quote::quote;

use margaret_input_weaving::input_value::InputValue;
use margaret_input_weaving::weaving_kind::WeavingKind;

#[derive(Clone, Debug, PartialEq)]
pub enum ConsoleArgument {
    Flag { name: String },
    Named { name: String, value: InputValue },
    Positional { id: String, value: InputValue },
}

impl ConsoleArgument {
    #[must_use]
    pub fn field_type(&self) -> TokenStream {
        match self {
            ConsoleArgument::Flag { .. } => quote! { bool },
            ConsoleArgument::Named { value, .. } | ConsoleArgument::Positional { value, .. } => {
                value.field_type()
            }
        }
    }

    #[must_use]
    pub fn is_shareable(&self) -> bool {
        !matches!(self, ConsoleArgument::Positional { .. })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            ConsoleArgument::Flag { name } | ConsoleArgument::Named { name, .. } => name,
            ConsoleArgument::Positional { id, .. } => id,
        }
    }

    #[must_use]
    pub fn weaving(&self) -> WeavingKind {
        match self {
            ConsoleArgument::Flag { .. } => WeavingKind::Copy,
            ConsoleArgument::Named { value, .. } | ConsoleArgument::Positional { value, .. } => {
                value.weaving.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use super::ConsoleArgument;

    fn collapsed(tokens: &proc_macro2::TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

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

    #[test]
    fn a_flag_is_a_shareable_copy_bool() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(&flag.field_type()), "bool");
        assert_eq!(flag.name(), "loud");
        assert_eq!(flag.weaving(), WeavingKind::Copy);
        assert!(flag.is_shareable());
    }

    #[test]
    fn a_named_argument_takes_its_shape_from_its_value() {
        let named = ConsoleArgument::Named {
            name: "config".to_string(),
            value: value(true, WeavingKind::BorrowedPath, &["std", "path", "PathBuf"]),
        };

        assert_eq!(collapsed(&named.field_type()), "std::path::PathBuf");
        assert_eq!(named.name(), "config");
        assert_eq!(named.weaving(), WeavingKind::BorrowedPath);
        assert!(named.is_shareable());
    }

    #[test]
    fn a_positional_argument_names_its_id_and_never_shares_a_slot() {
        let positional = ConsoleArgument::Positional {
            id: "point".to_string(),
            value: value(true, WeavingKind::Cloned, &["crate", "geometry", "Point"]),
        };

        assert_eq!(
            collapsed(&positional.field_type()),
            "crate::geometry::Point"
        );
        assert_eq!(positional.name(), "point");
        assert_eq!(positional.weaving(), WeavingKind::Cloned);
        assert!(!positional.is_shareable());
    }
}
