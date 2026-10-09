#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExtractionPhase {
    CallerIdentity,
    RequestInput,
    RouteModel,
}
