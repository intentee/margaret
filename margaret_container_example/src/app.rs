use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::greeter::Greeter;
use crate::plugins::Plugin;

#[singleton]
pub struct App {
    greeter: Arc<dyn Greeter + Send + Sync>,
    plugins: Vec<Arc<dyn Plugin + Send + Sync>>,
}

impl App {
    #[constructor]
    pub fn create(
        greeter: Arc<dyn Greeter + Send + Sync>,
        plugins: Vec<Arc<dyn Plugin + Send + Sync>>,
    ) -> Self {
        Self { greeter, plugins }
    }

    pub fn describe(&self) -> String {
        let mut description = self.greeter.greet();

        for plugin in &self.plugins {
            description.push('+');
            description.push_str(&plugin.name());
        }

        description
    }
}
