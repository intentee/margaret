use std::collections::BTreeSet;

use syn::Attribute;
use syn::Path;

use crate::attribute_error::AttributeError;
use crate::attribute_host::AttributeHost;
use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;
use crate::indexed_attribute::IndexedAttribute;

pub(crate) struct ScannedAttribute {
    attribute: Attribute,
}

impl ScannedAttribute {
    fn new(attribute: Attribute) -> Self {
        Self { attribute }
    }

    pub(crate) fn resolve_all(
        attributes: Vec<Self>,
        host: AttributeHost,
        target: impl Fn() -> String,
        resolve: impl Copy + Fn(&Path) -> CanonicalPath,
    ) -> Result<Vec<IndexedAttribute>, AttributeError> {
        let mut held: BTreeSet<FrameworkAttribute> = BTreeSet::new();

        attributes
            .into_iter()
            .map(|attribute| {
                let indexed = IndexedAttribute::from_canonical(
                    &attribute.attribute,
                    &resolve(attribute.attribute.path()),
                );

                match indexed.framework_attribute() {
                    Some(framework)
                        if !host.admits_repetition(framework) && !held.insert(framework) =>
                    {
                        Err(AttributeError::RepeatedAttribute {
                            attribute_path: framework.name().to_string(),
                            target: target(),
                        })
                    }
                    Some(_) | None => Ok(indexed),
                }
            })
            .collect()
    }

    pub(crate) fn scan_all(attributes: Vec<Attribute>) -> Vec<Self> {
        attributes.into_iter().map(Self::new).collect()
    }
}
