use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn too_many_lines_allow() -> TokenStream {
    quote! {
        #[allow(
            clippy::too_many_lines,
            reason = "one statement per declared item; the generated list is the unit of meaning"
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::too_many_lines_allow;

    #[test]
    fn allows_the_lint_with_a_reason() {
        let rendered: String = too_many_lines_allow()
            .to_string()
            .split_whitespace()
            .collect();

        assert!(rendered.starts_with("#[allow(clippy::too_many_lines,reason="));
    }
}
