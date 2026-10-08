use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_client_codegen::module_sign_in::ModuleSignIn;
use margaret_oauth_client_codegen::oauth_client_module::OAuthClientModule;
use margaret_oauth_client_codegen::render_oauth_clients::render_oauth_clients;
use margaret_tag_codegen::bound_sign_in::BoundSignIn;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;

pub(crate) fn build_oauth_client_artifacts(
    bindings: &[OAuthClientBinding],
) -> Vec<GeneratedModuleTokens> {
    if bindings.is_empty() {
        Vec::new()
    } else {
        render_oauth_clients(
            &bindings
                .iter()
                .map(|binding| OAuthClientModule {
                    client_id: binding.client_id(),
                    sign_in: match binding.sign_in {
                        BoundSignIn::Available { scopes, .. } => ModuleSignIn::Available { scopes },
                        BoundSignIn::Unavailable => ModuleSignIn::Unavailable,
                    },
                    tag: &binding.client.tag,
                })
                .collect::<Vec<OAuthClientModule>>(),
        )
    }
}
