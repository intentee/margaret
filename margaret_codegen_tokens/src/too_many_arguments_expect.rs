use proc_macro2::TokenStream;
use quote::quote;

const CLIPPY_TOO_MANY_ARGUMENTS_THRESHOLD: usize = 7;

#[must_use]
pub fn too_many_arguments_expect(parameter_count: usize) -> TokenStream {
    if parameter_count > CLIPPY_TOO_MANY_ARGUMENTS_THRESHOLD {
        quote! {
            #[expect(
                clippy::too_many_arguments,
                reason = "generated bootstrap weaves one parameter per declared console argument"
            )]
        }
    } else {
        TokenStream::new()
    }
}

#[cfg(test)]
mod tests {
    use super::too_many_arguments_expect;

    #[test]
    fn emits_nothing_at_the_threshold() {
        assert!(too_many_arguments_expect(7).is_empty());
    }

    #[test]
    fn emits_the_expect_attribute_above_the_threshold() {
        let rendered: String = too_many_arguments_expect(8)
            .to_string()
            .split_whitespace()
            .collect();

        assert!(rendered.contains("#[expect(clippy::too_many_arguments"));
    }
}
