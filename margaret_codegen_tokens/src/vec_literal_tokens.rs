use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;

pub fn vec_literal_tokens<Elements>(elements: Elements) -> TokenStream
where
    Elements: IntoIterator,
    Elements::Item: ToTokens,
{
    let elements: Vec<Elements::Item> = elements.into_iter().collect();

    if elements.is_empty() {
        quote! { ::std::vec::Vec::new() }
    } else {
        quote! { ::std::vec::Vec::from([#(#elements),*]) }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::vec_literal_tokens;

    fn collapsed(tokens: &TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    #[test]
    fn builds_an_empty_vec_without_an_array() {
        assert_eq!(
            collapsed(&vec_literal_tokens(Vec::<TokenStream>::new())),
            "::std::vec::Vec::new()"
        );
    }

    #[test]
    fn builds_a_vec_from_an_array_of_elements() {
        assert_eq!(
            collapsed(&vec_literal_tokens(vec![
                quote! { first },
                quote! { second }
            ])),
            "::std::vec::Vec::from([first,second])"
        );
    }
}
