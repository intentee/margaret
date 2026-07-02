use std::path::PathBuf;

const DEFAULT_MAX_BODY_SIZE: u64 = 8 * 1024 * 1024;

pub enum UploadConfig {
    Disabled,
    Enabled { directory: PathBuf, max_size: u64 },
}

impl UploadConfig {
    pub fn enabled(directory: PathBuf) -> Self {
        Self::Enabled {
            directory,
            max_size: DEFAULT_MAX_BODY_SIZE,
        }
    }

    pub fn max_body_size(&self) -> u64 {
        match self {
            Self::Disabled => DEFAULT_MAX_BODY_SIZE,
            Self::Enabled { max_size, .. } => *max_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::DEFAULT_MAX_BODY_SIZE;
    use super::UploadConfig;

    #[test]
    fn a_disabled_config_caps_bodies_at_the_default_size() {
        assert_eq!(
            UploadConfig::Disabled.max_body_size(),
            DEFAULT_MAX_BODY_SIZE
        );
    }

    #[test]
    fn an_enabled_config_defaults_to_the_default_body_size() {
        assert_eq!(
            UploadConfig::enabled(PathBuf::from("/tmp")).max_body_size(),
            DEFAULT_MAX_BODY_SIZE
        );
    }

    #[test]
    fn an_enabled_config_reports_its_configured_max_size() {
        let upload_config = UploadConfig::Enabled {
            directory: PathBuf::from("/tmp"),
            max_size: 16,
        };

        assert_eq!(upload_config.max_body_size(), 16);
    }
}
