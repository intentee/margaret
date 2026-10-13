use syn::Type;

pub struct GenericArgumentPair<'segment> {
    pub first: &'segment Type,
    pub second: &'segment Type,
}
