use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oauth_client_codegen::oauth_client_item::OAuthClientItem;
use margaret_oauth_client_codegen::oauth_client_item_path::oauth_client_item_path;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_trusted_issuer_codegen::issuer_metadata_canonical_path::issuer_metadata_canonical_path;
use margaret_trusted_issuer_codegen::trusted_issuer_canonical_path::trusted_issuer_canonical_path;

use crate::issuer_request_client_canonical_path::issuer_request_client_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

fn created(dependencies: Vec<FrameworkDependency>) -> FrameworkConstruction {
    FrameworkConstruction::Constructor {
        dependencies,
        is_async: false,
        method: "create".to_string(),
        outcome: ConstructorOutcome::Infallible,
    }
}

fn oauth_client_providers(
    OAuthClientBinding {
        declaring,
        issuer,
        tag,
    }: &OAuthClientBinding,
) -> [FrameworkProvider; 4] {
    let authorization_server =
        oauth_client_item_path(tag, OAuthClientItem::AuthorizationServerClient);
    let depending_on_server = |item: OAuthClientItem| FrameworkProvider {
        construction: created(vec![FrameworkDependency::Provider(
            authorization_server.clone(),
        )]),
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: oauth_client_item_path(tag, item),
    };

    [
        FrameworkProvider {
            construction: created(vec![
                FrameworkDependency::Provider(issuer_request_client_canonical_path()),
                FrameworkDependency::Provider(issuer_metadata_canonical_path(
                    &issuer.module_segment,
                )),
                FrameworkDependency::Provider(trusted_issuer_canonical_path(
                    &issuer.module_segment,
                )),
                FrameworkDependency::SingletonView(declaring.clone()),
            ]),
            enablement: FrameworkEnablement::Always,
            injection: FrameworkInjectionRole::OAuthClient(tag.clone()),
            provided: authorization_server.clone(),
        },
        depending_on_server(OAuthClientItem::ClientCredentials),
        FrameworkProvider {
            construction: created(vec![
                FrameworkDependency::Provider(authorization_server.clone()),
                FrameworkDependency::Provider(server_secret_store_canonical_path()),
            ]),
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: oauth_client_item_path(tag, OAuthClientItem::SignInFlow),
        },
        depending_on_server(OAuthClientItem::TokenExchange),
    ]
}

pub(crate) fn oauth_client_framework_providers(
    oauth_client_bindings: &[OAuthClientBinding],
) -> Vec<FrameworkProvider> {
    oauth_client_bindings
        .iter()
        .flat_map(oauth_client_providers)
        .collect()
}
