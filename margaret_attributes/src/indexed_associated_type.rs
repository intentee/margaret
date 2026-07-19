use syn::Type;

pub struct IndexedAssociatedType {
    name: String,
    ty: Type,
}

impl IndexedAssociatedType {
    pub(crate) fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}
