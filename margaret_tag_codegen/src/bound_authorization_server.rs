use margaret_accepted_clients_codegen::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_oauth_client_codegen::declared_client_authentication::DeclaredClientAuthentication;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;

pub enum BoundAuthorizationServer<'declarations, 'index> {
    External {
        authentication: &'declarations DeclaredClientAuthentication,
        client_id: &'declarations ClientId,
        issuer: TrustedIssuerBinding<'declarations, 'index>,
    },
    Own {
        admitted: &'declarations AcceptedClientDeclaration<'index>,
        issuance: &'declarations TokenIssuanceDeclaration<'index>,
    },
}
