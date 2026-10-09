use crate::route_method::RouteMethod;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContentMethod {
    Delete,
    Patch,
    Post,
    Put,
    Query,
}

impl ContentMethod {
    #[must_use]
    pub fn of(method: RouteMethod) -> Option<Self> {
        match method {
            RouteMethod::Delete => Some(Self::Delete),
            RouteMethod::Get => None,
            RouteMethod::Patch => Some(Self::Patch),
            RouteMethod::Post => Some(Self::Post),
            RouteMethod::Put => Some(Self::Put),
            RouteMethod::Query => Some(Self::Query),
        }
    }

    #[must_use]
    pub fn route_method(self) -> RouteMethod {
        match self {
            Self::Delete => RouteMethod::Delete,
            Self::Patch => RouteMethod::Patch,
            Self::Post => RouteMethod::Post,
            Self::Put => RouteMethod::Put,
            Self::Query => RouteMethod::Query,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContentMethod;
    use crate::route_method::RouteMethod;

    #[test]
    fn carries_content_on_every_method_but_get() {
        for method in [
            RouteMethod::Delete,
            RouteMethod::Patch,
            RouteMethod::Post,
            RouteMethod::Put,
            RouteMethod::Query,
        ] {
            assert_eq!(
                ContentMethod::of(method).map(ContentMethod::route_method),
                Some(method)
            );
        }
    }

    #[test]
    fn carries_no_content_on_get() {
        assert_eq!(ContentMethod::of(RouteMethod::Get), None);
    }
}
