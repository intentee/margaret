use std::any::type_name;
use std::env::VarError;
use std::env::var;
use std::error::Error;
use std::str::FromStr;

use zeroize::Zeroizing;

use crate::environment_variable_error::EnvironmentVariableError;

/// # Errors
///
/// Returns `EnvironmentVariableError::NotUnicode` or
/// `EnvironmentVariableError::Malformed`.
pub fn read_optional<Value>(name: &str) -> Result<Option<Value>, EnvironmentVariableError>
where
    Value: FromStr,
    <Value as FromStr>::Err: Error + Send + Sync + 'static,
{
    let raw = match var(name) {
        Ok(raw) => Zeroizing::new(raw),
        Err(VarError::NotPresent) => return Ok(None),
        Err(VarError::NotUnicode(raw)) => {
            drop(Zeroizing::new(raw.into_encoded_bytes()));

            return Err(EnvironmentVariableError::NotUnicode {
                name: name.to_string(),
            });
        }
    };

    match raw.parse::<Value>() {
        Ok(value) => Ok(Some(value)),
        Err(source) => Err(EnvironmentVariableError::Malformed {
            name: name.to_string(),
            source: Box::new(source),
            value_type: type_name::<Value>(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    #[cfg(unix)]
    use std::ffi::OsStr;
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    use std::path::PathBuf;

    use crate::environment_variable_error::EnvironmentVariableError;

    use super::read_optional;

    fn set(name: &str, value: &str) {
        // SAFETY: `cargo nextest` runs every test in its own process, so no
        // other thread observes the process environment while it is mutated.
        unsafe { env::set_var(name, value) };
    }

    #[test]
    fn reads_an_absent_variable_as_none() {
        assert_eq!(
            read_optional::<String>("MARGARET_OPTIONAL_ABSENT").expect("an absent variable reads"),
            None
        );
    }

    #[test]
    fn reads_a_present_string() {
        set("MARGARET_OPTIONAL_PRESENT", "postgres://localhost");

        assert_eq!(
            read_optional::<String>("MARGARET_OPTIONAL_PRESENT").expect("the variable reads"),
            Some("postgres://localhost".to_string())
        );
    }

    #[test]
    fn parses_a_present_value_into_its_declared_type() {
        set("MARGARET_OPTIONAL_PORT", "8443");

        assert_eq!(
            read_optional::<u16>("MARGARET_OPTIONAL_PORT").expect("the variable parses"),
            Some(8443)
        );
    }

    #[test]
    fn parses_a_present_path() {
        set("MARGARET_OPTIONAL_ROOT", "/srv/uploads");

        assert_eq!(
            read_optional::<PathBuf>("MARGARET_OPTIONAL_ROOT").expect("the variable parses"),
            Some(PathBuf::from("/srv/uploads"))
        );
    }

    #[test]
    fn reports_a_value_that_does_not_parse() {
        set("MARGARET_OPTIONAL_MALFORMED", "not-a-number");

        let error = read_optional::<u16>("MARGARET_OPTIONAL_MALFORMED")
            .expect_err("an unparseable value is rejected");

        assert!(matches!(
            error,
            EnvironmentVariableError::Malformed {
                ref name,
                value_type,
                ..
            } if name == "MARGARET_OPTIONAL_MALFORMED" && value_type == "u16"
        ));
        assert!(!error.to_string().contains("not-a-number"));
    }

    #[cfg(unix)]
    #[test]
    fn reports_a_value_that_is_not_unicode() {
        // SAFETY: `cargo nextest` runs every test in its own process, so no
        // other thread observes the process environment while it is mutated.
        unsafe {
            env::set_var(
                "MARGARET_OPTIONAL_NOT_UNICODE",
                OsStr::from_bytes(&[0x66, 0x80, 0x6f]),
            );
        };

        assert!(matches!(
            read_optional::<String>("MARGARET_OPTIONAL_NOT_UNICODE")
                .expect_err("a non-unicode value is rejected"),
            EnvironmentVariableError::NotUnicode { ref name }
                if name == "MARGARET_OPTIONAL_NOT_UNICODE"
        ));
    }
}
