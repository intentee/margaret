use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::console_argument_clone::console_argument_clone;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;

use crate::console_argument::ConsoleArgument;
use crate::weaving_kind::WeavingKind;

#[must_use]
pub fn owned_weave(argument: &ConsoleArgument, slot: usize, is_final_use: bool) -> TokenStream {
    if is_final_use || argument.weaving() == WeavingKind::Copy {
        let ident = console_argument_ident(slot);

        quote! { #ident }
    } else {
        console_argument_clone(slot)
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::console_argument::ConsoleArgument;
    use crate::weaving_kind::WeavingKind;

    use super::owned_weave;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn collapsed(argument: &ConsoleArgument, slot: usize, is_final_use: bool) -> String {
        owned_weave(argument, slot, is_final_use)
            .to_string()
            .split_whitespace()
            .collect()
    }

    fn cloned_string() -> ConsoleArgument {
        ConsoleArgument::Named {
            name: "note".to_string(),
            required: false,
            weaving: WeavingKind::Cloned,
            value_type: path(&["std", "string", "String"]),
            relations: Vec::new(),
        }
    }

    #[test]
    fn a_copy_argument_moves_even_when_not_the_final_use() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(&flag, 3, false), "console_argument_3");
    }

    #[test]
    fn a_copy_argument_moves_on_its_final_use() {
        let flag = ConsoleArgument::Flag {
            name: "loud".to_string(),
        };

        assert_eq!(collapsed(&flag, 3, true), "console_argument_3");
    }

    #[test]
    fn a_non_copy_argument_clones_before_its_final_use() {
        assert_eq!(
            collapsed(&cloned_string(), 3, false),
            "console_argument_3.clone()"
        );
    }

    #[test]
    fn a_non_copy_argument_moves_on_its_final_use() {
        assert_eq!(collapsed(&cloned_string(), 3, true), "console_argument_3");
    }

    #[test]
    fn a_borrowed_argument_clones_before_its_final_use_from_an_owned_source() {
        let named = ConsoleArgument::Named {
            name: "label".to_string(),
            required: true,
            weaving: WeavingKind::BorrowedStr,
            value_type: path(&["std", "string", "String"]),
            relations: Vec::new(),
        };

        assert_eq!(collapsed(&named, 2, false), "console_argument_2.clone()");
    }
}
