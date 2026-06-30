use argon2::password_hash;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("the stored password hash could not be parsed")]
    PasswordHashParse(#[source] password_hash::Error),

    #[error("the password could not be verified against the stored hash")]
    PasswordVerify(#[source] password_hash::Error),
}
