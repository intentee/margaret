use syn::Path;

pub(crate) enum RawTarget {
    Collection(Path),
    Single(Path),
}
