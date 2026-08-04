use std::path::PathBuf;

use proc_macro2::Ident;
use proc_macro2::Span;
use syn::parse_quote;

use crate::format_scaffolded_module::format_scaffolded_module;
use crate::generated_module_gitignore::generated_module_gitignore;
use crate::inherited_dependency::InheritedDependency;
use crate::project_name::ProjectName;
use crate::render_member_manifest::render_member_manifest;
use crate::render_module_declarations::render_module_declarations;
use crate::render_service_build_script::render_service_build_script;
use crate::render_service_entry_point::render_service_entry_point;
use crate::render_service_library_root::render_service_library_root;
use crate::render_system_clock::render_system_clock;
use crate::scaffold_error::ScaffoldError;
use crate::scaffolded_file::ScaffoldedFile;
use crate::scaffolded_module::ScaffoldedModule;
use crate::workspace_dependency_table::WorkspaceDependencyTable;

const BASE_DEPENDENCIES: &[InheritedDependency] = &[
    InheritedDependency {
        additional_features: &[],
        name: "anyhow",
    },
    InheritedDependency {
        additional_features: &[],
        name: "async-trait",
    },
    InheritedDependency {
        additional_features: &["clock"],
        name: "chrono",
    },
    InheritedDependency {
        additional_features: &[],
        name: "clap",
    },
    InheritedDependency {
        additional_features: &[],
        name: "reqwest",
    },
    InheritedDependency {
        additional_features: &["derive"],
        name: "serde",
    },
    InheritedDependency {
        additional_features: &["macros", "net", "rt-multi-thread", "sync", "time"],
        name: "tokio",
    },
    InheritedDependency {
        additional_features: &[],
        name: "tokio-util",
    },
    InheritedDependency {
        additional_features: &[],
        name: "trzcina",
    },
    InheritedDependency {
        additional_features: &[],
        name: "url",
    },
    InheritedDependency {
        additional_features: &["derive"],
        name: "validator",
    },
];
const BASE_FORM_MODULES: &[&str] = &["access_token_cookie"];
const BASE_LIBRARY_MODULES: &[&str] = &[
    "access_claims",
    "forms",
    "identity_client",
    "jwks_endpoint",
    "routes",
    "system_clock",
];
const BASE_ROUTE_MODULES: &[&str] = &["get_whoami"];
const BASE_SERVER_NAME: &str = "base";
const IDENTITY_SERVICE_IDENTITY_URL: &str = "https://identity.internal/identity";
const IDENTITY_SERVICE_JWKS_URL: &str = "https://identity.internal/.well-known/jwks.json";
const JWKS_CLIENT_TAG: &str = "identity";

fn render_access_claims() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use chrono::DateTime;
            use chrono::Utc;
            use serde::Deserialize;

            use margaret::framework::identity_session::is_expired::IsExpired;

            #[derive(Deserialize)]
            pub struct AccessClaims {
                pub exp: i64,
                pub sub: String,
            }

            impl IsExpired for AccessClaims {
                fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool> {
                    Ok(self.exp < now.timestamp())
                }
            }
        },
        relative_path: PathBuf::from("src").join("access_claims.rs"),
    }
}

fn render_access_token_cookie() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            #[derive(serde::Deserialize, validator::Validate)]
            pub struct AccessTokenCookie {
                pub access_token: Option<String>,
            }
        },
        relative_path: PathBuf::from("src")
            .join("forms")
            .join("access_token_cookie.rs"),
    }
}

fn render_identity_client() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use reqwest::Client;
            use url::Url;

            use margaret::framework::macros::constructor;
            use margaret::framework::macros::singleton;

            const IDENTITY_SERVICE_IDENTITY_URL: &str = #IDENTITY_SERVICE_IDENTITY_URL;

            #[singleton]
            pub struct IdentityClient {
                http_client: Client,
                identity_url: Url,
            }

            impl IdentityClient {
                #[constructor]
                pub fn create(#[spiffe_http_client] http_client: Client) -> anyhow::Result<Self> {
                    Ok(Self {
                        http_client,
                        identity_url: Url::parse(IDENTITY_SERVICE_IDENTITY_URL)?,
                    })
                }

                pub async fn fetch_peer_identity(&self) -> anyhow::Result<String> {
                    Ok(self
                        .http_client
                        .get(self.identity_url.clone())
                        .send()
                        .await?
                        .text()
                        .await?)
                }
            }
        },
        relative_path: PathBuf::from("src").join("identity_client.rs"),
    }
}

