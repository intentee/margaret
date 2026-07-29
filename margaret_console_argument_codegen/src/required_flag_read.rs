use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn required_flag_read(
    value_type: &TokenStream,
    id: &str,
    present: &TokenStream,
    missing: &TokenStream,
) -> TokenStream {
    quote! {
        match matches.get_one::<#value_type>(#id) {
            Some(value) => #present,
            None => #missing,
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::required_flag_read;

    #[test]
    fn returns_the_missing_outcome_the_caller_supplies() {
        let rendered: String = required_flag_read(
            &quote! { String },
            "label",
            &quote! { value.clone() },
            &quote! { return ::std::result::Result::Err(outcome) },
        )
        .to_string()
        .split_whitespace()
        .collect();

        assert!(rendered.contains("None=>return::std::result::Result::Err(outcome)"));
    }
}
