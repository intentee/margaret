use proc_macro2::TokenStream;
use quote::quote;

use crate::weaving_kind::WeavingKind;

#[must_use]
pub fn owned_weave(weaving: &WeavingKind, source: &TokenStream, is_final_use: bool) -> TokenStream {
    if is_final_use || weaving == &WeavingKind::Copy {
        quote! { #source }
    } else {
        quote! { #source.clone() }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::weaving_kind::WeavingKind;

    use super::owned_weave;

    fn collapsed(weaving: &WeavingKind, is_final_use: bool) -> String {
        owned_weave(weaving, &quote! { serve_input_3 }, is_final_use)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_copy_value_moves_even_when_it_is_not_the_final_use() {
        assert_eq!(collapsed(&WeavingKind::Copy, false), "serve_input_3");
    }

    #[test]
    fn a_copy_value_moves_on_its_final_use() {
        assert_eq!(collapsed(&WeavingKind::Copy, true), "serve_input_3");
    }

    #[test]
    fn a_cloned_value_clones_before_its_final_use() {
        assert_eq!(
            collapsed(&WeavingKind::Cloned, false),
            "serve_input_3.clone()"
        );
    }

    #[test]
    fn a_cloned_value_moves_on_its_final_use() {
        assert_eq!(collapsed(&WeavingKind::Cloned, true), "serve_input_3");
    }

    #[test]
    fn a_borrowed_string_clones_before_its_final_use() {
        assert_eq!(
            collapsed(&WeavingKind::BorrowedStr, false),
            "serve_input_3.clone()"
        );
    }

    #[test]
    fn a_borrowed_path_clones_before_its_final_use() {
        assert_eq!(
            collapsed(&WeavingKind::BorrowedPath, false),
            "serve_input_3.clone()"
        );
    }
}
