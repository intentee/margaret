#[singleton]
struct Consumer {
    roller: std::sync::Arc<crate::KeyRoller>,
}

impl Consumer {
    #[constructor]
    fn create(roller: std::sync::Arc<crate::KeyRoller>) -> anyhow::Result<Self> {}
}

pub struct KeyRoller;
