use std::collections::BTreeMap;

use crate::slotted_serve_input::SlottedServeInput;

pub(crate) fn unify_serve_inputs<'inputs>(
    inputs: impl IntoIterator<Item = &'inputs SlottedServeInput>,
) -> Vec<SlottedServeInput> {
    let mut inputs_by_slot = BTreeMap::new();

    for slotted in inputs {
        inputs_by_slot.entry(slotted.slot).or_insert(slotted);
    }

    inputs_by_slot.into_values().cloned().collect()
}
