use crate::declared_trust::DeclaredTrust;
use crate::trusted_issuer_group::TrustedIssuerGroup;

pub struct TrustedIssuerBinding<'trusts, 'index> {
    pub group: &'trusts TrustedIssuerGroup<'index>,
    pub trust: &'trusts DeclaredTrust<'index>,
}
