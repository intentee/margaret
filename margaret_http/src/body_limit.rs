const DEFAULT_MAX_BODY_SIZE: u64 = 8 * 1024 * 1024;

#[derive(Clone, Copy)]
pub struct BodyLimit {
    max_bytes: u64,
}

impl Default for BodyLimit {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MAX_BODY_SIZE,
        }
    }
}

impl BodyLimit {
    #[must_use]
    pub fn new(max_bytes: u64) -> Self {
        Self { max_bytes }
    }

    #[must_use]
    pub fn max_bytes(&self) -> u64 {
        self.max_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::BodyLimit;
    use super::DEFAULT_MAX_BODY_SIZE;

    #[test]
    fn defaults_to_the_framework_body_size() {
        assert_eq!(BodyLimit::default().max_bytes(), DEFAULT_MAX_BODY_SIZE);
    }

    #[test]
    fn reports_its_configured_max_bytes() {
        assert_eq!(BodyLimit::new(16).max_bytes(), 16);
    }
}
