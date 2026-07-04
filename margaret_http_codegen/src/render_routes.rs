use std::collections::HashMap;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_generated_module::generated_module::GeneratedModule;

use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::path_parameter_names::path_parameter_names;
use crate::render::unparse;
use crate::route_url_template::route_url_template;
use crate::url_segment::UrlSegment;

struct ServerLayout {
    constructor: Ident,
    origin: Ident,
    struct_ident: Ident,
}

fn route_name(route: &HttpRoute) -> &str {
    route
        .name
        .as_deref()
        .expect("only named routes reach the routes module")
}

fn server_field_ident(server: &HttpServer) -> Ident {
    format_ident!("{}", server.name())
}

fn origin_param_ident(server: &HttpServer) -> Ident {
    format_ident!("origin_{}", server.name())
}

fn route_field_ident(route: &HttpRoute) -> Ident {
    format_ident!("{}", route_name(route))
}

fn placeholders(route: &HttpRoute) -> Vec<String> {
    path_parameter_names(&route.path).expect("the route path was validated during http codegen")
}

fn is_get(route: &HttpRoute) -> bool {
    route.method == "Get"
}

fn named_routes_for<'route>(
    routes: &'route [HttpRoute],
    server: &HttpServer,
) -> Vec<&'route HttpRoute> {
    let mut named: Vec<&HttpRoute> = routes
        .iter()
        .filter(|route| route.server == server.name() && route.name.is_some())
        .collect();

    named.sort_by(|first, second| route_name(first).cmp(route_name(second)));

    named
}

fn server_layouts(routes: &[HttpRoute], servers: &[HttpServer]) -> HashMap<String, ServerLayout> {
    let mut type_allocator = NameAllocator::new();

    servers
        .iter()
        .map(|server| {
            let struct_ident =
                format_ident!("{}", type_allocator.allocate(server.name()).type_name());
            let named = named_routes_for(routes, server);

            let mut field_allocator = NameAllocator::new();

            for route in named.iter().filter(|route| placeholders(route).is_empty()) {
                field_allocator.reserve(route_name(route));
            }

            let origin = format_ident!("{}", field_allocator.allocate("origin").field());

            let mut method_allocator = NameAllocator::new();

            for route in named.iter().filter(|route| !placeholders(route).is_empty()) {
                method_allocator.reserve(route_name(route));
            }

            let constructor = format_ident!("{}", method_allocator.allocate("new").field());

            (
                server.name().to_string(),
                ServerLayout {
                    constructor,
                    origin,
                    struct_ident,
                },
            )
        })
        .collect()
}

