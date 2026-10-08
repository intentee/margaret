use http::Method;

const QUERY_METHOD_TOKEN: &str = "QUERY";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RouteMethod {
    Delete,
    Get,
    Patch,
    Post,
    Put,
    Query,
}

impl RouteMethod {
    #[must_use]
    pub fn of(method: &Method) -> Option<Self> {
        match *method {
            Method::DELETE => Some(Self::Delete),
            Method::GET => Some(Self::Get),
            Method::PATCH => Some(Self::Patch),
            Method::POST => Some(Self::Post),
            Method::PUT => Some(Self::Put),
            _ if method.as_str() == QUERY_METHOD_TOKEN => Some(Self::Query),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use super::RouteMethod;

    #[test]
    fn classifies_every_routable_request_method() {
        assert_eq!(RouteMethod::of(&Method::DELETE), Some(RouteMethod::Delete));
        assert_eq!(RouteMethod::of(&Method::GET), Some(RouteMethod::Get));
        assert_eq!(RouteMethod::of(&Method::PATCH), Some(RouteMethod::Patch));
        assert_eq!(RouteMethod::of(&Method::POST), Some(RouteMethod::Post));
        assert_eq!(RouteMethod::of(&Method::PUT), Some(RouteMethod::Put));
        assert_eq!(
            RouteMethod::of(&Method::from_bytes(b"QUERY").expect("QUERY is a valid method token")),
            Some(RouteMethod::Query)
        );
    }

    #[test]
    fn does_not_route_a_method_outside_the_declarable_set() {
        assert_eq!(RouteMethod::of(&Method::OPTIONS), None);
    }
}
