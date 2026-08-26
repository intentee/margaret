use std::collections::BTreeMap;

use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::planned_provider::PlannedProvider;

pub(crate) struct RootServeInputs {
    pub(crate) inputs: Vec<ServeInput>,
    pub(crate) slots: Vec<usize>,
}

impl RootServeInputs {
    pub(crate) fn from_roots(roots: &[&PlannedProvider]) -> Self {
        let mut inputs_by_slot = BTreeMap::new();

        for root in roots {
            for (input, slot) in root
                .serve_inputs
                .iter()
                .zip(root.serve_input_slots.iter().copied())
            {
                inputs_by_slot.entry(slot).or_insert(input);
            }
        }

        let mut inputs = Vec::with_capacity(inputs_by_slot.len());
        let mut slots = Vec::with_capacity(inputs_by_slot.len());

        for (slot, input) in inputs_by_slot {
            inputs.push(input.clone());
            slots.push(slot);
        }

        Self { inputs, slots }
    }
}
