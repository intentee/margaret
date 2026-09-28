use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_parameter::IndexedParameter;

use crate::parameter_target::ParameterTarget;

pub(crate) struct DraftParameter<'index> {
    pub(crate) indexed: &'index IndexedParameter,
    pub(crate) target: ParameterTarget,
}

impl<'index> DraftParameter<'index> {
    pub(crate) fn read(
        index: &AttributeIndex,
        item: &IndexedItem,
        indexed: &'index IndexedParameter,
    ) -> Self {
        Self {
            indexed,
            target: ParameterTarget::peel(index, item, indexed.declared()),
        }
    }
}
