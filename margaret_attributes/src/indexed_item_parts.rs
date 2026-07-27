use crate::canonical_path::CanonicalPath;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_field::IndexedField;
use crate::indexed_method::IndexedMethod;
use crate::indexed_trait_impl::IndexedTraitImpl;
use crate::indexed_variant::IndexedVariant;
use crate::item_kind::ItemKind;

pub(crate) struct IndexedItemParts {
    pub(crate) attributes: Vec<IndexedAttribute>,
    pub(crate) canonical_path: CanonicalPath,
    pub(crate) fields: Vec<IndexedField>,
    pub(crate) identifier: String,
    pub(crate) kind: ItemKind,
    pub(crate) methods: Vec<IndexedMethod>,
    pub(crate) trait_impls: Vec<IndexedTraitImpl>,
    pub(crate) variants: Vec<IndexedVariant>,
}
