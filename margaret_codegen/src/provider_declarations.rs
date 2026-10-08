use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

pub(crate) struct ProviderDeclarations<'declarations, 'index> {
    pub(crate) accepted: &'declarations DeclaredAcceptedClients<'index>,
    pub(crate) token_issuance: &'declarations DeclaredTokenIssuance<'index>,
}
