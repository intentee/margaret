use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Audience {
    One(String),
    Many(Vec<String>),
}

impl Audience {
    #[must_use]
    pub fn contains(&self, audience: &str) -> bool {
        match self {
            Self::One(declared) => declared == audience,
            Self::Many(declared) => declared.iter().any(|entry| entry == audience),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Audience;

    #[test]
    fn recognizes_the_single_declared_audience() {
        let audience = Audience::One("dashboard".to_string());

        assert!(audience.contains("dashboard"));
        assert!(!audience.contains("another-service"));
    }

    #[test]
    fn recognizes_any_of_the_declared_audiences() {
        let audience = Audience::Many(vec!["dashboard".to_string(), "reports".to_string()]);

        assert!(audience.contains("reports"));
        assert!(!audience.contains("another-service"));
    }

    #[test]
    fn declares_no_audience_when_the_list_is_empty() {
        assert!(!Audience::Many(Vec::new()).contains("dashboard"));
    }
}
