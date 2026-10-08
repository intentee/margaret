use uuid::Uuid;

#[derive(Clone)]
pub struct AccessTokenHolder {
    pub subject: Uuid,
}
