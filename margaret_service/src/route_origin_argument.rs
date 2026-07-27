use margaret_http::route_origin::RouteOrigin;

#[must_use]
pub fn route_origin_argument(name: &'static str) -> clap::Arg {
    clap::Arg::new(name)
        .long(name)
        .required(true)
        .value_parser(clap::value_parser!(RouteOrigin))
}

#[cfg(test)]
mod tests {
    use super::route_origin_argument;

    fn parses(value: &str) -> Result<clap::ArgMatches, clap::Error> {
        clap::Command::new("test")
            .arg(route_origin_argument("origin"))
            .try_get_matches_from(["test", "--origin", value])
    }

    #[test]
    fn accepts_only_canonical_https_origins() {
        assert!(parses("https://example.test").is_ok());
        assert!(parses("http://example.test").is_err());
    }
}
