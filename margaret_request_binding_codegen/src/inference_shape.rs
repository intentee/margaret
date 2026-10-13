use proc_macro2::TokenStream;
use quote::quote;

use crate::inference_fallibility::InferenceFallibility;
use crate::inference_kind::InferenceKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InferenceShape {
    pub fallibility: InferenceFallibility,
    pub kind: InferenceKind,
}

impl InferenceShape {
    #[must_use]
    pub fn inferred_type(self, user: &TokenStream) -> TokenStream {
        self.result_of(&self.kind.inferred_type(user))
    }

    #[must_use]
    pub fn result_of(self, inferred: &TokenStream) -> TokenStream {
        match self.fallibility {
            InferenceFallibility::Fallible => {
                quote! { margaret::framework::anyhow::Result<#inferred> }
            }
            InferenceFallibility::Infallible => inferred.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::InferenceShape;
    use crate::inference_fallibility::InferenceFallibility;
    use crate::inference_kind::InferenceKind;

    fn inferred(fallibility: InferenceFallibility, kind: InferenceKind) -> String {
        InferenceShape { fallibility, kind }
            .inferred_type(&quote! { crate::User })
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn infers_an_outcome_that_may_interrupt_and_fail() {
        assert_eq!(
            inferred(InferenceFallibility::Fallible, InferenceKind::Outcome),
            "margaret::framework::anyhow::Result<margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<crate::User>>"
        );
    }

    #[test]
    fn infers_a_presence_that_can_neither_interrupt_nor_fail() {
        assert_eq!(
            inferred(InferenceFallibility::Infallible, InferenceKind::Presence),
            "margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence<crate::User>"
        );
    }
}
