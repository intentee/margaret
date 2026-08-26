#[derive(Clone, Copy)]
struct Budget(u32);

#[singleton]
struct Child;

impl Child {
    #[constructor]
    fn create(
        #[console_argument(from = "retries")] retries: u16,
        #[console_argument(from = "max-connections")] max_connections: std::num::NonZeroU32,
        #[console_argument(from = "budget")] budget: crate::Budget,
    ) -> anyhow::Result<Self> {
    }
}

#[singleton]
struct Parent;

impl Parent {
    #[constructor]
    fn create(
        #[console_argument(from = "retries")] retries: u16,
        #[console_argument(from = "max-connections")] max_connections: std::num::NonZeroU32,
        #[console_argument(from = "budget")] budget: crate::Budget,
        child: std::sync::Arc<Child>,
    ) -> anyhow::Result<Self> {
    }
}
