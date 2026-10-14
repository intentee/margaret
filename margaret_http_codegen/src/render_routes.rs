use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::http_route_table::HttpRouteTable;
use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::render_forward_targets::render_forward_targets;
use crate::render_route_props::render_route_props;
use crate::route_props_layout::RoutePropsLayout;
use crate::server_layout::ServerLayout;

fn origin_param_ident(layout: &ServerLayout<'_>) -> Ident {
    format_ident!("origin_{}", layout.server.name())
}

fn server_binding_ident(layout: &ServerLayout<'_>) -> Ident {
    format_ident!("server_{}", layout.field)
}

fn props_type(RoutePropsLayout { type_ident, .. }: &RoutePropsLayout) -> TokenStream {
    quote! { super::super::props::#type_ident }
}

fn addressable_type(props: &RoutePropsLayout, reference: &TokenStream) -> TokenStream {
    let props = props_type(props);

    quote! { margaret::framework::http::addressable_route::AddressableRoute<#props, #reference> }
}

fn get_route_reference() -> TokenStream {
    quote! { margaret::framework::http::get_route_reference::GetRouteReference }
}

fn route_reference() -> TokenStream {
    quote! { margaret::framework::http::route_reference::RouteReference }
}

fn forwardable_type(
    target: &TokenStream,
    props: &RoutePropsLayout,
    locator: &TokenStream,
) -> TokenStream {
    let props = props_type(props);

    quote! {
        margaret::framework::http::forwardable_route::ForwardableRoute<#target, #props, #locator>
    }
}

fn route_field_type(named: &NamedRoute<'_>, target: &TokenStream) -> TokenStream {
    match &named.shape {
        NamedRouteShape::ForwardableLiteral { props, .. } => {
            forwardable_type(target, props, &get_route_reference())
        }
        NamedRouteShape::ForwardableParameterized { props, .. } => forwardable_type(
            target,
            props,
            &addressable_type(props, &get_route_reference()),
        ),
        NamedRouteShape::ReferenceLiteral { .. } => route_reference(),
        NamedRouteShape::ReferenceParameterized { props } => {
            addressable_type(props, &route_reference())
        }
    }
}

fn route_field_init(
    named: &NamedRoute<'_>,
    layout: &ServerLayout<'_>,
    target: &TokenStream,
    shared_origin: &Ident,
) -> TokenStream {
    let ServerLayout { origin, paths, .. } = layout;
    let position = Literal::usize_unsuffixed(named.position);
    let literal_url = |literal: &str| {
        quote! { margaret::framework::http::literal_url::literal_url(#origin, #literal) }
    };
    let addressable = |reference: &TokenStream| {
        quote! {
            #paths.addressable(
                &#shared_origin,
                margaret::framework::http::route_position::RoutePosition::new(#position),
                #reference::new,
            )
        }
    };
    let get_route_reference = get_route_reference();
    let route_reference = route_reference();

    match &named.shape {
        NamedRouteShape::ForwardableLiteral {
            literal, variant, ..
        } => {
            let url = literal_url(literal);

            quote! {
                margaret::framework::http::forwardable_route::ForwardableRoute::new(
                    #get_route_reference::new(#url),
                    #target::#variant,
                )
            }
        }
        NamedRouteShape::ForwardableParameterized { variant, .. } => {
            let locator = addressable(&get_route_reference);

            quote! {
                margaret::framework::http::forwardable_route::ForwardableRoute::new(
                    #locator,
                    #target::#variant,
                )
            }
        }
        NamedRouteShape::ReferenceLiteral { literal } => {
            let url = literal_url(literal);

            quote! { #route_reference::new(#url) }
        }
        NamedRouteShape::ReferenceParameterized { .. } => addressable(&route_reference),
    }
}

fn shares_origin(named: &NamedRoute<'_>) -> bool {
    matches!(
        named.shape,
        NamedRouteShape::ForwardableParameterized { .. }
            | NamedRouteShape::ReferenceParameterized { .. }
    )
}

fn server_struct(
    table: &HttpRouteTable,
    layout: &ServerLayout<'_>,
    websocket_paths: &BTreeMap<String, Vec<String>>,
) -> TokenStream {
    let ServerLayout {
        forwarding,
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
    let target = forwarding.target_type(&quote! { super::super });
    let shared_origin = format_ident!("shared_{origin}");
    let field_decls = named.iter().map(|route| {
        let field = format_ident!("{}", route.name);
        let field_type = route_field_type(route, &target);

        quote! { pub #field: #field_type, }
    });
    let field_inits = named.iter().map(|route| {
        let field = format_ident!("{}", route.name);
        let init = route_field_init(route, layout, &target, &shared_origin);

        quote! { #field: #init, }
    });
    let origin_sharing = named.iter().any(shares_origin).then(|| {
        quote! { let #shared_origin: ::std::sync::Arc<str> = ::std::sync::Arc::from(#origin); }
    });
    let origin_param = if named.is_empty() {
        format_ident!("_origin")
    } else {
        origin.clone()
    };
    let too_many_lines = too_many_lines_allow();

    quote! {
        pub struct #struct_ident {
            pub(crate) #paths: margaret::framework::http::server_paths::ServerPaths<#path_count>,
            #(#field_decls)*
        }

        impl #struct_ident {
            #too_many_lines
            pub(crate) fn new(
                #origin_param: &str,
            ) -> ::std::result::Result<Self, margaret::framework::http::router_error::RouterError> {
                margaret::framework::http::server_paths::ServerPaths::build([
                    #(#server_patterns),*
                ])
                .map(|#paths| {
                    #origin_sharing

                    Self {
                        #(#field_inits)*
                        #paths,
                    }
                })
            }
        }
    }
}

fn built_server(layout: &ServerLayout<'_>) -> TokenStream {
    let field = &layout.field;
    let struct_ident = &layout.struct_ident;
    let param = origin_param_ident(layout);

    quote! { servers::#field::#struct_ident::new(#param) }
}

fn constructed_servers(layouts: &[ServerLayout<'_>]) -> TokenStream {
    let initializers = layouts.iter().map(|layout| {
        let field = &layout.field;
        let server = server_binding_ident(layout);

        quote! { #field: #server }
    });
    let constructed = quote! { Self { #(#initializers),* } };
    let Some((last, preceding)) = layouts.split_last() else {
        return quote! { ::std::result::Result::Ok(#constructed) };
    };
    let last_server = server_binding_ident(last);
    let last_built = built_server(last);

    preceding.iter().rev().fold(
        quote! { #last_built.map(|#last_server| #constructed) },
        |chained, layout| {
            let server = server_binding_ident(layout);
            let built = built_server(layout);

            quote! { #built.and_then(|#server| #chained) }
        },
    )
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
    let server_inits = constructed_servers(layouts);
    let server_declarations = layouts.iter().map(|layout| {
        let field = &layout.field;

        quote! {
            #[rustfmt::skip]
            pub mod #field;
        }
    });
    let routes = ModulePath::root("routes");
    let forward_targets = render_forward_targets(&routes, layouts);
    let props = render_route_props(&routes, layouts);
    let forward_targets_declaration = (!forward_targets.is_empty()).then(|| {
        quote! {
            #[rustfmt::skip]
            pub mod forward_targets;
        }
    });
    let props_declaration = (!props.is_empty()).then(|| {
        quote! {
            #[rustfmt::skip]
            pub mod props;
        }
    });
    let routes_tokens = quote! {
        #forward_targets_declaration
        #props_declaration
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
                #server_inits
            }
        }
    };
    let servers = routes.child("servers");
    let mut modules = vec![
        GeneratedModuleTokens::new(routes, routes_tokens),
        GeneratedModuleTokens::new(servers.clone(), quote! { #(#server_declarations)* }),
    ];

    modules.extend(forward_targets);
    modules.extend(props);

    for layout in layouts {
        modules.push(GeneratedModuleTokens::new(
            servers.child(layout.field.to_string()),
            server_struct(table, layout, websocket_paths),
        ));
    }

    modules
}
