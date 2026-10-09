#[must_use]
pub fn literal_url(origin: &str, path: &'static str) -> String {
    [origin, path].concat()
}

#[cfg(test)]
mod tests {
    use super::literal_url;

    #[test]
    fn joins_an_encoded_literal_path_onto_the_origin() {
        assert_eq!(
            literal_url("https://localhost:8443", "/oauth/%7Bcallback%7D"),
            "https://localhost:8443/oauth/%7Bcallback%7D"
        );
    }
}
