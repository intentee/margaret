use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_url_template::RouteUrlTemplate;
use margaret_route_parameter_codegen::url_segment::UrlSegment;

use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;

fn server_field_ident(server: &HttpServer) -> Ident {
    format_ident!("{}", server.name())
}

fn origin_param_ident(server: &HttpServer) -> Ident {
    format_ident!("origin_{}", server.name())
}

fn route_field_ident(named: &NamedRoute<'_>) -> Ident {
    format_ident!("{}", named.name)
}

fn is_literal(named: &NamedRoute<'_>) -> bool {
    matches!(named.path.template(), RouteUrlTemplate::Literal(_))
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

            for route in named.iter().filter(|named| is_literal(named)) {
                field_allocator.reserve(route.name);
            }

            let origin = format_ident!("{}", field_allocator.allocate("origin").field());

            let mut method_allocator = NameAllocator::new();

            for route in named.iter().filter(|named| !is_literal(named)) {
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

fn segment_tokens(segment: &UrlSegment) -> TokenStream {
    match segment {
        UrlSegment::CatchAllParameter { name, prefix } => {
            let value = format_ident!("{}", name);

            quote! {
                margaret::framework::http::url_segment::UrlSegment::CatchAllParameter {
                    parameter: margaret::framework::http::url_parameter::UrlParameter {
                        name: #name,
                        value: #value,
                    },
                    prefix: #prefix,
                }
            }
        }
        UrlSegment::Literal(text) => {
            quote! { margaret::framework::http::url_segment::UrlSegment::Literal(#text) }
        }
        UrlSegment::Parameter {
            name,
            prefix,
            suffix,
        } => {
            let value = format_ident!("{}", name);

            quote! {
                margaret::framework::http::url_segment::UrlSegment::Parameter {
                    parameter: margaret::framework::http::url_parameter::UrlParameter {
                        name: #name,
                        value: #value,
                    },
                    prefix: #prefix,
                    suffix: #suffix,
                }
            }
        }
    }
}

fn route_type_tokens(named: &NamedRoute<'_>) -> TokenStream {
    if named.route.method() == RouteMethod::Get {
        quote! { margaret::framework::http::forwardable_route::ForwardableRoute }
    } else {
        quote! { margaret::framework::http::route_reference::RouteReference }
    }
}

fn literal_route_field(named: &NamedRoute<'_>) -> TokenStream {
    let field = route_field_ident(named);
    let return_type = route_type_tokens(named);

    quote! { pub #field: #return_type, }
}

fn literal_route_init(named: &NamedRoute<'_>, path: &str, origin: &Ident) -> TokenStream {
    let field = route_field_ident(named);
    let route_type = route_type_tokens(named);

    quote! {
        #field: #route_type::new(margaret::framework::http::literal_url::literal_url(#origin, #path)),
    }
}

fn route_method(named: &NamedRoute<'_>, segments: &[UrlSegment], origin: &Ident) -> TokenStream {
    let method = route_field_ident(named);
    let route_type = route_type_tokens(named);
    let parameters = named.path.parameters().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter: ::std::string::String }
    });
    let segments = segments.iter().map(segment_tokens);

    quote! {
        #[must_use]
        pub fn #method(
            &self,
            #(#parameters),*
        ) -> margaret::framework::http::route_addressing::RouteAddressing<#route_type> {
            margaret::framework::http::build_url::build_url(&self.#origin, &[#(#segments),*])
                .map(#route_type::new)
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
    let mut field_decls = Vec::new();
    let mut field_inits = Vec::new();
    let mut methods = Vec::new();

    for route in &named {
        match route.path.template() {
            RouteUrlTemplate::Literal(path) => {
                field_decls.push(literal_route_field(route));
                field_inits.push(literal_route_init(route, path, origin));
            }
            RouteUrlTemplate::Parameterized(segments) => {
                methods.push(route_method(route, segments, origin));
            }
        }
    }

    let stores_origin = !methods.is_empty();
    let origin_field = stores_origin.then(|| quote! { #origin: ::std::sync::Arc<str>, });
    let origin_init = stores_origin.then(|| quote! { #origin: ::std::sync::Arc::from(#origin), });
    let origin_param = if named.is_empty() {
        format_ident!("_origin")
    } else {
        origin.clone()
    };
    let too_many_lines = too_many_lines_allow();

    quote! {
        pub struct #struct_ident {
            #origin_field
            #(#field_decls)*
        }

        impl #struct_ident {
            #too_many_lines
            pub(crate) fn #constructor(#origin_param: &str) -> Self {
                Self {
                    #(#field_inits)*
                    #origin_init
                }
            }

            #(#methods)*
        }
    }
}

struct ServerLayout<'server> {
    constructor: Ident,
    origin: Ident,
    server: &'server HttpServer,
    struct_ident: Ident,
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

        quote! { #param: &str, }
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
