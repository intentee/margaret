use margaret_http_codegen::http_server::HttpServer;
use margaret_oidc_provider_codegen::provider_endpoints_path::provider_endpoints_path;

pub(crate) enum ProviderServer {
    Absent,
    Named(String),
}

impl ProviderServer {
    pub(crate) fn origin_of(&self, server: HttpServer) -> HttpServer {
        match self {
            Self::Named(name) if name == server.name() => {
                server.served_at_issuer_origin(provider_endpoints_path())
            }
            Self::Absent | Self::Named(_) => server,
        }
    }
}
