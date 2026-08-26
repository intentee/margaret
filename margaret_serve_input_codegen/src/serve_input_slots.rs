use std::collections::BTreeMap;

use crate::serve_input::ServeInput;
use crate::serve_input_key::ServeInputKey;

pub struct ServeInputSlots {
    pub inputs: Vec<ServeInput>,
    pub slots: BTreeMap<ServeInputKey, usize>,
}
