use margaret_attributes::canonical_path::CanonicalPath;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;

pub struct SubjectTokenExchangerBinding<'trusts, 'index> {
    pub declaring: CanonicalPath,
    pub issuer: TrustedIssuerBinding<'trusts, 'index>,
}
