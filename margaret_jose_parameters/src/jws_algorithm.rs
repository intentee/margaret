use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

const WIRE_NAMES: [&str; 3] = ["ES256", "ES384", "RS256"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JwsAlgorithm {
    Es256,
    Es384,
    Rs256,
}

impl JwsAlgorithm {
    pub const ALL: [Self; 3] = [Self::Es256, Self::Es384, Self::Rs256];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Es256 => WIRE_NAMES[0],
            Self::Es384 => WIRE_NAMES[1],
            Self::Rs256 => WIRE_NAMES[2],
        }
    }
}

impl Display for JwsAlgorithm {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self.wire_name())
    }
}

impl Serialize for JwsAlgorithm {
    fn serialize<Target: Serializer>(
        &self,
        serializer: Target,
    ) -> std::result::Result<Target::Ok, Target::Error> {
        serializer.serialize_str(self.wire_name())
    }
}

impl<'wire> Deserialize<'wire> for JwsAlgorithm {
    fn deserialize<Source: Deserializer<'wire>>(
        deserializer: Source,
    ) -> std::result::Result<Self, Source::Error> {
        let written = String::deserialize(deserializer)?;

        Self::ALL
            .into_iter()
            .find(|algorithm| algorithm.wire_name() == written)
            .ok_or_else(|| Source::Error::unknown_variant(&written, &WIRE_NAMES))
    }
}

#[cfg(test)]
mod tests {
    use super::JwsAlgorithm;

    #[test]
    fn reads_every_registered_algorithm_by_its_wire_name() {
        for algorithm in JwsAlgorithm::ALL {
            let written = format!("\"{}\"", algorithm.wire_name());

            assert_eq!(
                serde_json::from_str::<JwsAlgorithm>(&written).expect("the wire name parses"),
                algorithm
            );
        }
    }

    #[test]
    fn writes_the_wire_name() {
        assert_eq!(
            serde_json::to_string(&JwsAlgorithm::Es384).expect("the algorithm serializes"),
            "\"ES384\""
        );
    }

    #[test]
    fn rejects_an_algorithm_it_does_not_implement() {
        assert!(serde_json::from_str::<JwsAlgorithm>("\"HS256\"").is_err());
    }

    #[test]
    fn rejects_an_algorithm_that_is_not_a_string() {
        assert!(serde_json::from_str::<JwsAlgorithm>("256").is_err());
    }

    #[test]
    fn displays_the_wire_name() {
        assert_eq!(JwsAlgorithm::Es256.to_string(), "ES256");
    }
}
