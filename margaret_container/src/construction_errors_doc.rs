use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn construction_errors_doc() -> TokenStream {
    quote! {
        #[doc = " # Errors"]
        #[doc = ""]
        #[doc = " Returns `ConstructionError` when a singleton constructor fails."]
    }
}

#[cfg(test)]
mod tests {
    use super::construction_errors_doc;

    #[test]
    fn documents_the_construction_error_section() {
        let rendered: String = construction_errors_doc()
            .to_string()
            .split_whitespace()
            .collect();

        assert!(rendered.starts_with(r##"#[doc="#Errors"]"##));
        assert!(rendered.contains("ConstructionError"));
    }
}
