use margaret_attributes::tag::Tag;
use margaret_registered_claims::audience::Audience;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedResource {
    pub audience: Audience,
    pub tag: Tag,
}
