use std::ops::ControlFlow;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ParameterValue<TValue> {
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
