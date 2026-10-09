use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_client_codegen::module_sign_in::ModuleSignIn;
use margaret_oauth_client_codegen::oauth_client_module::OAuthClientModule;
use margaret_oauth_client_codegen::render_oauth_clients::render_oauth_clients;
use margaret_sign_in_endpoints_codegen::client_sign_in::ClientSignIn;
use margaret_sign_in_endpoints_codegen::served_sign_in::ServedSignIn;
use margaret_sign_in_endpoints_codegen::sign_in_service::SignInService;

fn module_sign_in<'declarations>(
    service: &'declarations SignInService,
) -> ModuleSignIn<'declarations> {
    match service {
        SignInService::Served(ServedSignIn {
            admission, scopes, ..
        }) => ModuleSignIn::Available { admission, scopes },
        SignInService::Unavailable => ModuleSignIn::Unavailable,
    }
}

pub(crate) fn build_oauth_client_artifacts(
    client_sign_ins: &[ClientSignIn],
) -> Vec<GeneratedModuleTokens> {
    if client_sign_ins.is_empty() {
        Vec::new()
    } else {
        render_oauth_clients(
            &client_sign_ins
                .iter()
                .map(|client_sign_in| OAuthClientModule {
                    client_id: client_sign_in.binding.client_id(),
                    credentials: &client_sign_in.binding.credentials,
                    sign_in: module_sign_in(&client_sign_in.service),
                    tag: &client_sign_in.binding.client.tag,
                })
                .collect::<Vec<OAuthClientModule>>(),
        )
    }
}
