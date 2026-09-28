use std::error::Error;
use std::str::FromStr;

use crate::environment_variable_error::EnvironmentVariableError;
use crate::read_optional::read_optional;

/// # Errors
///
/// Returns `EnvironmentVariableError::Missing`, or an error propagated from
/// reading the variable.
pub fn read_required<Value>(name: &str) -> Result<Value, EnvironmentVariableError>
where
    Value: FromStr,
    <Value as FromStr>::Err: Error + Send + Sync + 'static,
{
    read_optional::<Value>(name)?.ok_or_else(|| EnvironmentVariableError::Missing {
        name: name.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::environment_variable_error::EnvironmentVariableError;

    use super::read_required;

    #[test]
    fn reads_a_present_value() {
        // SAFETY: `cargo nextest` runs every test in its own process, so no
        // other thread observes the process environment while it is mutated.
        unsafe { env::set_var("MARGARET_REQUIRED_PRESENT", "true") };

        assert!(read_required::<bool>("MARGARET_REQUIRED_PRESENT").expect("the variable parses"));
    }

    #[test]
    fn reports_an_absent_variable() {
        assert!(matches!(
            read_required::<String>("MARGARET_REQUIRED_ABSENT")
                .expect_err("an absent variable is rejected"),
            EnvironmentVariableError::Missing { ref name }
                if name == "MARGARET_REQUIRED_ABSENT"
        ));
    }

    #[test]
    fn propagates_a_value_that_does_not_parse() {
        // SAFETY: `cargo nextest` runs every test in its own process, so no
        // other thread observes the process environment while it is mutated.
        unsafe { env::set_var("MARGARET_REQUIRED_MALFORMED", "maybe") };

        assert!(matches!(
            read_required::<bool>("MARGARET_REQUIRED_MALFORMED")
                .expect_err("an unparseable value is rejected"),
            EnvironmentVariableError::Malformed { ref name, .. }
                if name == "MARGARET_REQUIRED_MALFORMED"
        ));
    }
}
