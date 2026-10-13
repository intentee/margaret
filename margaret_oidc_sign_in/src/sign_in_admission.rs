use uuid::Uuid;

pub enum SignInAdmission {
    Admitted(Uuid),
    Refused,
}
