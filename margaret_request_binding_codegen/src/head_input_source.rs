use proc_macro2::Ident;
use quote::format_ident;

pub enum HeadInputSource {
    Cookie,
    Query,
}

impl HeadInputSource {
    #[must_use]
    pub fn inputs_field(&self) -> Ident {
        match self {
            Self::Cookie => format_ident!("cookies"),
            Self::Query => format_ident!("query"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeadInputSource;

    #[test]
    fn reads_cookies_from_the_cookie_inputs() {
        assert_eq!(
            HeadInputSource::Cookie.inputs_field().to_string(),
            "cookies"
        );
    }

    #[test]
    fn reads_the_query_from_the_query_inputs() {
        assert_eq!(HeadInputSource::Query.inputs_field().to_string(), "query");
    }
}
