use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_client_codegen::render_oauth_clients::render_oauth_clients;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;

pub(crate) fn build_oauth_client_artifacts(
    oauth_client_bindings: &[OAuthClientBinding],
) -> Vec<GeneratedModuleTokens> {
    if oauth_client_bindings.is_empty() {
        Vec::new()
    } else {
        render_oauth_clients(oauth_client_bindings)
    }
}
