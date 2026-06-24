use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::FnArg;
use syn::ImplItemFn;

#[proc_macro_attribute]
pub fn singleton(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn constructor(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    match strip_console_arguments(item.into()) {
        Ok(stripped) => stripped.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn responds_to_http(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn http_middleware(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn console_command(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn strip_console_arguments(item: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let mut function: ImplItemFn = syn::parse2(item)?;

    for input in &mut function.sig.inputs {
        if let FnArg::Typed(pattern_type) = input {
            pattern_type
                .attrs
                .retain(|attribute| !attribute.path().is_ident("console_argument"));
        }
    }

    Ok(quote!(#function))
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::strip_console_arguments;

    #[test]
    fn removes_console_argument_markers_from_parameters() {
        let stripped = strip_console_arguments(quote! {
            pub fn create(
                greeter: Arc<dyn Greeter>,
                #[console_argument(name = "name", required = true)] name: String,
            ) -> Self {
                Self { greeter, name }
            }
        })
        .expect("the constructor parses")
        .to_string();

        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("greeter"));
        assert!(stripped.contains("name"));
    }

    #[test]
    fn reports_a_parse_error_for_a_non_method() {
        assert!(strip_console_arguments(quote! { struct NotAMethod; }).is_err());
    }
}
