#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl Method {
    pub(crate) fn from_http(method: &http::Method) -> Option<Self> {
        if *method == http::Method::DELETE {
            Some(Method::Delete)
        } else if *method == http::Method::GET {
            Some(Method::Get)
        } else if *method == http::Method::PATCH {
            Some(Method::Patch)
        } else if *method == http::Method::POST {
            Some(Method::Post)
        } else if *method == http::Method::PUT {
            Some(Method::Put)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Method;

    #[test]
    fn maps_each_supported_http_method() {
        assert_eq!(
            Method::from_http(&http::Method::DELETE),
            Some(Method::Delete)
        );
        assert_eq!(Method::from_http(&http::Method::GET), Some(Method::Get));
        assert_eq!(Method::from_http(&http::Method::PATCH), Some(Method::Patch));
        assert_eq!(Method::from_http(&http::Method::POST), Some(Method::Post));
        assert_eq!(Method::from_http(&http::Method::PUT), Some(Method::Put));
    }

    #[test]
    fn rejects_an_unsupported_http_method() {
        assert_eq!(Method::from_http(&http::Method::OPTIONS), None);
    }
}
