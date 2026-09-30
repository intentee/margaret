use margaret_attributes::canonical_path::CanonicalPath;

use crate::trusted_issuer_binding::TrustedIssuerBinding;

pub struct SubjectTokenExchangerBinding<'bindings> {
    pub declaring: CanonicalPath,
    pub issuer: &'bindings TrustedIssuerBinding,
}
