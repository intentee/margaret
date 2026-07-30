use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone)]
pub(crate) enum ProvidedType {
    Concrete(CanonicalPath),
    Endpoint(CanonicalPath),
    UriSelected(CanonicalPath),
}

impl ProvidedType {
    pub(crate) fn is_trait_object(&self) -> bool {
        match self {
            ProvidedType::Concrete(_) => false,
            ProvidedType::Endpoint(_) | ProvidedType::UriSelected(_) => true,
        }
    }

    pub(crate) fn key(&self) -> &CanonicalPath {
        match self {
            ProvidedType::Concrete(path)
            | ProvidedType::Endpoint(path)
            | ProvidedType::UriSelected(path) => path,
        }
    }
}