fn render_jwks_endpoint() -> ScaffoldedModule {
    let jwks_client_tag = Ident::new(JWKS_CLIENT_TAG, Span::call_site());

    ScaffoldedModule {
        file: parse_quote! {
            use async_trait::async_trait;
            use url::Url;

            use margaret::framework::jwks_endpoint::endpoint_error::EndpointError;
            use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
            use margaret::framework::macros::provides_jwks_endpoint;
            use margaret::framework::macros::singleton;

            const IDENTITY_SERVICE_JWKS_URL: &str = #IDENTITY_SERVICE_JWKS_URL;

            #[singleton]
            #[provides_jwks_endpoint(#jwks_client_tag)]
            pub struct JwksEndpoint;

            #[async_trait]
            impl ProvidesEndpoint for JwksEndpoint {
                async fn provide(&self) -> anyhow::Result<Url> {
                    Url::parse(IDENTITY_SERVICE_JWKS_URL).map_err(|source| {
                        EndpointError::Resolution {
                            source: Box::new(source),
                        }
                        .into()
                    })
                }
            }
        },
        relative_path: PathBuf::from("src").join("jwks_endpoint.rs"),
    }
}

fn render_get_whoami_route() -> ScaffoldedModule {
    let jwks_client_tag = Ident::new(JWKS_CLIENT_TAG, Span::call_site());

    ScaffoldedModule {
        file: parse_quote! {
            use std::sync::Arc;

            use margaret::framework::http::response::Response;
            use margaret::framework::jwks_client::access_token_verification::AccessTokenVerification;
            use margaret::framework::macros::constructor;
            use margaret::framework::macros::process;
            use margaret::framework::macros::responds_to_http;
            use margaret::framework::macros::singleton;

            use crate::access_claims::AccessClaims;
            use crate::forms::access_token_cookie::AccessTokenCookie;
            use crate::identity_client::IdentityClient;
            use crate::margaret::jwks::jwks_endpoint_jwks_endpoint::PublicJwksVerifier;
            use crate::system_clock::SystemClock;

            #[singleton]
            #[responds_to_http(method = "get", path = "/whoami", server = #BASE_SERVER_NAME)]
            pub struct GetWhoami {
                clock: Arc<SystemClock>,
                identity_client: Arc<IdentityClient>,
                verifier: Arc<PublicJwksVerifier>,
            }

            impl GetWhoami {
                #[constructor]
                pub fn create(
                    clock: Arc<SystemClock>,
                    identity_client: Arc<IdentityClient>,
                    #[jwks_secret_store(client = #jwks_client_tag)] verifier: Arc<PublicJwksVerifier>,
                ) -> anyhow::Result<Self> {
                    Ok(Self {
                        clock,
                        identity_client,
                        verifier,
                    })
                }

                #[process]
                pub async fn respond(
                    &self,
                    #[form_request(from = Cookie)] cookie: AccessTokenCookie,
                ) -> anyhow::Result<Response> {
                    let Some(access_token) = cookie.access_token else {
                        return Ok(Response::text(401, "the access token cookie is missing"));
                    };

                    Ok(
                        match self
                            .verifier
                            .verify::<AccessClaims>(&access_token, self.clock.now())?
                        {
                            AccessTokenVerification::Verified(AccessClaims { sub, .. }) => {
                                let peer = self.identity_client.fetch_peer_identity().await?;

                                Response::text(200, format!("subject={sub} peer={peer}"))
                            }
                            AccessTokenVerification::Expired => {
                                Response::text(401, "the access token is expired")
                            }
                            AccessTokenVerification::Malformed(malformation) => Response::text(
                                400,
                                format!("the access token is malformed: {malformation}"),
                            ),
                            AccessTokenVerification::NotReady => Response::text(
                                503,
                                "the jwks document of the identity service is not available yet",
                            ),
                            AccessTokenVerification::SignatureMismatch => {
                                Response::text(401, "the access token is not signed by a published key")
                            }
                        },
                    )
                }
            }
        },
        relative_path: PathBuf::from("src").join("routes").join("get_whoami.rs"),
    }
}

