use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Malformation {
    Absent,
    Unreadable,
}

impl Display for Malformation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Absent => "the request input is absent",
            Self::Unreadable => "the request input could not be interpreted",
        };

        formatter.write_str(message)
    }
}

#[cfg(test)]
mod tests {
    use super::Malformation;

    #[test]
    fn describes_an_absent_input() {
        assert_eq!(
            Malformation::Absent.to_string(),
            "the request input is absent"
        );
    }

    #[test]
    fn describes_an_unreadable_input() {
        assert_eq!(
            Malformation::Unreadable.to_string(),
            "the request input could not be interpreted"
        );
    }
}
