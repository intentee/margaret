use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::request_binding::RequestBinding;
use margaret_route_method::route_method::RouteMethod;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::route_group::RouteGroup;

fn forward_method(named: &NamedRoute<'_>) -> TokenStream {
    let method = format_ident!("{}", named.name);
    let name = named.name;
    let placeholders: Vec<&str> = named.path.parameters().collect();
    let parameters = placeholders.iter().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter: ::std::string::String }
    });
    let path_params = if placeholders.is_empty() {
        quote! { ::std::collections::HashMap::new() }
    } else {
        let entries = placeholders.iter().map(|placeholder| {
            let parameter = format_ident!("{}", placeholder);

            quote! { (#placeholder.to_string(), #parameter) }
        });

        quote! { ::std::collections::HashMap::from([#(#entries),*]) }
    };

    quote! {
        #[must_use]
        pub fn #method(self, #(#parameters),*) -> margaret::framework::http::forward::Forward {
            margaret::framework::http::forward::Forward::new(#name, #path_params)
        }
    }
}

fn injects_forwarder(table: &HttpRouteTable, server: &HttpServer) -> bool {
    table
        .route_groups(server.name())
        .flat_map(RouteGroup::method_routes)
        .flat_map(HttpRoute::arguments)
        .any(|argument| matches!(argument.binding, RequestBinding::Forwarder))
}

fn server_forwarder(table: &HttpRouteTable, server: &HttpServer) -> TokenStream {
    let methods: Vec<TokenStream> = table
        .named_routes(server.name())
        .iter()
        .filter(|named| named.route.method() == RouteMethod::Get)
        .map(forward_method)
        .collect();
    let constructor = injects_forwarder(table, server).then(|| {
        quote! {
            pub(in crate::margaret) fn new() -> Self {
                Self(())
            }
        }
    });

    quote! {
        pub struct Forwarder(());

        impl Forwarder {
            #constructor

            #(#methods)*
        }
    }
}

pub(crate) fn render_forwarders(
    table: &HttpRouteTable,
    servers: &[HttpServer],
) -> Vec<GeneratedModuleTokens> {
    let server_declarations = servers.iter().map(|server| {
        let field = format_ident!("{}", server.name());

        quote! {
            #[rustfmt::skip]
            pub mod #field;
        }
    });

    let mut modules = vec![GeneratedModuleTokens::new(
        "forwarders",
        quote! { #(#server_declarations)* },
    )];

    for server in servers {
        modules.push(GeneratedModuleTokens::new(
            format!("forwarders/{}", server.name()),
            quote! {
                mod forwarder;
                pub use forwarder::Forwarder;
            },
        ));
        modules.push(GeneratedModuleTokens::new(
            format!("forwarders/{}/forwarder", server.name()),
            server_forwarder(table, server),
        ));
    }

    modules
}
