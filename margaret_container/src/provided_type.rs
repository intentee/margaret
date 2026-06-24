use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum ProvidedType {
    Concrete(CanonicalPath),
    Interface(CanonicalPath),
}

impl ProvidedType {
    pub(crate) fn key(&self) -> &CanonicalPath {
        match self {
            ProvidedType::Concrete(path) => path,
            ProvidedType::Interface(path) => path,
        }
    }
}
