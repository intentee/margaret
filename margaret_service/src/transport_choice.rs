use std::sync::Arc;

use clap::ValueEnum;
use clap::builder::PossibleValue;
use clap::builder::PossibleValuesParser;
use clap::builder::TypedValueParser;
use rustls::ServerConfig;

use margaret_http::transport_config::TransportConfig;

const PLAIN: &str = "plain";
const SPIFFE_MTLS: &str = "spiffe_mtls";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportChoice {
    Plain,
    SpiffeMtls,
}

impl TransportChoice {
    #[must_use]
    pub fn pinned_to_spiffe_mtls() -> impl TypedValueParser<Value = Self> {
        PossibleValuesParser::new([SPIFFE_MTLS]).map(|_| Self::SpiffeMtls)
    }

    #[must_use]
    pub fn config(self, spiffe_server_config: &Arc<ServerConfig>) -> TransportConfig {
        match self {
            Self::Plain => TransportConfig::Plain,
            Self::SpiffeMtls => TransportConfig::MutualTls {
                server_config: Arc::clone(spiffe_server_config),
            },
        }
    }
}

impl ValueEnum for TransportChoice {
    fn value_variants<'variants>() -> &'variants [Self] {
        &[Self::Plain, Self::SpiffeMtls]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::Plain => PLAIN,
            Self::SpiffeMtls => SPIFFE_MTLS,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use clap::Arg;
    use clap::Command;
    use clap::error::ErrorKind;
    use clap::value_parser;

    use rustls::ServerConfig;

    use margaret_http::transport_config::TransportConfig;
    use margaret_http_tests::tls_fixture::TlsFixture;

    use super::TransportChoice;

    fn chosen(arg: Arg, value: &str) -> Result<TransportChoice, clap::Error> {
        Command::new("serve")
            .arg(arg.long("transport").required(true))
            .try_get_matches_from(["serve", "--transport", value])
            .map(|matches| {
                *matches
                    .get_one::<TransportChoice>("transport")
                    .expect("the transport is required")
            })
    }

    fn mutual_tls_server_config(config: TransportConfig) -> Option<Arc<ServerConfig>> {
        match config {
            TransportConfig::MutualTls { server_config } => Some(server_config),
            TransportConfig::Plain => None,
        }
    }

    fn negotiable() -> Arg {
        Arg::new("transport").value_parser(value_parser!(TransportChoice))
    }

    fn pinned() -> Arg {
        Arg::new("transport").value_parser(TransportChoice::pinned_to_spiffe_mtls())
    }

    #[test]
    fn negotiates_a_plain_transport() {
        assert_eq!(
            chosen(negotiable(), "plain").expect("plain is a transport"),
            TransportChoice::Plain
        );
    }

    #[test]
    fn negotiates_a_spiffe_mutual_tls_transport() {
        assert_eq!(
            chosen(negotiable(), "spiffe_mtls").expect("spiffe_mtls is a transport"),
            TransportChoice::SpiffeMtls
        );
    }

    #[test]
    fn pins_a_server_to_spiffe_mutual_tls() {
        assert_eq!(
            chosen(pinned(), "spiffe_mtls").expect("spiffe_mtls is the pinned transport"),
            TransportChoice::SpiffeMtls
        );
    }

    #[test]
    fn refuses_a_plain_transport_for_a_pinned_server() {
        assert_eq!(
            chosen(pinned(), "plain")
                .expect_err("a pinned server refuses a plain transport")
                .kind(),
            ErrorKind::InvalidValue
        );
    }

    #[test]
    fn configures_a_plain_transport() {
        assert!(
            mutual_tls_server_config(
                TransportChoice::Plain.config(&TlsFixture::generate().server_config)
            )
            .is_none()
        );
    }

    #[test]
    fn configures_a_spiffe_mutual_tls_transport() {
        let server_config = TlsFixture::generate().server_config;

        assert!(Arc::ptr_eq(
            &mutual_tls_server_config(TransportChoice::SpiffeMtls.config(&server_config))
                .expect("the transport is mutual tls"),
            &server_config
        ));
    }
}
