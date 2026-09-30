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
    pub fn from_attribute_value(value: &str) -> Option<Self> {
        match value {
            "delete" => Some(Self::Delete),
            "get" => Some(Self::Get),
            "patch" => Some(Self::Patch),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "query" => Some(Self::Query),
            _ => None,
        }
    }

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

    const DECLARED: [(&str, RouteMethod); 6] = [
        ("delete", RouteMethod::Delete),
        ("get", RouteMethod::Get),
        ("patch", RouteMethod::Patch),
        ("post", RouteMethod::Post),
        ("put", RouteMethod::Put),
        ("query", RouteMethod::Query),
    ];

    #[test]
    fn reads_every_declarable_method() {
        for (value, expected) in DECLARED {
            assert_eq!(RouteMethod::from_attribute_value(value), Some(expected));
        }
    }

    #[test]
    fn refuses_a_method_outside_the_declarable_set() {
        assert_eq!(RouteMethod::from_attribute_value("options"), None);
    }

    #[test]
    fn refuses_an_uppercase_declaration() {
        assert_eq!(RouteMethod::from_attribute_value("GET"), None);
    }

    #[test]
    fn classifies_every_routable_request_method() {
        for (value, expected) in DECLARED {
            let method = Method::from_bytes(value.to_uppercase().as_bytes())
                .expect("a routable method is a valid token");

            assert_eq!(RouteMethod::of(&method), Some(expected));
        }
    }

    #[test]
    fn does_not_route_a_method_outside_the_declarable_set() {
        assert_eq!(RouteMethod::of(&Method::OPTIONS), None);
    }
}
