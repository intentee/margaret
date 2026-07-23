trait Widget {}

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "shared")] shared: String) -> Self {}
}

#[singleton(collection = Widget, provides = Widget)]
#[console_command(name = "widget")]
struct WidgetPlugin;

impl WidgetPlugin {
    #[constructor]
    fn create(
        config: std::sync::Arc<Config>,
        #[console_argument(positional)] shared: String,
    ) -> Self {
    }
}

impl Widget for WidgetPlugin {}

#[singleton]
struct Aggregator;

impl Aggregator {
    #[constructor]
    fn create(widgets: Vec<std::sync::Arc<dyn Widget>>) -> Self {}
}
