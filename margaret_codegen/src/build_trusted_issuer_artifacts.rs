use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;
use margaret_trusted_issuer_codegen::render_trusted_issuers::render_trusted_issuers;

pub(crate) fn build_trusted_issuer_artifacts(
    trusts: &DeclaredTrusts,
    own_trusts: &[OwnTrust],
) -> Vec<GeneratedModuleTokens> {
    if trusts.groups.is_empty() && own_trusts.is_empty() {
        Vec::new()
    } else {
        render_trusted_issuers(trusts, own_trusts)
    }
}