pub(crate) fn render_base_service(
    margaret_revision: &str,
    project_name: &ProjectName,
    workspace_dependencies: &WorkspaceDependencyTable,
) -> Result<Vec<ScaffoldedFile>, ScaffoldError> {
    let crate_name = project_name.base_crate();
    let modules = [
        render_service_build_script(),
        render_service_entry_point(crate_name),
        render_service_library_root(BASE_LIBRARY_MODULES),
        render_system_clock(),
        render_access_claims(),
        render_module_declarations(
            PathBuf::from("src").join("forms").join("mod.rs"),
            BASE_FORM_MODULES,
        ),
        render_access_token_cookie(),
        render_identity_client(),
        render_jwks_endpoint(),
        render_module_declarations(
            PathBuf::from("src").join("routes").join("mod.rs"),
            BASE_ROUTE_MODULES,
        ),
        render_get_whoami_route(),
    ];
    let mut files = vec![
        render_member_manifest(
            crate_name,
            BASE_DEPENDENCIES,
            margaret_revision,
            workspace_dependencies,
        )?,
        generated_module_gitignore(),
    ];

    files.extend(modules.into_iter().map(format_scaffolded_module));

    Ok(files
        .into_iter()
        .map(|file| file.nested_under(crate_name))
        .collect())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::render_base_service;
    use crate::project_name::ProjectName;
    use crate::scaffolded_file::ScaffoldedFile;
    use crate::workspace_dependency_table::WorkspaceDependencyTable;
    use crate::workspace_manifest::WORKSPACE_MANIFEST;

    fn base_service() -> Vec<ScaffoldedFile> {
        render_base_service(
            "83a27bf",
            &ProjectName::new("acme".to_string()),
            &WorkspaceDependencyTable::parse(WORKSPACE_MANIFEST).expect("the manifest is parsed"),
        )
        .expect("the base service is rendered")
    }

    fn contents_of(files: &[ScaffoldedFile], relative_path: &Path) -> String {
        files
            .iter()
            .find(|file| file.relative_path == relative_path)
            .map(|file| file.contents.clone())
            .expect("the scaffolded file is rendered")
    }

    #[test]
    fn nests_every_rendered_file_under_the_base_crate() {
        let paths: Vec<PathBuf> = base_service()
            .into_iter()
            .map(|file| file.relative_path)
            .collect();

        assert_eq!(
            paths,
            vec![
                PathBuf::from("acme_base").join("Cargo.toml"),
                PathBuf::from("acme_base").join(".gitignore"),
                PathBuf::from("acme_base").join("build.rs"),
                PathBuf::from("acme_base").join("src").join("main.rs"),
                PathBuf::from("acme_base").join("src").join("lib.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("system_clock.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("access_claims.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("forms")
                    .join("mod.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("forms")
                    .join("access_token_cookie.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("identity_client.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("jwks_endpoint.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("routes")
                    .join("mod.rs"),
                PathBuf::from("acme_base")
                    .join("src")
                    .join("routes")
                    .join("get_whoami.rs"),
            ]
        );
    }

    #[test]
    fn opts_the_base_service_into_the_spiffe_http_client() {
        let client = contents_of(
            &base_service(),
            &PathBuf::from("acme_base")
                .join("src")
                .join("identity_client.rs"),
        );

        assert!(client.contains("#[spiffe_http_client] http_client: Client"));
        assert!(client.contains("https://identity.internal/identity"));
    }

    #[test]
    fn binds_the_verifier_to_the_endpoint_the_base_service_declares() {
        let files = base_service();
        let endpoint = contents_of(
            &files,
            &PathBuf::from("acme_base")
                .join("src")
                .join("jwks_endpoint.rs"),
        );
        let route = contents_of(
            &files,
            &PathBuf::from("acme_base")
                .join("src")
                .join("routes")
                .join("get_whoami.rs"),
        );

        assert!(endpoint.contains("#[provides_jwks_endpoint(identity)]"));
        assert!(route.contains("#[jwks_secret_store(client = identity)]"));
        assert!(route.contains("jwks_endpoint_jwks_endpoint::PublicJwksVerifier"));
    }

    #[test]
    fn answers_every_token_verification_outcome_without_failing_the_request() {
        let route = contents_of(
            &base_service(),
            &PathBuf::from("acme_base")
                .join("src")
                .join("routes")
                .join("get_whoami.rs"),
        );

        assert!(route.contains("AccessTokenVerification::Verified(AccessClaims { sub, .. })"));
        assert!(route.contains("AccessTokenVerification::Expired"));
        assert!(route.contains("AccessTokenVerification::Malformed(malformation)"));
        assert!(route.contains("AccessTokenVerification::NotReady"));
        assert!(route.contains("AccessTokenVerification::SignatureMismatch"));
    }
}
