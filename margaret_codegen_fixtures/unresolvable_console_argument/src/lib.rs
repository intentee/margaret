#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "widget")] widget: Widget) -> Self {}
}
