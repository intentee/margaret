use margaret_http::route_addressing::RouteAddressing;

pub fn addressed_route<Route>(addressing: RouteAddressing<Route>) -> Route {
    let RouteAddressing::Addressed(route) = addressing else {
        panic!("the route parameters address the route");
    };

    route
}
