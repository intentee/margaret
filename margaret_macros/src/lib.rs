use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Attribute;
use syn::FnArg;
use syn::ImplItemFn;
use syn::ItemStruct;

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
pub fn process(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(
        item,
        &["route_parameter", "form_request", "console_argument"],
    )
}

#[proc_macro_attribute]
pub fn responds_to_http(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn renders_view(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn model(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_field_markers(item, &["column", "foreign_key", "index"])
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
pub fn websocket_session(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn websocket_message(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn build_for_session(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, &["route_parameter", "form_request"])
}

fn retain_non_marker_attributes(attributes: &mut Vec<Attribute>, markers: &[&str]) {
    attributes.retain(|attribute| {
        !markers
            .iter()
            .any(|marker| attribute.path().is_ident(marker))
    });
}

fn or_compile_error(result: Result<TokenStream2, syn::Error>) -> TokenStream2 {
    match result {
        Ok(stripped) => stripped,
        Err(error) => error.to_compile_error(),
    }
}

fn strip_parameter_markers(item: TokenStream, markers: &[&str]) -> TokenStream {
    strip_or_compile_error(item.into(), markers).into()
}

fn strip_field_markers(item: TokenStream, markers: &[&str]) -> TokenStream {
    strip_struct_or_compile_error(item.into(), markers).into()
}

fn strip_or_compile_error(item: TokenStream2, markers: &[&str]) -> TokenStream2 {
    or_compile_error(strip(item, markers))
}

fn strip_struct_or_compile_error(item: TokenStream2, markers: &[&str]) -> TokenStream2 {
    or_compile_error(strip_struct(item, markers))
}

fn strip(item: TokenStream2, markers: &[&str]) -> Result<TokenStream2, syn::Error> {
    let mut function: ImplItemFn = syn::parse2(item)?;

    for input in &mut function.sig.inputs {
        if let FnArg::Typed(pattern_type) = input {
            retain_non_marker_attributes(&mut pattern_type.attrs, markers);
        }
    }

    Ok(quote!(#function))
}

fn strip_struct(item: TokenStream2, markers: &[&str]) -> Result<TokenStream2, syn::Error> {
    let mut item_struct: ItemStruct = syn::parse2(item)?;

    for field in &mut item_struct.fields {
        retain_non_marker_attributes(&mut field.attrs, markers);
    }

    Ok(quote!(#item_struct))
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::strip_or_compile_error;
    use super::strip_struct_or_compile_error;

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
                    #[form_request(from = Form)] form: ValidationResult<Data>,
                    #[console_argument] name: String,
                ) -> Response {
                    Response::text(200, id)
                }
            },
            &["route_parameter", "form_request", "console_argument"],
        )
        .to_string();

        assert!(!stripped.contains("route_parameter"));
        assert!(!stripped.contains("form_request"));
        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("id"));
        assert!(stripped.contains("form"));
        assert!(stripped.contains("name"));
    }

    #[test]
    fn turns_a_parse_error_into_a_compile_error() {
        let output =
            strip_or_compile_error(quote! { struct NotAMethod; }, &["route_parameter"]).to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn removes_column_markers_from_struct_fields() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                struct Article {
                    #[column(primary_key, name = "id")]
                    id: Uuid,
                    #[column]
                    title: String,
                }
            },
            &["column"],
        )
        .to_string();

        assert!(!stripped.contains("column"));
        assert!(stripped.contains("id"));
        assert!(stripped.contains("title"));
    }

    #[test]
    fn removes_foreign_key_markers_from_struct_fields() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                struct Article {
                    #[column(primary_key)]
                    id: Uuid,
                    #[column]
                    #[foreign_key]
                    author: Author,
                }
            },
            &["column", "foreign_key"],
        )
        .to_string();

        assert!(!stripped.contains("column"));
        assert!(!stripped.contains("foreign_key"));
        assert!(stripped.contains("author"));
        assert!(stripped.contains("Author"));
    }

    #[test]
    fn removes_index_markers_from_struct_fields() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                struct Article {
                    #[column(primary_key)]
                    id: Uuid,
                    #[column]
                    #[index]
                    created_at: DateTime<Utc>,
                }
            },
            &["column", "foreign_key", "index"],
        )
        .to_string();

        assert!(!stripped.contains("column"));
        assert!(!stripped.contains("index"));
        assert!(stripped.contains("created_at"));
    }

    #[test]
    fn turns_a_struct_parse_error_into_a_compile_error() {
        let output =
            strip_struct_or_compile_error(quote! { fn not_a_struct() {} }, &["column"]).to_string();

        assert!(output.contains("compile_error"));
    }
}
