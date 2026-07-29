use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_route_parameter_codegen::url_segment::UrlSegment;

use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;

struct ServerLayout<'server> {
    constructor: Ident,
    origin: Ident,
    server: &'server HttpServer,
    struct_ident: Ident,
}

fn server_field_ident(server: &HttpServer) -> Ident {
    format_ident!("{}", server.name())
}

fn origin_param_ident(server: &HttpServer) -> Ident {
    format_ident!("origin_{}", server.name())
}

fn route_field_ident(named: &NamedRoute<'_>) -> Ident {
    format_ident!("{}", named.name)
}

fn placeholders<'route>(named: &NamedRoute<'route>) -> Vec<&'route str> {
    named.path.parameters().collect()
}

fn is_get(named: &NamedRoute<'_>) -> bool {
    named.route.method == "GET"
}

fn server_layouts<'server>(
    table: &HttpRouteTable,
    servers: &'server [HttpServer],
) -> Vec<ServerLayout<'server>> {
    let mut type_allocator = NameAllocator::new();

    servers
        .iter()
        .map(|server| {
            let struct_ident =
                format_ident!("{}", type_allocator.allocate(server.name()).type_name());
            let named = table.named_routes(server.name());

            let mut field_allocator = NameAllocator::new();

            for route in named.iter().filter(|named| placeholders(named).is_empty()) {
                field_allocator.reserve(route.name);
            }

            let origin = format_ident!("{}", field_allocator.allocate("origin").field());

            let mut method_allocator = NameAllocator::new();

            for route in named.iter().filter(|named| !placeholders(named).is_empty()) {
                method_allocator.reserve(route.name);
            }

            let constructor = format_ident!("{}", method_allocator.allocate("new").field());

            ServerLayout {
                constructor,
                origin,
                server,
                struct_ident,
            }
        })
        .collect()
}

fn segments_tokens(named: &NamedRoute<'_>) -> TokenStream {
    let segments = named.path.segments().iter().map(|segment| match segment {
        UrlSegment::Literal(text) => {
            quote! { margaret::framework::http::url_segment::UrlSegment::Literal(#text) }
        }
        UrlSegment::Parameter(name) => {
            let value = format_ident!("{}", name);

            quote! {
                margaret::framework::http::url_segment::UrlSegment::Parameter(
                    margaret::framework::http::url_parameter::UrlParameter {
                        name: #name,
                        value: #value,
                    },
                )
            }
        }
    });

    quote! { ::std::vec::Vec::from([#(#segments),*]) }
}

fn route_type_tokens(named: &NamedRoute<'_>) -> TokenStream {
    if is_get(named) {
        quote! { margaret::framework::http::forwardable_route::ForwardableRoute }
    } else {
        quote! { margaret::framework::http::route_reference::RouteReference }
    }
}

fn route_constructor(named: &NamedRoute<'_>, origin: &TokenStream) -> TokenStream {
    let segments = segments_tokens(named);

    if is_get(named) {
        quote! {
            margaret::framework::http::forwardable_route::ForwardableRoute::new(#origin, #segments)
        }
    } else {
        quote! {
            margaret::framework::http::route_reference::RouteReference::new(#origin, #segments)
        }
    }
}

fn route_method(named: &NamedRoute<'_>, origin: &Ident) -> TokenStream {
    let method = route_field_ident(named);
    let return_type = route_type_tokens(named);
    let placeholders = placeholders(named);
    let parameters = placeholders.iter().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter: String }
    });
    let constructor = route_constructor(named, &quote! { self.#origin.clone() });

    quote! {
        #[must_use]
        pub fn #method(&self, #(#parameters),*) -> #return_type {
            #constructor
        }
    }
}

fn server_struct(table: &HttpRouteTable, layout: &ServerLayout) -> TokenStream {
    let ServerLayout {
        constructor,
        origin,
        server,
        struct_ident,
    } = layout;
    let named = table.named_routes(server.name());
    let paramless: Vec<&NamedRoute<'_>> = named
        .iter()
        .filter(|named| placeholders(named).is_empty())
        .collect();
    let parameterized: Vec<&NamedRoute<'_>> = named
        .iter()
        .filter(|named| !placeholders(named).is_empty())
        .collect();

    let has_named = !named.is_empty();
    let has_parameterized = !parameterized.is_empty();

    let origin_field = has_parameterized.then(|| quote! { #origin: ::std::sync::Arc<str>, });
    let paramless_field_decls = paramless.iter().map(|named| {
        let field = route_field_ident(named);
        let return_type = route_type_tokens(named);

        quote! { pub #field: #return_type, }
    });

    let origin_param = if has_named {
        origin.clone()
    } else {
        format_ident!("_origin")
    };
    let paramless_inits = paramless.iter().map(|named| {
        let field = route_field_ident(named);
        let route_construction = route_constructor(named, &quote! { #origin.clone() });

        quote! { #field: #route_construction, }
    });
    let origin_init = has_parameterized.then(|| quote! { #origin, });
    let methods = parameterized
        .iter()
        .map(|named| route_method(named, origin));

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

pub(crate) fn render_routes(
    table: &HttpRouteTable,
    servers: &[HttpServer],
) -> Vec<GeneratedModuleTokens> {
    let layouts = server_layouts(table, servers);
    let server_fields = layouts.iter().map(|layout| {
        let field = server_field_ident(layout.server);
        let struct_ident = &layout.struct_ident;

        quote! { pub #field: servers::#field::#struct_ident, }
    });
    let origin_params = layouts.iter().map(|layout| {
        let param = origin_param_ident(layout.server);

        quote! { #param: ::std::sync::Arc<str>, }
    });
    let server_inits = layouts.iter().map(|layout| {
        let field = server_field_ident(layout.server);
        let struct_ident = &layout.struct_ident;
        let constructor = &layout.constructor;
        let param = origin_param_ident(layout.server);

        quote! { #field: servers::#field::#struct_ident::#constructor(#param), }
    });
    let server_declarations = layouts.iter().map(|layout| {
        let field = server_field_ident(layout.server);

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
            #[must_use]
            pub fn from_origins(#(#origin_params)*) -> Self {
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
        GeneratedModuleTokens::new("routes", routes_tokens),
        GeneratedModuleTokens::new("routes/servers", servers_tokens),
    ];

    for layout in &layouts {
        modules.push(GeneratedModuleTokens::new(
            format!("routes/servers/{}", server_field_ident(layout.server)),
            server_struct(table, layout),
        ));
    }

    modules
}
