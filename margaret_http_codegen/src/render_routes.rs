use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::Literal;
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
use crate::named_route::NamedRoute;
use crate::server_layout::ServerLayout;

fn origin_param_ident(layout: &ServerLayout<'_>) -> Ident {
    format_ident!("origin_{}", layout.server.name())
}

fn route_field_ident(named: &NamedRoute<'_>) -> Ident {
    format_ident!("{}", named.name)
}

fn segment_tokens(segment: &UrlSegment, values: &mut ParameterValues) -> TokenStream {
    match segment {
        UrlSegment::CatchAllParameter { name, prefix } => {
            let value = values.allocated(name);

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
        UrlSegment::Literal { decoded, encoded } => {
            quote! {
                margaret::framework::http::url_segment::UrlSegment::Literal {
                    decoded: #decoded,
                    encoded: #encoded,
                }
            }
        }
        UrlSegment::Parameter { name, prefix } => {
            let value = values.allocated(name);

            quote! {
                margaret::framework::http::url_segment::UrlSegment::Parameter {
                    parameter: margaret::framework::http::url_parameter::UrlParameter {
                        name: #name,
                        value: #value,
                    },
                    prefix: #prefix,
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

fn route_method(
    named: &NamedRoute<'_>,
    segments: &[UrlSegment],
    layout: &ServerLayout<'_>,
) -> TokenStream {
    let ServerLayout { origin, paths, .. } = layout;
    let method = route_field_ident(named);
    let route_type = route_type_tokens(named);
    let position = Literal::usize_unsuffixed(named.position);
    let mut values = ParameterValues {
        allocator: NameAllocator::new(),
        idents: Vec::new(),
    };
    let segments: Vec<TokenStream> = segments
        .iter()
        .map(|segment| segment_tokens(segment, &mut values))
        .collect();
    let parameters = values
        .idents
        .iter()
        .map(|parameter| quote! { #parameter: ::std::string::String });

    quote! {
        #[must_use]
        pub fn #method(
            &self,
            #(#parameters),*
        ) -> margaret::framework::http::route_addressing::RouteAddressing<#route_type> {
            margaret::framework::http::build_url::build_url(
                &self.#origin,
                &self.#paths,
                margaret::framework::http::route_position::RoutePosition::new(#position),
                &[#(#segments),*],
            )
            .map(#route_type::new)
        }
    }
}

fn server_struct(
    table: &HttpRouteTable,
    layout: &ServerLayout<'_>,
    websocket_paths: &BTreeMap<String, Vec<String>>,
) -> TokenStream {
    let ServerLayout {
        constructor,
        named,
        origin,
        paths,
        server,
        struct_ident,
        ..
    } = layout;
    let server_patterns: Vec<&str> = table
        .route_groups(server.name())
        .map(|group| group.path().pattern())
        .chain(
            websocket_paths
                .get(server.name())
                .into_iter()
                .flatten()
                .map(String::as_str),
        )
        .collect();
    let path_count = Literal::usize_unsuffixed(server_patterns.len());
    let mut field_decls = Vec::new();
    let mut field_inits = Vec::new();
    let mut methods = Vec::new();

    for route in named {
        match route.path.template() {
            RouteUrlTemplate::Literal(path) => {
                field_decls.push(literal_route_field(route));
                field_inits.push(literal_route_init(route, path, origin));
            }
            RouteUrlTemplate::Parameterized(segments) => {
                methods.push(route_method(route, segments, layout));
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
            pub(crate) #paths: margaret::framework::http::server_paths::ServerPaths<#path_count>,
            #(#field_decls)*
        }

        impl #struct_ident {
            #too_many_lines
            pub(crate) fn #constructor(
                #origin_param: &str,
            ) -> ::std::result::Result<Self, margaret::framework::http::router_error::RouterError> {
                ::std::result::Result::Ok(Self {
                    #paths: margaret::framework::http::server_paths::ServerPaths::build([
                        #(#server_patterns),*
                    ])?,
                    #(#field_inits)*
                    #origin_init
                })
            }

            #(#methods)*
        }
    }
}

struct ParameterValues {
    allocator: NameAllocator,
    idents: Vec<Ident>,
}

impl ParameterValues {
    fn allocated(&mut self, name: &str) -> Ident {
        let value = format_ident!("{}", self.allocator.allocate(name).field());

        self.idents.push(value.clone());

        value
    }
}

pub(crate) fn render_routes(
    table: &HttpRouteTable,
    layouts: &[ServerLayout<'_>],
    websocket_paths: &BTreeMap<String, Vec<String>>,
) -> Vec<GeneratedModuleTokens> {
    let server_fields = layouts.iter().map(|layout| {
        let field = &layout.field;
        let struct_ident = &layout.struct_ident;

        quote! { pub #field: servers::#field::#struct_ident, }
    });
    let origin_params = layouts.iter().map(|layout| {
        let param = origin_param_ident(layout);

        quote! { #param: &str, }
    });
    let server_inits = layouts.iter().map(|layout| {
        let field = &layout.field;
        let struct_ident = &layout.struct_ident;
        let constructor = &layout.constructor;
        let param = origin_param_ident(layout);

        quote! { #field: servers::#field::#struct_ident::#constructor(#param)?, }
    });
    let server_declarations = layouts.iter().map(|layout| {
        let field = &layout.field;

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
            #[doc = " # Errors"]
            #[doc = ""]
            #[doc = " Returns `RouterError` when the paths of a server cannot build its router."]
            pub fn from_origins(
                #(#origin_params)*
            ) -> ::std::result::Result<Self, margaret::framework::http::router_error::RouterError> {
                ::std::result::Result::Ok(Self {
                    #(#server_inits)*
                })
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

    for layout in layouts {
        modules.push(GeneratedModuleTokens::new(
            format!("routes/servers/{}", layout.field),
            server_struct(table, layout, websocket_paths),
        ));
    }

    modules
}
