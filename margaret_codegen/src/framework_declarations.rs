use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;

pub(crate) struct FrameworkDeclarations<'declarations, 'index> {
    pub(crate) accepted_clients: &'declarations DeclaredAcceptedClients<'index>,
    pub(crate) oauth_client_bindings: &'declarations [OAuthClientBinding<'declarations, 'index>],
    pub(crate) own_trusts: &'declarations [OwnTrust<'declarations>],
    pub(crate) trusts: &'declarations DeclaredTrusts<'index>,
}
