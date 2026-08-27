use clap::ArgMatches;

use margaret_http::body_limit::BodyLimit;

pub(crate) fn resolve_body_limit(matches: &ArgMatches, argument: &str) -> BodyLimit {
    match matches.get_one::<usize>(argument) {
        Some(max_bytes) => BodyLimit::new(*max_bytes),
        None => BodyLimit::default(),
    }
}

#[cfg(test)]
mod tests {
    use clap::Arg;
    use clap::ArgMatches;
    use clap::Command;

    use margaret_http::body_limit::BodyLimit;

    use super::resolve_body_limit;

    fn matches(arguments: &[&str]) -> ArgMatches {
        Command::new("test")
            .arg(
                Arg::new("public-body-limit")
                    .long("public-body-limit")
                    .value_parser(clap::value_parser!(usize)),
            )
            .try_get_matches_from(std::iter::once("test").chain(arguments.iter().copied()))
            .expect("the test arguments parse")
    }

    #[test]
    fn reads_the_declared_maximum_body_size() {
        assert_eq!(
            resolve_body_limit(
                &matches(&["--public-body-limit", "16"]),
                "public-body-limit"
            )
            .max_bytes(),
            16
        );
    }

    #[test]
    fn falls_back_to_the_framework_body_size() {
        assert_eq!(
            resolve_body_limit(&matches(&[]), "public-body-limit").max_bytes(),
            BodyLimit::default().max_bytes()
        );
    }
}
