use margaret_oauth_client_codegen::oauth_client_declaration::OAuthClientDeclaration;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;

pub struct OAuthClientBinding<'declarations, 'index> {
    pub client: &'declarations OAuthClientDeclaration<'index>,
    pub issuer: TrustedIssuerBinding<'declarations, 'index>,
}
