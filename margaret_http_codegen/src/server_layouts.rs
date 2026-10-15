use proc_macro2::Ident;
use quote::format_ident;

use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;
use margaret_route_parameter_codegen::route_url_template::RouteUrlTemplate;
use margaret_route_parameter_codegen::url_segment::UrlSegment;

use crate::forward_targets_layout::ForwardTargetsLayout;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::props_prefix::PropsPrefix;
use crate::props_segment::PropsSegment;
use crate::route_props_layout::RoutePropsLayout;
use crate::server_forwarding::ServerForwarding;
use crate::server_layout::ServerLayout;

fn placeholder_fields(segments: &[UrlSegment]) -> NameAllocator {
    let mut allocator = NameAllocator::new();

    for segment in segments {
        match segment {
            UrlSegment::CatchAllParameter { name, .. } | UrlSegment::Parameter { name, .. } => {
                allocator.reserve(name);
            }
            UrlSegment::Literal { .. } => {}
        }
    }

    allocator
}

fn field_of(name: &str, allocator: &mut NameAllocator) -> Ident {
    if is_snake_case_identifier(name) {
        format_ident!("{name}")
    } else {
        format_ident!("{}", allocator.allocate(name).field())
    }
}

fn props_prefix(
    route: &str,
    placeholder: &str,
    prefix: &str,
    props_allocator: &mut NameAllocator,
) -> PropsPrefix {
    if prefix.is_empty() {
        return PropsPrefix::Unprefixed;
    }

    let identifier = props_allocator.allocate(&format!("{route}_{placeholder}_prefix"));

    PropsPrefix::Prefixed {
        module: format_ident!("{}", identifier.field()),
        text: prefix.to_string(),
        type_ident: format_ident!("{}", identifier.type_name()),
    }
}

fn props_segments(
    route: &str,
    path: &RoutePath,
    props_allocator: &mut NameAllocator,
) -> Vec<PropsSegment> {
    let RouteUrlTemplate::Parameterized(segments) = path.template() else {
        return Vec::new();
    };
    let mut allocator = placeholder_fields(segments);

    segments
        .iter()
        .map(|segment| match segment {
            UrlSegment::CatchAllParameter { name, prefix } => PropsSegment::CatchAllParameter {
                field: field_of(name, &mut allocator),
                name: name.clone(),
                prefix: props_prefix(route, name, prefix, props_allocator),
            },
            UrlSegment::Literal { decoded, encoded } => PropsSegment::Literal {
                decoded: decoded.clone(),
                encoded: encoded.clone(),
            },
            UrlSegment::Parameter { name, prefix } => PropsSegment::Parameter {
                field: field_of(name, &mut allocator),
                name: name.clone(),
                prefix: props_prefix(route, name, prefix, props_allocator),
            },
        })
        .collect()
}

fn route_props(
    name: &str,
    path: &RoutePath,
    props_allocator: &mut NameAllocator,
) -> RoutePropsLayout {
    let identifier = props_allocator.allocate(&format!("{name}_route_props"));

    RoutePropsLayout {
        module: format_ident!("{}", identifier.field()),
        segments: props_segments(name, path, props_allocator),
        type_ident: format_ident!("{}", identifier.type_name()),
    }
}

fn named_route_shape(
    DeclaredNamedRoute {
        name, path, route, ..
    }: &DeclaredNamedRoute<'_>,
    props_allocator: &mut NameAllocator,
    variant_allocator: &mut NameAllocator,
) -> NamedRouteShape {
    match path.template() {
        RouteUrlTemplate::Literal(literal) if route.method() == RouteMethod::Get => {
            NamedRouteShape::ForwardableLiteral {
                field: format_ident!("{name}"),
                literal: literal.clone(),
                props: route_props(name, path, props_allocator),
                variant: format_ident!("{}", variant_allocator.allocate(name).type_name()),
            }
        }
        RouteUrlTemplate::Literal(literal) => NamedRouteShape::ReferenceLiteral {
            literal: literal.clone(),
        },
        RouteUrlTemplate::Parameterized(_) if route.method() == RouteMethod::Get => {
            NamedRouteShape::ForwardableParameterized {
                field: format_ident!("{name}"),
                props: route_props(name, path, props_allocator),
                variant: format_ident!("{}", variant_allocator.allocate(name).type_name()),
            }
        }
        RouteUrlTemplate::Parameterized(_) => NamedRouteShape::ReferenceParameterized {
            props: route_props(name, path, props_allocator),
        },
    }
}

fn named_routes<'plan>(
    table: &'plan HttpRouteTable,
    server: &HttpServer,
    props_allocator: &mut NameAllocator,
) -> Vec<NamedRoute<'plan>> {
    let mut variant_allocator = NameAllocator::new();
    let mut declared: Vec<DeclaredNamedRoute<'plan>> = table
        .route_groups(server.name())
        .enumerate()
        .flat_map(|(position, group)| {
            group.method_routes().filter_map(move |route| {
                route.name.as_deref().map(|name| DeclaredNamedRoute {
                    name,
                    path: group.path(),
                    position,
                    route,
                })
            })
        })
        .collect();

    declared.sort_by(|first, second| first.name.cmp(second.name));

    declared
        .into_iter()
        .map(|declared| NamedRoute {
            name: declared.name,
            position: declared.position,
            route: declared.route,
            shape: named_route_shape(&declared, props_allocator, &mut variant_allocator),
        })
        .collect()
}

fn server_forwarding(
    named: &[NamedRoute<'_>],
    server: &HttpServer,
    forwarding_allocator: &mut NameAllocator,
) -> ServerForwarding {
    let forwards = named.iter().any(|route| {
        matches!(
            route.shape,
            NamedRouteShape::ForwardableLiteral { .. }
                | NamedRouteShape::ForwardableParameterized { .. }
        )
    });

    if !forwards {
        return ServerForwarding::Unforwarded;
    }

    let target = forwarding_allocator.allocate(&format!("{}_forward_target", server.name()));
    let targets = forwarding_allocator.allocate(&format!("{}_forward_targets", server.name()));

    ServerForwarding::Targets(ForwardTargetsLayout {
        target_module: format_ident!("{}", target.field()),
        target_type: format_ident!("{}", target.type_name()),
        targets_module: format_ident!("{}", targets.field()),
        targets_type: format_ident!("{}", targets.type_name()),
    })
}

struct DeclaredNamedRoute<'plan> {
    name: &'plan str,
    path: &'plan RoutePath,
    position: usize,
    route: &'plan HttpRoute,
}

pub(crate) fn server_layouts<'plan>(
    table: &'plan HttpRouteTable,
    servers: &'plan [HttpServer],
) -> Vec<ServerLayout<'plan>> {
    let mut type_allocator = NameAllocator::new();
    let mut props_allocator = NameAllocator::new();
    let mut forwarding_allocator = NameAllocator::new();

    servers
        .iter()
        .map(|server| {
            let struct_ident =
                format_ident!("{}", type_allocator.allocate(server.name()).type_name());
            let named = named_routes(table, server, &mut props_allocator);
            let mut field_allocator = NameAllocator::new();

            for route in &named {
                field_allocator.reserve(route.name);
            }

            let paths = format_ident!("{}", field_allocator.allocate("paths").field());
            let origin = format_ident!("{}", field_allocator.allocate("origin").field());

            ServerLayout {
                field: format_ident!("{}", server.name()),
                forwarding: server_forwarding(&named, server, &mut forwarding_allocator),
                named,
                origin,
                paths,
                server,
                struct_ident,
            }
        })
        .collect()
}
