use uuid::Uuid;

#[derive(Clone)]
pub(crate) enum RefreshTokenState {
    Current { family: Uuid },
    Superseded { family: Uuid },
}
