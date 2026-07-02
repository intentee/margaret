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
pub fn service(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn scheduled_with_tick_timer(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn runner(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, &["console_argument"])
}

#[proc_macro_attribute]
pub fn responder(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, &["route_parameter"])
}

#[proc_macro_attribute]
pub fn responds_to_http(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn provides_route_parameter(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn handles_middleware_attribute(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn middleware(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn console_command(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn intercepts(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn strip_parameter_markers(item: TokenStream, markers: &[&str]) -> TokenStream {
    strip_or_compile_error(item.into(), markers).into()
}

fn strip_or_compile_error(item: TokenStream2, markers: &[&str]) -> TokenStream2 {
    match strip(item, markers) {
        Ok(stripped) => stripped,
        Err(error) => error.to_compile_error(),
    }
}

fn strip(item: TokenStream2, markers: &[&str]) -> Result<TokenStream2, syn::Error> {
    let mut function: ImplItemFn = syn::parse2(item)?;

    for input in &mut function.sig.inputs {
        if let FnArg::Typed(pattern_type) = input {
            pattern_type.attrs.retain(|attribute| {
                !markers
                    .iter()
                    .any(|marker| attribute.path().is_ident(marker))
            });
        }
    }

    Ok(quote!(#function))
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::strip_or_compile_error;

    #[test]
    fn removes_console_argument_markers_from_parameters() {
        let stripped = strip_or_compile_error(
            quote! {
                pub fn create(
                    greeter: Arc<dyn Greeter>,
                    #[console_argument] name: String,
                    #[console_argument(name = "loud")] loud: bool,
                ) -> Self {
                    Self { greeter, name, loud }
                }
            },
            &["console_argument"],
        )
        .to_string();

        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("greeter"));
        assert!(stripped.contains("name"));
        assert!(stripped.contains("loud"));
    }

    #[test]
    fn removes_several_parameter_markers_at_once() {
        let stripped = strip_or_compile_error(
            quote! {
                pub async fn respond(
                    &self,
                    #[route_parameter] id: String,
                    #[console_argument] name: String,
                ) -> Response {
                    Response::text(200, id)
                }
            },
            &["route_parameter", "console_argument"],
        )
        .to_string();

        assert!(!stripped.contains("route_parameter"));
        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("id"));
        assert!(stripped.contains("name"));
    }

    #[test]
    fn turns_a_parse_error_into_a_compile_error() {
        let output =
            strip_or_compile_error(quote! { struct NotAMethod; }, &["route_parameter"]).to_string();

        assert!(output.contains("compile_error"));
    }
}
