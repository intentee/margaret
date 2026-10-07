use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oauth_client_codegen::render_oauth_clients::render_oauth_clients;

pub(crate) fn build_oauth_client_artifacts(
    clients: &DeclaredOAuthClients,
) -> Vec<GeneratedModuleTokens> {
    if clients.clients.is_empty() {
        Vec::new()
    } else {
        render_oauth_clients(clients)
    }
}
