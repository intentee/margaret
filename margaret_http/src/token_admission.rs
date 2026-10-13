use crate::requirement::Requirement;
use crate::response_continuation::ResponseContinuation;

pub enum TokenAdmission<TToken> {
    Admitted(TToken),
    Refused(ResponseContinuation),
    Unaddressed,
}

impl<TToken> TokenAdmission<TToken> {
    #[must_use]
    pub fn into_requirement(self) -> Requirement<Option<TToken>> {
        match self {
            Self::Admitted(token) => Requirement::Met(Some(token)),
            Self::Refused(response) => Requirement::Unmet(response),
            Self::Unaddressed => Requirement::Met(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenAdmission;
    use crate::requirement::Requirement;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    fn met(requirement: Requirement<Option<u8>>) -> Result<Option<u8>, ResponseContinuation> {
        match requirement {
            Requirement::Met(token) => Ok(token),
            Requirement::Unmet(continuation) => Err(continuation),
        }
    }

    #[test]
    fn meets_the_requirement_with_an_admitted_token() {
        assert_eq!(
            met(TokenAdmission::Admitted(7).into_requirement()).ok(),
            Some(Some(7))
        );
    }

    #[test]
    fn meets_the_requirement_without_a_token_of_an_unaddressed_request() {
        assert_eq!(
            met(TokenAdmission::<u8>::Unaddressed.into_requirement()).ok(),
            Some(None)
        );
    }

    #[test]
    fn leaves_the_requirement_unmet_for_a_refused_token() {
        assert!(
            met(TokenAdmission::<u8>::Refused(
                ResponseContinuation::from(Response::unauthorized())
            )
            .into_requirement())
            .is_err()
        );
    }
}
