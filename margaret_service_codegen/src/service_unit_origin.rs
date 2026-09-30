use crate::runner_outcome::RunnerOutcome;

#[derive(Clone, Copy)]
pub(crate) enum ServiceUnitOrigin {
    Framework { outcome: RunnerOutcome },
    User,
}
