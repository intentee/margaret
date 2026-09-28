use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_parameter::IndexedParameter;
use margaret_tag_codegen::read_jwks_secret_store_target::read_jwks_secret_store_target;

use crate::container_error::ContainerError;
use crate::jwks_store_marking::JwksStoreMarking;
use crate::parameter_target::ParameterTarget;

pub(crate) struct DraftParameter<'index> {
    pub(crate) indexed: &'index IndexedParameter,
    pub(crate) jwks_store: JwksStoreMarking,
    pub(crate) target: ParameterTarget,
}

impl<'index> DraftParameter<'index> {
    pub(crate) fn read(
        index: &AttributeIndex,
        item: &IndexedItem,
        indexed: &'index IndexedParameter,
        concrete_path: &CanonicalPath,
    ) -> Result<Self, ContainerError> {
        let jwks_store = match indexed.framework_attribute(FrameworkAttribute::JwksSecretStore) {
            Some(attribute) => {
                let site = format!(
                    "parameter '{}' of singleton '{concrete_path}'",
                    indexed.diagnostic_name()
                );

                JwksStoreMarking::Marked(read_jwks_secret_store_target(attribute.args()?, &site)?)
            }
            None => JwksStoreMarking::Unmarked,
        };

        Ok(Self {
            indexed,
            jwks_store,
            target: ParameterTarget::peel(index, item, indexed.declared()),
        })
    }
}
