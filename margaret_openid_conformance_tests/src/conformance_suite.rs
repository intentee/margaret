use std::collections::HashMap;
use std::net::IpAddr;
use std::net::SocketAddr;

use rcgen::SanType;
use rcgen::string::Ia5String;
use reqwest::ClientBuilder;
use serde::Deserialize;

use margaret_http_tests::fixture_certificate_authority::FixtureCertificateAuthority;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::issued_pem_certificate::IssuedPemCertificate;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

use crate::compose_project::ComposeProject;
use crate::docker_output::docker_output;
use crate::provider_host::PROVIDER_HOST;
use crate::suite_api::SuiteApi;
use crate::suite_base_url::SUITE_BASE_URL;
use crate::suite_host::SUITE_HOST;
use crate::suite_port::SUITE_PORT;

const PROVIDER_RELAY_COMPOSE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/compose/provider_relay.yml");

const SUITE_COMPOSE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/compose/conformance_suite.yml");

async fn proxy_address(project: &ComposeProject) -> SocketAddr {
    let container = docker_output(
        project.command().args(["ps", "--quiet", "nginx"]),
        "docker compose lists the suite proxy",
    )
    .await;
    let inspected = docker_output(
        tokio::process::Command::new("docker")
            .arg("inspect")
            .arg(String::from_utf8_lossy(&container).trim()),
        "docker inspects the suite proxy",
    )
    .await;
    let containers = serde_json::from_slice::<Vec<InspectedContainer>>(&inspected)
        .expect("docker describes the suite proxy as json");
    let [container]: [InspectedContainer; 1] = containers
        .try_into()
        .expect("docker describes exactly one suite proxy");
    let [network]: [AttachedNetwork; 1] = container
        .network_settings
        .networks
        .into_values()
        .collect::<Vec<AttachedNetwork>>()
        .try_into()
        .expect("the suite proxy is attached to exactly one network");

    SocketAddr::new(network.ip_address, SUITE_PORT)
}

struct ComposeVariable {
    name: &'static str,
    value: String,
}

#[derive(Debug, Deserialize)]
struct AttachedNetwork {
    #[serde(rename = "IPAddress")]
    ip_address: IpAddr,
}

#[derive(Debug, Deserialize)]
struct InspectedContainer {
    #[serde(rename = "NetworkSettings")]
    network_settings: NetworkSettings,
}

#[derive(Debug, Deserialize)]
struct NetworkSettings {
    #[serde(rename = "Networks")]
    networks: HashMap<String, AttachedNetwork>,
}

pub struct ConformanceSuite {
    pub api: SuiteApi,
    certificate_authority: FixtureCertificateAuthority,
    proxy: SocketAddr,
    project: ComposeProject,
}

impl ConformanceSuite {
    async fn composed(compose_files: &[&str], variables: Vec<ComposeVariable>) -> Self {
        let project = ComposeProject::named_uniquely();
        let certificate_authority = FixtureCertificateAuthority::generate();
        let IssuedPemCertificate {
            certificate_pem,
            private_key_pem,
        } = certificate_authority.issue_pem(
            SUITE_HOST,
            SanType::DnsName(Ia5String::try_from(SUITE_HOST).expect("the suite host is ia5")),
        );
        let mut up = project.command();

        for compose_file in compose_files {
            up.args(["--file", compose_file]);
        }

        up.args([
            "up",
            "--detach",
            "--wait",
            "--pull",
            "never",
            "--quiet-pull",
        ])
        .env("CONFORMANCE_SUITE_CERTIFICATE", certificate_pem)
        .env("CONFORMANCE_SUITE_HOST", SUITE_HOST)
        .env("CONFORMANCE_SUITE_PRIVATE_KEY", private_key_pem);

        for ComposeVariable { name, value } in variables {
            up.env(name, value);
        }

        docker_output(&mut up, "docker compose starts the suite").await;

        let proxy = proxy_address(&project).await;

        Self {
            api: SuiteApi {
                base_url: SUITE_BASE_URL.clone(),
                client: fixture_client_builder(&certificate_authority)
                    .resolve(SUITE_HOST, proxy)
                    .build()
                    .expect("the suite client builds"),
            },
            certificate_authority,
            proxy,
            project,
        }
    }

    /// # Panics
    ///
    /// Panics when docker cannot start the suite.
    pub async fn relaying_to(provider_port: u16) -> Self {
        Self::composed(
            &[SUITE_COMPOSE, PROVIDER_RELAY_COMPOSE],
            vec![
                ComposeVariable {
                    name: "MARGARET_PROVIDER_HOST",
                    value: PROVIDER_HOST.to_string(),
                },
                ComposeVariable {
                    name: "MARGARET_PROVIDER_PORT",
                    value: provider_port.to_string(),
                },
            ],
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when docker cannot start the suite.
    pub async fn start() -> Self {
        Self::composed(&[SUITE_COMPOSE], Vec::new()).await
    }

    /// # Panics
    ///
    /// Panics when the issuer request client cannot be built.
    #[must_use]
    pub fn issuer_request_client(&self) -> IssuerRequestClient {
        IssuerRequestClient::build(self.client_builder()).expect("the issuer request client builds")
    }

    pub fn stop(self) {
        drop(self.project);
    }

    /// # Panics
    ///
    /// Panics when the browsing client cannot be built.
    #[must_use]
    pub fn user_agent(&self) -> reqwest::Client {
        self.client_builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("the browsing client builds")
    }

    fn client_builder(&self) -> ClientBuilder {
        fixture_client_builder(&self.certificate_authority).resolve(SUITE_HOST, self.proxy)
    }
}
