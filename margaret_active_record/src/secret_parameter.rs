use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use bytes::BytesMut;
use tokio_postgres::types::IsNull;
use tokio_postgres::types::ToSql;
use tokio_postgres::types::Type;
use tokio_postgres::types::to_sql_checked;
use zeroize::Zeroizing;

pub(crate) struct SecretParameter {
    pub(crate) text: Zeroizing<String>,
}

impl Debug for SecretParameter {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretParameter")
    }
}

impl ToSql for SecretParameter {
    fn to_sql(
        &self,
        column_type: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.text.as_str().to_sql(column_type, out)
    }

    fn accepts(column_type: &Type) -> bool {
        <&str as ToSql>::accepts(column_type)
    }

    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use zeroize::Zeroizing;

    use super::SecretParameter;

    #[test]
    fn describes_itself_without_its_contents() {
        assert_eq!(
            format!(
                "{:?}",
                SecretParameter {
                    text: Zeroizing::new("private key material".to_string()),
                }
            ),
            "SecretParameter"
        );
    }
}
