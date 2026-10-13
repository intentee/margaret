use margaret_http::route_addressing::RouteAddressing;

/// # Panics
///
/// Panics when the route parameters do not address the route.
pub fn addressed_route<Route>(addressing: RouteAddressing<Route>) -> Route {
    let RouteAddressing::Addressed(route) = addressing else {
        panic!("the route parameters address the route");
    };

    route
}

#[cfg(test)]
mod tests {
    use margaret_http::route_addressing::RouteAddressing;

    use super::addressed_route;

    #[test]
    #[should_panic(expected = "the route parameters address the route")]
    fn refuses_a_route_its_parameters_do_not_address() {
        addressed_route(RouteAddressing::<()>::Misrouted {
            url: "https://localhost/notes/new".to_string(),
        });
    }
}
