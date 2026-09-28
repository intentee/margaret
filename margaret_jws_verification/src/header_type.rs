use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Deserializer;

use margaret_jose_parameters::jwt_type::JwtType;

const MEDIA_TYPE_PREFIX: &str = "application/";

fn media_subtype(written: &str) -> &str {
    match written.split_at_checked(MEDIA_TYPE_PREFIX.len()) {
        Some((prefix, subtype)) if prefix.eq_ignore_ascii_case(MEDIA_TYPE_PREFIX) => subtype,
        _ => written,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeaderType {
    Supported(JwtType),
    Unsupported(String),
}

impl Display for HeaderType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Supported(jwt_type) => jwt_type.fmt(formatter),
            Self::Unsupported(written) => formatter.write_str(written),
        }
    }
}

impl<'wire> Deserialize<'wire> for HeaderType {
    fn deserialize<Source: Deserializer<'wire>>(
        deserializer: Source,
    ) -> std::result::Result<Self, Source::Error> {
        let written = String::deserialize(deserializer)?;
        let subtype = media_subtype(&written);

        Ok(JwtType::ALL
            .into_iter()
            .find(|jwt_type| jwt_type.wire_name().eq_ignore_ascii_case(subtype))
            .map_or(Self::Unsupported(written), Self::Supported))
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::jwt_type::JwtType;

    use super::HeaderType;

    #[test]
    fn displays_a_supported_type_by_its_wire_name() {
        assert_eq!(HeaderType::Supported(JwtType::Jwt).to_string(), "JWT");
    }

    #[test]
    fn displays_an_unsupported_type_as_written() {
        assert_eq!(
            HeaderType::Unsupported("dpop+jwt".to_string()).to_string(),
            "dpop+jwt"
        );
    }
}
