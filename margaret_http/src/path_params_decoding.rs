use std::collections::HashMap;
use std::str::Utf8Error;

use matchit::Params;
use percent_encoding::percent_decode_str;

pub(crate) enum PathParamsDecoding {
    Decoded(HashMap<String, String>),
    NotValidUtf8 {
        parameter: String,
        source: Utf8Error,
    },
}

impl PathParamsDecoding {
    pub(crate) fn from_params(params: &Params<'_, '_>) -> Self {
        let mut decoded = HashMap::with_capacity(params.len());

        for (parameter, value) in params.iter() {
            match percent_decode_str(value).decode_utf8() {
                Ok(value) => {
                    decoded.insert(parameter.to_string(), value.into_owned());
                }
                Err(source) => {
                    return Self::NotValidUtf8 {
                        parameter: parameter.to_string(),
                        source,
                    };
                }
            }
        }

        Self::Decoded(decoded)
    }
}

#[cfg(test)]
mod tests {
    use matchit::Router;

    use super::PathParamsDecoding;

    fn decode(path: &str) -> PathParamsDecoding {
        let mut router: Router<()> = Router::new();

        router
            .insert("/articles/{article}", ())
            .expect("the route pattern is accepted");

        let matched = router.at(path).expect("the path matches the route");

        PathParamsDecoding::from_params(&matched.params)
    }

    fn decoded_article(path: &str) -> Option<String> {
        match decode(path) {
            PathParamsDecoding::Decoded(path_params) => path_params.get("article").cloned(),
            PathParamsDecoding::NotValidUtf8 { .. } => None,
        }
    }

    fn undecodable_parameter(path: &str) -> Option<String> {
        match decode(path) {
            PathParamsDecoding::Decoded(_) => None,
            PathParamsDecoding::NotValidUtf8 { parameter, .. } => Some(parameter),
        }
    }

    #[test]
    fn decodes_percent_escapes_in_a_parameter_value() {
        assert_eq!(
            decoded_article("/articles/rust%2Flang%20guide").as_deref(),
            Some("rust/lang guide")
        );
    }

    #[test]
    fn reports_the_parameter_whose_value_is_not_valid_utf8() {
        assert_eq!(
            undecodable_parameter("/articles/%FF").as_deref(),
            Some("article")
        );
    }

    #[test]
    fn reports_no_undecodable_parameter_when_every_value_decodes() {
        assert!(undecodable_parameter("/articles/rust").is_none());
    }

    #[test]
    fn reports_no_value_when_a_parameter_does_not_decode() {
        assert!(decoded_article("/articles/%FF").is_none());
    }
}
