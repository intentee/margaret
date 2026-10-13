use std::ops::ControlFlow;

use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum ParameterValue<TValue> {
    Supported(TValue),
    Unsupported(String),
}

impl<TValue> ParameterValue<TValue> {
    pub(crate) fn supported<TRejection>(
        self,
        unsupported: impl FnOnce(String) -> TRejection,
    ) -> ControlFlow<TRejection, TValue> {
        match self {
            Self::Supported(value) => ControlFlow::Continue(value),
            Self::Unsupported(written) => ControlFlow::Break(unsupported(written)),
        }
    }
}
