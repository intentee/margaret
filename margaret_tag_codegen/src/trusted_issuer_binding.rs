use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

use crate::trusted_issuer_kind::TrustedIssuerKind;

pub struct TrustedIssuerBinding {
    pub declaring: CanonicalPath,
    pub kind: TrustedIssuerKind,
    pub module_segment: String,
    pub tag: Tag,
}
