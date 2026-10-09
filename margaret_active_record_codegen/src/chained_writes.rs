use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn chained_writes(writes: &[TokenStream]) -> TokenStream {
    writes.iter().fold(
        quote! { ::std::result::Result::Ok(()) },
        |chained, write| quote! { #chained.and_then(|()| #write) },
    )
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::chained_writes;

    #[test]
    fn writes_each_field_after_the_previous_one() {
        assert_eq!(
            chained_writes(&[quote! { first(parameters) }, quote! { second(parameters) }])
                .to_string(),
            quote! {
                ::std::result::Result::Ok(())
                    .and_then(|()| first(parameters))
                    .and_then(|()| second(parameters))
            }
            .to_string()
        );
    }
}
