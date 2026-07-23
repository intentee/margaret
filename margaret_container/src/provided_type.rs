use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum ProvidedType {
    Concrete(CanonicalPath),
    Endpoint(CanonicalPath),
    Interface(CanonicalPath),
}

impl ProvidedType {
    pub(crate) fn key(&self) -> &CanonicalPath {
        match self {
            ProvidedType::Concrete(path)
            | ProvidedType::Endpoint(path)
            | ProvidedType::Interface(path) => path,
        }
    }
}
