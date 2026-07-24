#[singleton]
struct Widget;

#[singleton]
struct Consumer {
    widget: std::sync::Arc<crate::Widget>,
}

impl Consumer {
    #[constructor]
    fn create(widget: std::sync::Arc<crate::Widget>) -> Self {}
}
