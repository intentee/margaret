use std::path::PathBuf;

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

const IDENTITY_DEPENDENCIES: &[InheritedDependency] = &[
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
        additional_features: &["spiffe-types"],
        name: "spiffe",
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
];
const IDENTITY_LIBRARY_MODULES: &[&str] = &["routes", "system_clock"];
const IDENTITY_ROUTE_MODULES: &[&str] = &[
    "get_identity",
    "get_well_known_jwks",
    "post_mint_access_token",
];
const IDENTITY_SERVER_NAME: &str = "identity";

fn route_path(module_name: &str) -> PathBuf {
    PathBuf::from("src")
        .join("routes")
        .join(format!("{module_name}.rs"))
}

fn render_get_identity_route() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use spiffe::spiffe_id::SpiffeId;

            use margaret::framework::http::response::Response;
            use margaret::framework::macros::process;
            use margaret::framework::macros::responds_to_http;
            use margaret::framework::macros::singleton;

            #[singleton]
            #[responds_to_http(method = "get", path = "/identity", server = #IDENTITY_SERVER_NAME)]
            pub struct GetIdentity;

            impl GetIdentity {
                #[process]
                pub fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {
                    Ok(Response::text(
                        200,
                        format!("trust_domain={} path={}", peer.trust_domain(), peer.path()),
                    ))
                }
            }
        },
        relative_path: route_path("get_identity"),
    }
}

fn render_get_well_known_jwks_route() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use std::sync::Arc;

            use margaret::framework::http::response::Response;
            use margaret::framework::macros::constructor;
            use margaret::framework::macros::process;
            use margaret::framework::macros::responds_to_http;
            use margaret::framework::macros::singleton;

            use crate::margaret::jwks::PublicJwksHandler;

            #[singleton]
            #[responds_to_http(
                method = "get",
                path = "/.well-known/jwks.json",
                server = #IDENTITY_SERVER_NAME
            )]
            pub struct GetWellKnownJwks {
                public_jwks_handler: Arc<PublicJwksHandler>,
            }

            impl GetWellKnownJwks {
                #[constructor]
                pub fn create(public_jwks_handler: Arc<PublicJwksHandler>) -> anyhow::Result<Self> {
                    Ok(Self { public_jwks_handler })
                }

                #[process]
                pub fn respond(&self) -> anyhow::Result<Response> {
                    Ok(self.public_jwks_handler.respond())
                }
            }
        },
        relative_path: route_path("get_well_known_jwks"),
    }
}

fn render_post_mint_access_token_route() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use std::sync::Arc;

            use margaret::framework::http::request::Request;
            use margaret::framework::http::response::Response;
            use margaret::framework::macros::constructor;
            use margaret::framework::macros::process;
            use margaret::framework::macros::responds_to_http;
            use margaret::framework::macros::singleton;

            use crate::margaret::jwks::MintAccessTokenHandler;
            use crate::system_clock::SystemClock;

            #[singleton]
            #[responds_to_http(
                method = "post",
                path = "/.well-known/mint",
                server = #IDENTITY_SERVER_NAME
            )]
            pub struct PostMintAccessToken {
                clock: Arc<SystemClock>,
                mint_access_token_handler: Arc<MintAccessTokenHandler>,
            }

            impl PostMintAccessToken {
                #[constructor]
                pub fn create(
                    clock: Arc<SystemClock>,
                    mint_access_token_handler: Arc<MintAccessTokenHandler>,
                ) -> anyhow::Result<Self> {
                    Ok(Self {
                        clock,
                        mint_access_token_handler,
                    })
                }

                #[process]
                pub async fn respond(&self, request: &Request) -> anyhow::Result<Response> {
                    Ok(self
                        .mint_access_token_handler
                        .respond(request, self.clock.now())
                        .await)
                }
            }
        },
        relative_path: route_path("post_mint_access_token"),
    }
}

