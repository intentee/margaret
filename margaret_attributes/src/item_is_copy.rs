use syn::Path;

use crate::attribute_error::AttributeError;
use crate::canonical_path::CanonicalPath;
use crate::copy_path::copy_path;
use crate::indexed_attribute::IndexedAttribute;
use crate::indexed_trait_impl::IndexedTraitImpl;

pub(crate) fn item_is_copy(
    attributes: &[IndexedAttribute],
    module_path: &[String],
    trait_impls: &[IndexedTraitImpl],
    resolve: impl Fn(&[String], &Path) -> CanonicalPath,
) -> Result<bool, AttributeError> {
    let copy = copy_path();

    for attribute in attributes {
        for derived in attribute.derived_paths()? {
            if resolve(module_path, &derived) == copy {
                return Ok(true);
            }
        }
    }

    Ok(trait_impls
        .iter()
        .any(|trait_impl| resolve(trait_impl.module_path(), trait_impl.trait_path()) == copy))
}
