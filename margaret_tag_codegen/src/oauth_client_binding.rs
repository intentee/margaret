use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

use crate::trusted_issuer_binding::TrustedIssuerBinding;

pub struct OAuthClientBinding<'bindings> {
    pub declaring: CanonicalPath,
    pub issuer: &'bindings TrustedIssuerBinding,
    pub tag: Tag,
}
