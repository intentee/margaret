use std::num::NonZeroU64;

use margaret_attributes::identifier::Identifier;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_route_method::route_method::RouteMethod;
use margaret_route_parameter_codegen::route_path::RoutePath;

pub struct DeclaredRoute<'index> {
    pub identifier: &'index Identifier,
    pub item: &'index IndexedItem,
    pub max_body_bytes: Option<NonZeroU64>,
    pub method: RouteMethod,
    pub name: Option<String>,
    pub path: RoutePath,
    pub server: String,
}
