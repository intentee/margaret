use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::positional_naming::PositionalNaming;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;
use margaret_request_binding_codegen::request_binding::RequestBinding;
use margaret_route_method::route_method::RouteMethod;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::route_group::RouteGroup;
use crate::server_layout::ServerLayout;

fn forward_method(named: &NamedRoute<'_>) -> TokenStream {
    let method = format_ident!("{}", named.name);
    let name = named.name;
    let placeholders: Vec<&str> = named.path.parameters().collect();
    let naming = PositionalNaming::route_parameters(placeholders.len());
    let forwarded: Vec<ForwardedParameter<'_>> = placeholders
        .into_iter()
        .enumerate()
        .map(|(position, placeholder)| ForwardedParameter {
            placeholder,
            value: naming.ident(position),
        })
        .collect();
    let parameters = forwarded.iter().map(|ForwardedParameter { value, .. }| {
        quote! { #value: ::std::string::String }
    });
    let path_params = if forwarded.is_empty() {
        quote! { ::std::collections::HashMap::new() }
    } else {
        let entries = forwarded
            .iter()
            .map(|ForwardedParameter { placeholder, value }| {
                quote! { (#placeholder.to_string(), #value) }
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

fn server_forwarder(layout: &ServerLayout<'_>) -> TokenStream {
    let methods: Vec<TokenStream> = layout
        .named
        .iter()
        .filter(|named| named.route.method() == RouteMethod::Get)
        .map(forward_method)
        .collect();

    quote! {
        #[derive(Default)]
        pub struct Forwarder(());

        impl Forwarder {
            #(#methods)*
        }
    }
}

struct ForwardedParameter<'path> {
    placeholder: &'path str,
    value: Ident,
}

pub(crate) fn render_forwarders(
    table: &HttpRouteTable,
    layouts: &[ServerLayout<'_>],
) -> Vec<GeneratedModuleTokens> {
    let forwarding: Vec<&ServerLayout<'_>> = layouts
        .iter()
        .filter(|layout| injects_forwarder(table, layout.server))
        .collect();

    if forwarding.is_empty() {
        return Vec::new();
    }

    let server_declarations = forwarding.iter().map(|layout| {
        let field = &layout.field;

        quote! {
            #[rustfmt::skip]
            pub mod #field;
        }
    });

    let forwarders = ModulePath::root("forwarders");
    let mut modules = vec![GeneratedModuleTokens::new(
        forwarders.clone(),
        quote! { #(#server_declarations)* },
    )];

    for layout in forwarding {
        let server = forwarders.child(layout.field.to_string());

        modules.push(GeneratedModuleTokens::new(
            server.clone(),
            quote! {
                mod forwarder;
                pub use forwarder::Forwarder;
            },
        ));
        modules.push(GeneratedModuleTokens::new(
            server.child("forwarder"),
            server_forwarder(layout),
        ));
    }

    modules
}