fn segments_tokens(route: &HttpRoute) -> TokenStream {
    let segments = route_url_template(&route.path)
        .into_iter()
        .map(|segment| match segment {
            UrlSegment::Literal(text) => {
                quote! { margaret_http::url_segment::UrlSegment::Literal(#text) }
            }
            UrlSegment::Parameter(name) => {
                quote! { margaret_http::url_segment::UrlSegment::Parameter(#name) }
            }
        });

    quote! { &[#(#segments),*] }
}

fn route_type_tokens(route: &HttpRoute) -> TokenStream {
    if is_get(route) {
        quote! { margaret_http::forwardable_route::ForwardableRoute }
    } else {
        quote! { margaret_http::url_reference::UrlReference }
    }
}

fn route_constructor(route: &HttpRoute, origin: TokenStream, values: TokenStream) -> TokenStream {
    let segments = segments_tokens(route);

    if is_get(route) {
        let name = route_name(route);

        quote! {
            margaret_http::forwardable_route::ForwardableRoute::new(#name, #origin, #segments, #values)
        }
    } else {
        quote! {
            margaret_http::url_reference::UrlReference::new(#origin, #segments, #values)
        }
    }
}

fn route_method(route: &HttpRoute, origin: &Ident) -> TokenStream {
    let method = route_field_ident(route);
    let return_type = route_type_tokens(route);
    let placeholders = placeholders(route);
    let parameters = placeholders.iter().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter: String }
    });
    let values = placeholders.iter().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter }
    });
    let constructor = route_constructor(
        route,
        quote! { self.#origin.clone() },
        quote! { ::std::vec![#(#values),*] },
    );

    quote! {
        pub fn #method(&self, #(#parameters),*) -> #return_type {
            #constructor
        }
    }
}

fn server_struct(routes: &[HttpRoute], server: &HttpServer, layout: &ServerLayout) -> TokenStream {
    let ServerLayout {
        constructor,
        origin,
        struct_ident,
    } = layout;
    let named = named_routes_for(routes, server);
    let paramless: Vec<&HttpRoute> = named
        .iter()
        .filter(|route| placeholders(route).is_empty())
        .copied()
        .collect();
    let parameterized: Vec<&HttpRoute> = named
        .iter()
        .filter(|route| !placeholders(route).is_empty())
        .copied()
        .collect();

    let has_named = !named.is_empty();
    let has_parameterized = !parameterized.is_empty();

    let origin_field = has_parameterized.then(|| quote! { #origin: ::std::sync::Arc<str>, });
    let paramless_field_decls = paramless.iter().map(|route| {
        let field = route_field_ident(route);
        let return_type = route_type_tokens(route);

        quote! { pub #field: #return_type, }
    });

    let origin_param = if has_named {
        origin.clone()
    } else {
        format_ident!("_origin")
    };
    let paramless_inits = paramless.iter().map(|route| {
        let field = route_field_ident(route);
        let route_construction = route_constructor(
            route,
            quote! { #origin.clone() },
            quote! { ::std::vec::Vec::new() },
        );

        quote! { #field: #route_construction, }
    });
    let origin_init = has_parameterized.then(|| quote! { #origin, });
    let methods = parameterized
        .iter()
        .map(|route| route_method(route, origin));

    quote! {
        pub struct #struct_ident {
            #origin_field
            #(#paramless_field_decls)*
        }

        impl #struct_ident {
            pub(crate) fn #constructor(#origin_param: ::std::sync::Arc<str>) -> Self {
                Self {
                    #(#paramless_inits)*
                    #origin_init
                }
            }

            #(#methods)*
        }
    }
}

pub(crate) fn render_routes(routes: &[HttpRoute], servers: &[HttpServer]) -> Vec<GeneratedModule> {
    let layouts = server_layouts(routes, servers);
    let server_fields = servers.iter().map(|server| {
        let field = server_field_ident(server);
        let struct_ident = &layouts[server.name()].struct_ident;

        quote! { pub #field: servers::#field::#struct_ident, }
    });
    let origin_params = servers.iter().map(|server| {
        let param = origin_param_ident(server);

        quote! { #param: ::std::sync::Arc<str>, }
    });
    let server_inits = servers.iter().map(|server| {
        let field = server_field_ident(server);
        let layout = &layouts[server.name()];
        let struct_ident = &layout.struct_ident;
        let constructor = &layout.constructor;
        let param = origin_param_ident(server);

        quote! { #field: servers::#field::#struct_ident::#constructor(#param), }
    });
    let server_declarations = servers.iter().map(|server| {
        let field = server_field_ident(server);

        quote! {
            #[rustfmt::skip]
            pub mod #field;
        }
    });

    let routes_tokens = quote! {
        #[rustfmt::skip]
        pub mod servers;

        pub struct Routes {
            #(#server_fields)*
        }

        impl Routes {
            pub(crate) fn from_origins(#(#origin_params)*) -> Self {
                Self {
                    #(#server_inits)*
                }
            }
        }
    };
    let servers_tokens = quote! {
        #(#server_declarations)*
    };

    let mut modules = vec![
        GeneratedModule::new("routes", unparse(routes_tokens)),
        GeneratedModule::new("routes/servers", unparse(servers_tokens)),
    ];

    for server in servers {
        modules.push(GeneratedModule::new(
            format!("routes/servers/{}", server_field_ident(server)),
            unparse(server_struct(routes, server, &layouts[server.name()])),
        ));
    }

    modules
}
