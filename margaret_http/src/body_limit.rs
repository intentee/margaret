#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BodyLimit {
    max_bytes: usize,
}

impl BodyLimit {
    #[must_use]
    pub const fn new(max_bytes: usize) -> Self {
        Self { max_bytes }
    }

    #[must_use]
    pub fn admits(&self, declared_bytes: u64) -> bool {
        usize::try_from(declared_bytes).is_ok_and(|declared_bytes| declared_bytes <= self.max_bytes)
    }

    #[must_use]
    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::BodyLimit;

    #[test]
    fn admits_a_body_up_to_its_limit() {
        assert!(BodyLimit::new(16).admits(16));
    }

    #[test]
    fn refuses_a_body_over_its_limit() {
        assert!(!BodyLimit::new(16).admits(17));
    }

    #[test]
    fn reports_its_configured_max_bytes() {
        assert_eq!(BodyLimit::new(16).max_bytes(), 16);
    }
}
