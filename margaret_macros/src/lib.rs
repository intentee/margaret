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
    item
}

#[proc_macro_attribute]
pub fn provider(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn provide(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn service(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn ticker(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn runner(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, "console_argument")
}

#[proc_macro_attribute]
pub fn responder(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, "route_parameter")
}

#[proc_macro_attribute]
pub fn responds_to_http(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn route_parameter_binder(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn crud_gate(_attributes: TokenStream, item: TokenStream) -> TokenStream {
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

fn strip_parameter_markers(item: TokenStream, marker: &str) -> TokenStream {
    match strip(item.into(), marker) {
        Ok(stripped) => stripped.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn strip(item: TokenStream2, marker: &str) -> Result<TokenStream2, syn::Error> {
    let mut function: ImplItemFn = syn::parse2(item)?;

    for input in &mut function.sig.inputs {
        if let FnArg::Typed(pattern_type) = input {
            pattern_type
                .attrs
                .retain(|attribute| !attribute.path().is_ident(marker));
        }
    }

    Ok(quote!(#function))
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::strip;

    #[test]
    fn removes_console_argument_markers_from_parameters() {
        let stripped = strip(
            quote! {
                pub fn create(
                    greeter: Arc<dyn Greeter>,
                    #[console_argument] name: String,
                    #[console_argument(name = "loud")] loud: bool,
                ) -> Self {
                    Self { greeter, name, loud }
                }
            },
            "console_argument",
        )
        .expect("the constructor parses")
        .to_string();

        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("greeter"));
        assert!(stripped.contains("name"));
        assert!(stripped.contains("loud"));
    }

    #[test]
    fn removes_route_parameter_markers_from_parameters() {
        let stripped = strip(
            quote! {
                pub async fn respond(&self, #[route_parameter] id: String) -> Response {
                    Response::text(200, id)
                }
            },
            "route_parameter",
        )
        .expect("the responder parses")
        .to_string();

        assert!(!stripped.contains("route_parameter"));
        assert!(stripped.contains("id"));
    }

    #[test]
    fn reports_a_parse_error_for_a_non_method() {
        assert!(strip(quote! { struct NotAMethod; }, "route_parameter").is_err());
    }
}
