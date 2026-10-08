#[singleton]
struct Consumer {
    roller: std::sync::Arc<crate::KeyRoller>,
}

impl Consumer {
    #[constructor]
    fn create(roller: std::sync::Arc<crate::KeyRoller>) -> anyhow::Result<Self> {}
}

#[postgres_database(url_from = "KEY_DATABASE_URL")]
struct KeyDatabase;

pub struct KeyRoller;
