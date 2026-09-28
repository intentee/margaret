use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Attribute;
use syn::FnArg;
use syn::ImplItemFn;
use syn::ItemStruct;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_item_naming_argument::render_item_references::render_item_references;

const REQUEST_BINDING_MARKERS: [&str; 3] =
    ["authenticated_user", "route_parameter", "form_request"];

fn retain_non_marker_attributes(attributes: &mut Vec<Attribute>, markers: &[&str]) {
    attributes.retain(|attribute| {
        !markers
            .iter()
            .any(|marker| attribute.path().is_ident(marker))
    });
}

fn argument_error(error: &AttributeArgumentsError) -> syn::Error {
    syn::Error::new(Span::call_site(), error)
}

fn referencing_named_items(
    attribute_path: &str,
    attributes: TokenStream,
    item: TokenStream,
    item_naming_arguments: &[ItemNamingArgument],
) -> TokenStream {
    referencing_named_items_or_compile_error(
        attribute_path,
        attributes.into(),
        &item.into(),
        item_naming_arguments,
    )
    .into()
}

fn referencing_named_items_or_compile_error(
    attribute_path: &str,
    attributes: TokenStream2,
    item: &TokenStream2,
    item_naming_arguments: &[ItemNamingArgument],
) -> TokenStream2 {
    or_compile_error(
        AttributeArgs::from_argument_tokens(attribute_path.to_string(), attributes)
            .map(|arguments| {
                let references = render_item_references(&arguments, item_naming_arguments);

                quote!(#item #references)
            })
            .map_err(|error| argument_error(&error)),
    )
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

fn strip_struct_markers(item: TokenStream, markers: &[&str]) -> TokenStream {
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
    let mut references = Vec::new();

    for attribute in &item_struct.attrs {
        if attribute.path().is_ident("foreign_key") {
            let arguments =
                AttributeArgs::from_attribute(attribute).map_err(|error| argument_error(&error))?;

            references.push(render_item_references(
                &arguments,
                &[ItemNamingArgument::ForeignKeyReferences],
            ));
        }
    }

    retain_non_marker_attributes(&mut item_struct.attrs, markers);

    for field in &mut item_struct.fields {
        retain_non_marker_attributes(&mut field.attrs, markers);
    }

    Ok(quote!(#item_struct #(#references)*))
}

#[proc_macro_attribute]
pub fn singleton(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn constructor(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(
        item,
        &[
            "console_argument",
            "environment_variable",
            "jwks_secret_store",
            "spiffe_http_client",
        ],
    )
}

#[proc_macro_attribute]
pub fn service(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn scheduled_with_tick_timer(attributes: TokenStream, item: TokenStream) -> TokenStream {
    referencing_named_items(
        "scheduled_with_tick_timer",
        attributes,
        item,
        &[
            ItemNamingArgument::TickInterval,
            ItemNamingArgument::TickBehavior,
        ],
    )
}

#[proc_macro_attribute]
pub fn process(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, &REQUEST_BINDING_MARKERS)
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
    strip_struct_markers(
        item,
        &["column", "foreign_key", "index", "primary_key", "unique"],
    )
}

#[proc_macro_attribute]
pub fn provides_jwks_endpoint(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn provides_route_parameter(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn route_parameter_value(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn infers_authenticated_user(attributes: TokenStream, item: TokenStream) -> TokenStream {
    referencing_named_items(
        "infers_authenticated_user",
        attributes,
        item,
        &[ItemNamingArgument::UserModel],
    )
}

#[proc_macro_attribute]
pub fn infer_from_request(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    strip_parameter_markers(item, &REQUEST_BINDING_MARKERS)
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
    strip_parameter_markers(item, &REQUEST_BINDING_MARKERS)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

    use super::referencing_named_items_or_compile_error;
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
    fn removes_jwks_secret_store_markers_from_parameters() {
        let stripped = strip_or_compile_error(
            quote! {
                pub fn create(
                    #[jwks_secret_store(server)] minter: Arc<JwksSecretStore>,
                    #[jwks_secret_store(client = auth)] verifier: Arc<PublicJwksVerifier>,
                    #[console_argument(from = "label")] label: String,
                ) -> Self {
                    Self { minter, verifier, label }
                }
            },
            &["console_argument", "jwks_secret_store"],
        )
        .to_string();

        assert!(!stripped.contains("jwks_secret_store"));
        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("minter"));
        assert!(stripped.contains("verifier"));
        assert!(stripped.contains("label"));
    }

    #[test]
    fn removes_spiffe_http_client_markers_from_parameters() {
        let stripped = strip_or_compile_error(
            quote! {
                pub fn create(
                    #[spiffe_http_client] reqwest: Client,
                    #[console_argument(from = "label")] label: String,
                ) -> Self {
                    Self { reqwest, label }
                }
            },
            &["console_argument", "spiffe_http_client"],
        )
        .to_string();

        assert!(!stripped.contains("spiffe_http_client"));
        assert!(!stripped.contains("console_argument"));
        assert!(stripped.contains("reqwest"));
        assert!(stripped.contains("label"));
    }

    #[test]
    fn removes_several_parameter_markers_at_once() {
        let stripped = strip_or_compile_error(
            quote! {
                pub async fn respond(
                    &self,
                    #[route_parameter] id: String,
                    #[form_request(from = Form)] form: ValidationResult<Data>,
                ) -> Response {
                    Response::text(200, id)
                }
            },
            &["route_parameter", "form_request"],
        )
        .to_string();

        assert!(!stripped.contains("route_parameter"));
        assert!(!stripped.contains("form_request"));
        assert!(stripped.contains("id"));
        assert!(stripped.contains("form"));
    }

    #[test]
    fn removes_authenticated_user_markers_from_parameters() {
        let stripped = strip_or_compile_error(
            quote! {
                pub async fn respond(
                    &self,
                    #[authenticated_user] author: Author,
                    #[authenticated_user] moderator: Option<Moderator>,
                ) -> Response {
                    Response::text(200, author.name)
                }
            },
            &["authenticated_user", "route_parameter", "form_request"],
        )
        .to_string();

        assert!(!stripped.contains("authenticated_user"));
        assert!(stripped.contains("author"));
        assert!(stripped.contains("moderator"));
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

    #[test]
    fn removes_foreign_key_markers_from_the_struct_itself() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                #[foreign_key(columns = [partition, hash], references = crate::Metadata)]
                #[derive(Clone)]
                pub struct FragmentAssociation {
                    #[column(primary_key)]
                    pub partition: Uuid,
                }
            },
            &["column", "foreign_key", "index"],
        )
        .to_string();

        assert!(!stripped.contains("foreign_key"));
        assert!(!stripped.contains("column"));
        assert!(stripped.contains("derive"));
        assert!(stripped.contains("partition"));
    }

    #[test]
    fn references_the_model_named_by_a_struct_foreign_key() {
        let stripped: String = strip_struct_or_compile_error(
            quote! {
                #[foreign_key(columns = [partition], references = Metadata)]
                pub struct FragmentAssociation {
                    #[column]
                    pub partition: Uuid,
                }
            },
            &["column", "foreign_key"],
        )
        .to_string()
        .split_whitespace()
        .collect();

        assert!(stripped.contains("const_:()={let_:::core::marker::PhantomData<Metadata>"));
    }

    #[test]
    fn turns_malformed_foreign_key_arguments_into_a_compile_error() {
        let output = strip_struct_or_compile_error(
            quote! {
                #[foreign_key(= 5)]
                pub struct FragmentAssociation;
            },
            &["foreign_key"],
        )
        .to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn references_the_items_named_by_attribute_arguments() {
        let output: String = referencing_named_items_or_compile_error(
            "infers_authenticated_user",
            quote!(user_model = Account),
            &quote!(
                struct AccountProvider;
            ),
            &[ItemNamingArgument::UserModel],
        )
        .to_string()
        .split_whitespace()
        .collect();

        assert!(output.starts_with("structAccountProvider;"));
        assert!(output.contains("::core::marker::PhantomData<Account>"));
    }

    #[test]
    fn turns_malformed_attribute_arguments_into_a_compile_error() {
        let output = referencing_named_items_or_compile_error(
            "infers_authenticated_user",
            quote!(= 5),
            &quote!(
                struct AccountProvider;
            ),
            &[ItemNamingArgument::UserModel],
        )
        .to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn removes_primary_key_markers_from_the_struct_itself() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                #[primary_key(columns = [partition, hash])]
                pub struct FragmentAssociation {
                    #[column]
                    pub partition: Uuid,
                }
            },
            &["column", "foreign_key", "index", "primary_key", "unique"],
        )
        .to_string();

        assert!(!stripped.contains("primary_key"));
        assert!(stripped.contains("partition"));
    }

    #[test]
    fn removes_unique_markers_from_the_struct_itself() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                #[unique(columns = [hash, context])]
                pub struct FragmentAssociation {
                    #[column]
                    pub hash: Vec<u8>,
                }
            },
            &["column", "foreign_key", "index", "primary_key", "unique"],
        )
        .to_string();

        assert!(!stripped.contains("unique"));
        assert!(stripped.contains("hash"));
    }

    #[test]
    fn removes_index_markers_from_the_struct_itself() {
        let stripped = strip_struct_or_compile_error(
            quote! {
                #[index(name = "fragment_context", columns = [context, partition])]
                pub struct FragmentAssociation {
                    #[column]
                    pub context: Uuid,
                }
            },
            &["column", "foreign_key", "index", "primary_key", "unique"],
        )
        .to_string();

        assert!(!stripped.contains("index"));
        assert!(stripped.contains("context"));
    }
}
