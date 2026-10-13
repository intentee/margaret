use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::constants::SESSIONS_MODULE_NAME;
use crate::constants::SIGN_OUT_ENDPOINTS_MODULE_NAME;
use crate::constants::SIGN_OUT_HANDLER_MODULE_NAME;
use crate::constants::SIGN_OUT_HANDLER_TYPE_NAME;
use crate::sign_out_endpoint_module::SignOutEndpointModule;

fn endpoint_modules(endpoint: &SignOutEndpointModule) -> [GeneratedModuleTokens; 2] {
    let handler_module = format_ident!("{SIGN_OUT_HANDLER_MODULE_NAME}");
    let handler_type = format_ident!("{SIGN_OUT_HANDLER_TYPE_NAME}");

    [
        GeneratedModuleTokens::new(endpoint.module_path(), quote! { pub mod #handler_module; }),
        GeneratedModuleTokens::new(
            endpoint.handler_module_path(),
            quote! {
                pub use margaret::framework::sessions::session_sign_out_endpoint::#handler_type;
            },
        ),
    ]
}

pub(crate) fn render_sign_out_endpoints(
    endpoints: &[&SignOutEndpointModule],
) -> Vec<GeneratedModuleTokens> {
    if endpoints.is_empty() {
        return Vec::new();
    }

    let names = endpoints
        .iter()
        .map(|endpoint| format_ident!("{}", endpoint.name()));

    [GeneratedModuleTokens::new(
        ModulePath::root(SESSIONS_MODULE_NAME).child(SIGN_OUT_ENDPOINTS_MODULE_NAME),
        quote! { #(pub mod #names;)* },
    )]
    .into_iter()
    .chain(
        endpoints
            .iter()
            .flat_map(|endpoint| endpoint_modules(endpoint)),
    )
    .collect()
}
