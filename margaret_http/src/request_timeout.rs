use std::time::Duration;

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy)]
pub struct RequestTimeout {
    duration: Duration,
}

impl Default for RequestTimeout {
    fn default() -> Self {
        Self {
            duration: DEFAULT_REQUEST_TIMEOUT,
        }
    }
}

impl RequestTimeout {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    #[must_use]
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::DEFAULT_REQUEST_TIMEOUT;
    use super::RequestTimeout;

    #[test]
    fn defaults_to_the_framework_request_timeout() {
        assert_eq!(
            RequestTimeout::default().duration(),
            DEFAULT_REQUEST_TIMEOUT
        );
    }

    #[test]
    fn reports_its_configured_duration() {
        assert_eq!(
            RequestTimeout::new(Duration::from_millis(25)).duration(),
            Duration::from_millis(25)
        );
    }
}
