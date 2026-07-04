use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::Handler;
use crate::router::Router;

pub struct RouterBuilder {
    matcher: matchit::Router<usize>,
    paths: HashMap<String, usize>,
    routes: Vec<HashMap<&'static str, Arc<dyn Handler>>>,
}

impl RouterBuilder {
    pub fn empty() -> Self {
        Self {
            matcher: matchit::Router::new(),
            paths: HashMap::new(),
            routes: Vec::new(),
        }
    }

    pub fn build(self) -> Router {
        Router::new(self.matcher, self.routes)
    }

    pub fn route(mut self, method: &'static str, path: &str, handler: Arc<dyn Handler>) -> Self {
        let index = match self.paths.get(path) {
            Some(&index) => index,
            None => {
                let index = self.routes.len();

                self.routes.push(HashMap::new());
                self.matcher
                    .insert(path.to_string(), index)
                    .expect("a well-formed route path");
                self.paths.insert(path.to_string(), index);

                index
            }
        };

        self.routes[index].insert(method, handler);

        self
    }
}
