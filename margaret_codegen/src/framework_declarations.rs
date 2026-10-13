use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_sign_in_endpoints_codegen::client_sign_in::ClientSignIn;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;

pub(crate) struct FrameworkDeclarations<'declarations, 'index> {
    pub(crate) accepted_clients: &'declarations DeclaredAcceptedClients<'index>,
    pub(crate) client_sign_ins:
        &'declarations [ClientSignIn<'declarations, 'declarations, 'index>],
    pub(crate) own_trusts: &'declarations [OwnTrust<'declarations>],
    pub(crate) sessions: &'declarations DeclaredSessions<'index>,
    pub(crate) trusts: &'declarations DeclaredTrusts<'index>,
}
