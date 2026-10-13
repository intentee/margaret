use std::future::Future;

use crate::requirement::Requirement;
use crate::response_continuation::ResponseContinuation;

pub async fn with_requirement<TValue, TOutput, TContinuation, TFuture>(
    requirement: Requirement<TValue>,
    refused: fn(ResponseContinuation) -> TOutput,
    continuation: TContinuation,
) -> TOutput
where
    TContinuation: FnOnce(TValue) -> TFuture,
    TFuture: Future<Output = TOutput>,
{
    match requirement {
        Requirement::Met(value) => continuation(value).await,
        Requirement::Unmet(refusal) => refused(refusal),
    }
}

#[cfg(test)]
mod tests {
    use super::with_requirement;
    use crate::requirement::Requirement;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    async fn named(name: String) -> Result<String, ResponseContinuation> {
        Ok(name)
    }

    #[tokio::test]
    async fn continues_with_the_value_of_a_met_requirement() {
        assert!(matches!(
            with_requirement(Requirement::Met("milo".to_string()), Err, named).await,
            Ok(name) if name == "milo"
        ));
    }

    #[tokio::test]
    async fn answers_an_unmet_requirement_with_its_refusal_without_continuing() {
        assert!(matches!(
            with_requirement(
                Requirement::<String>::Unmet(ResponseContinuation::from(Response::unauthorized())),
                Err,
                named,
            )
            .await,
            Err(ResponseContinuation::Done(response)) if response.status() == 401
        ));
    }
}
