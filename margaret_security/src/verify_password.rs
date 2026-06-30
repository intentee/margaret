use argon2::Argon2;
use argon2::password_hash;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordVerifier;

use crate::security_error::SecurityError;

pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, SecurityError> {
    let parsed = PasswordHash::new(phc_hash).map_err(SecurityError::PasswordHashParse)?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(source) => Err(SecurityError::PasswordVerify(source)),
    }
}

#[cfg(test)]
mod tests {
    use argon2::Argon2;
    use argon2::password_hash::PasswordHasher;
    use argon2::password_hash::SaltString;

    use super::verify_password;

    fn fast_argon2() -> Argon2<'static> {
        Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(8, 1, 1, None).expect("the argon2 parameters are valid"),
        )
    }

    fn hash_of(password: &str) -> String {
        let salt = SaltString::from_b64("YWFhYWFhYWFhYWFhYWFhYQ").expect("a valid salt");

        fast_argon2()
            .hash_password(password.as_bytes(), &salt)
            .expect("hashing succeeds")
            .to_string()
    }

    #[test]
    fn accepts_the_matching_password() {
        let hash = hash_of("correct horse");

        assert!(verify_password("correct horse", &hash).expect("the hash verifies"));
    }

    #[test]
    fn rejects_a_wrong_password() {
        let hash = hash_of("correct horse");

        assert!(!verify_password("wrong", &hash).expect("the verification runs"));
    }

    #[test]
    fn reports_an_unparsable_hash() {
        let error =
            verify_password("anything", "not-a-phc-string").expect_err("the hash fails to parse");

        assert!(error.to_string().contains("could not be parsed"));
    }

    #[test]
    fn reports_a_hash_that_cannot_be_verified() {
        let memory_cost_below_minimum =
            "$argon2id$v=19$m=1,t=1,p=1$YWFhYWFhYWFhYWFhYWFhYQ$YWFhYWFhYWFhYWFhYWFhYQ";

        let error = verify_password("anything", memory_cost_below_minimum)
            .expect_err("the hash fails to verify");

        assert!(error.to_string().contains("could not be verified"));
    }
}
