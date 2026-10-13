use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub(crate) struct SignInAdmissionDeclaration {
    pub(crate) admission: CanonicalPath,
    pub(crate) client: Tag,
}
