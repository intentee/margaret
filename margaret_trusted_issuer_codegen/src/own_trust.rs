use margaret_attributes::tag::Tag;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

pub struct OwnTrust<'declarations> {
    pub audience: &'declarations str,
    pub issuer: &'declarations IssuerIdentifier,
    pub tag: &'declarations Tag,
}
