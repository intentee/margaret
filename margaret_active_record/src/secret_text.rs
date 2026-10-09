use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use zeroize::Zeroizing;

use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;
use crate::secret_parameter::SecretParameter;
use crate::value::Value;

#[derive(Clone)]
pub struct SecretText {
    text: Zeroizing<String>,
}

impl SecretText {
    #[must_use]
    pub fn new(text: Zeroizing<String>) -> Self {
        Self { text }
    }

    #[must_use]
    pub fn expose(&self) -> &str {
        &self.text
    }
}

impl Debug for SecretText {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretText")
    }
}

impl Value for SecretText {
    const WIDTH: usize = 1;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        cursor
            .read::<String>()
            .map(|text| Self::new(Zeroizing::new(text)))
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        parameters.push(SecretParameter {
            text: self.text.clone(),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use zeroize::Zeroizing;

    use super::SecretText;

    #[test]
    fn describes_itself_without_its_contents() {
        assert_eq!(
            format!(
                "{:?}",
                SecretText::new(Zeroizing::new("private key material".to_string()))
            ),
            "SecretText"
        );
    }
}
