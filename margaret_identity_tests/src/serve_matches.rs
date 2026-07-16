use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;

#[must_use]
pub fn serve_matches(arguments: &[&str]) -> ArgMatches {
    let mut invocation = vec!["identity"];
    invocation.extend_from_slice(arguments);

    Command::new("identity")
        .arg(Arg::new("internal-addr").long("internal-addr"))
        .arg(Arg::new("internal-url").long("internal-url"))
        .arg(
            Arg::new("internal-uploads")
                .long("internal-uploads")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("internal-upload-dir").long("internal-upload-dir"))
        .arg(Arg::new("public-addr").long("public-addr"))
        .arg(Arg::new("public-url").long("public-url"))
        .arg(
            Arg::new("public-uploads")
                .long("public-uploads")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("public-upload-dir").long("public-upload-dir"))
        .arg(Arg::new("public-transport").long("public-transport"))
        .arg(Arg::new("spiffe-trust-domain").long("spiffe-trust-domain"))
        .arg(Arg::new("spire-agent-addr").long("spire-agent-addr"))
        .arg(
            Arg::new("jwks-secret-path")
                .long("jwks-secret-path")
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .try_get_matches_from(invocation)
        .expect("the serve arguments parse")
}
