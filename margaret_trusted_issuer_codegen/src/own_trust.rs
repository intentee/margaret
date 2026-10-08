use margaret_attributes::tag::Tag;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

pub struct OwnTrust<'declarations> {
    pub audience: &'declarations Audience,
    pub issuer: &'declarations IssuerIdentifier,
    pub tag: &'declarations Tag,
}