pub(crate) fn render_identity_service(
    margaret_revision: &str,
    project_name: &ProjectName,
    workspace_dependencies: &WorkspaceDependencyTable,
) -> Result<Vec<ScaffoldedFile>, ScaffoldError> {
    let crate_name = project_name.identity_crate();
    let modules = [
        render_service_build_script(),
        render_service_entry_point(crate_name),
        render_service_library_root(IDENTITY_LIBRARY_MODULES),
        render_system_clock(),
        render_module_declarations(
            PathBuf::from("src").join("routes").join("mod.rs"),
            IDENTITY_ROUTE_MODULES,
        ),
        render_get_identity_route(),
        render_get_well_known_jwks_route(),
        render_post_mint_access_token_route(),
    ];
    let mut files = vec![
        render_member_manifest(
            crate_name,
            IDENTITY_DEPENDENCIES,
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

    use super::render_identity_service;
    use crate::project_name::ProjectName;
    use crate::scaffolded_file::ScaffoldedFile;
    use crate::workspace_dependency_table::WorkspaceDependencyTable;
    use crate::workspace_manifest::WORKSPACE_MANIFEST;

    fn identity_service() -> Vec<ScaffoldedFile> {
        render_identity_service(
            "83a27bf",
            &ProjectName::new("acme".to_string()),
            &WorkspaceDependencyTable::parse(WORKSPACE_MANIFEST).expect("the manifest is parsed"),
        )
        .expect("the identity service is rendered")
    }

    fn contents_of(files: &[ScaffoldedFile], relative_path: &Path) -> String {
        files
            .iter()
            .find(|file| file.relative_path == relative_path)
            .map(|file| file.contents.clone())
            .expect("the scaffolded file is rendered")
    }

    #[test]
    fn nests_every_rendered_file_under_the_identity_crate() {
        let paths: Vec<PathBuf> = identity_service()
            .into_iter()
            .map(|file| file.relative_path)
            .collect();

        assert_eq!(
            paths,
            vec![
                PathBuf::from("acme_identity").join("Cargo.toml"),
                PathBuf::from("acme_identity").join(".gitignore"),
                PathBuf::from("acme_identity").join("build.rs"),
                PathBuf::from("acme_identity").join("src").join("main.rs"),
                PathBuf::from("acme_identity").join("src").join("lib.rs"),
                PathBuf::from("acme_identity")
                    .join("src")
                    .join("system_clock.rs"),
                PathBuf::from("acme_identity")
                    .join("src")
                    .join("routes")
                    .join("mod.rs"),
                PathBuf::from("acme_identity")
                    .join("src")
                    .join("routes")
                    .join("get_identity.rs"),
                PathBuf::from("acme_identity")
                    .join("src")
                    .join("routes")
                    .join("get_well_known_jwks.rs"),
                PathBuf::from("acme_identity")
                    .join("src")
                    .join("routes")
                    .join("post_mint_access_token.rs"),
            ]
        );
    }

    #[test]
    fn pins_the_identity_server_to_spiffe_mutual_tls_through_the_peer_parameter() {
        let route = contents_of(
            &identity_service(),
            &PathBuf::from("acme_identity")
                .join("src")
                .join("routes")
                .join("get_identity.rs"),
        );

        assert!(route.contains("peer: &SpiffeId"));
        assert!(route.contains("server = \"identity\""));
    }

    #[test]
    fn references_both_jwks_server_handlers_so_codegen_activates_the_roller() {
        let files = identity_service();
        let jwks = contents_of(
            &files,
            &PathBuf::from("acme_identity")
                .join("src")
                .join("routes")
                .join("get_well_known_jwks.rs"),
        );
        let mint = contents_of(
            &files,
            &PathBuf::from("acme_identity")
                .join("src")
                .join("routes")
                .join("post_mint_access_token.rs"),
        );

        assert!(jwks.contains("Arc<PublicJwksHandler>"));
        assert!(mint.contains("Arc<MintAccessTokenHandler>"));
    }

    #[test]
    fn declares_the_spiffe_dependency_the_peer_parameter_needs() {
        let manifest = contents_of(
            &identity_service(),
            &PathBuf::from("acme_identity").join("Cargo.toml"),
        );

        assert!(manifest.contains("spiffe = { version ="));
        assert!(manifest.contains("features = [\"spiffe-types\"]"));
    }
}
