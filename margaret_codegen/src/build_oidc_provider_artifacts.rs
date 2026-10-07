use margaret_attributes::tag::Tag;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::render_oidc_provider::render_oidc_provider;
use margaret_oidc_provider_codegen::subject_token_exchanger_path::subject_token_exchanger_path;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;

pub(crate) fn build_oidc_provider_artifacts(
    bindings: &ContainerBindings,
    exchangers: &[SubjectTokenExchangerBinding],
) -> Vec<GeneratedModuleTokens> {
    let items: Vec<OidcProviderItem> = OidcProviderItem::ALL
        .into_iter()
        .filter(|item| bindings.provides(&oidc_provider_item_path(*item)))
        .collect();
    let exchanger_issuers: Vec<&Tag> = exchangers
        .iter()
        .map(|binding| &binding.issuer.trust.tag)
        .filter(|issuer| bindings.provides(&subject_token_exchanger_path(issuer)))
        .collect();

    if items.is_empty() {
        Vec::new()
    } else {
        render_oidc_provider(&items, &exchanger_issuers)
    }
}
