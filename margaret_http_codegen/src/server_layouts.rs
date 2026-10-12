use quote::format_ident;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_route_parameter_codegen::route_url_template::RouteUrlTemplate;

use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::server_layout::ServerLayout;

fn is_literal(named: &NamedRoute<'_>) -> bool {
    matches!(named.path.template(), RouteUrlTemplate::Literal(_))
}

pub(crate) fn server_layouts<'server>(
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
            let paths = format_ident!("{}", field_allocator.allocate("paths").field());
            let mut method_allocator = NameAllocator::new();

            for route in named.iter().filter(|named| !is_literal(named)) {
                method_allocator.reserve(route.name);
            }

            let constructor = format_ident!("{}", method_allocator.allocate("new").field());

            ServerLayout {
                constructor,
                field: format_ident!("{}", server.name()),
                origin,
                paths,
                server,
                struct_ident,
            }
        })
        .collect()
}
