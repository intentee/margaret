use proc_macro2::TokenStream;
use quote::quote;

use crate::constructor_outcome::ConstructorOutcome;

#[must_use]
pub fn reported_construction(invocation: &TokenStream, outcome: ConstructorOutcome) -> TokenStream {
    match outcome {
        ConstructorOutcome::Fallible => quote! {
            match #invocation {
                Ok(value) => value,
                Err(error) => {
                    return margaret::framework::console::report_failure::report_failure(error);
                }
            }
        },
        ConstructorOutcome::Infallible => invocation.clone(),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::reported_construction;
    use crate::constructor_outcome::ConstructorOutcome;

    fn collapsed(outcome: ConstructorOutcome) -> String {
        reported_construction(&quote! { build() }, outcome)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn reports_the_failure_of_a_fallible_construction_into_the_command_outcome() {
        assert_eq!(
            collapsed(ConstructorOutcome::Fallible),
            "matchbuild(){Ok(value)=>value,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}}"
        );
    }

    #[test]
    fn uses_an_infallible_construction_as_it_is_since_it_has_no_failure_to_report() {
        assert_eq!(collapsed(ConstructorOutcome::Infallible), "build()");
    }
}
