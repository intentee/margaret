use std::sync::Arc;

use margaret_http::route_parameter_binder::RouteParameterBinder;
use margaret_macros::constructor;
use margaret_macros::route_parameter_binder;
use margaret_macros::singleton;

use crate::models::user::User;
use crate::user_repository::UserRepository;

#[singleton]
#[route_parameter_binder]
pub struct UserBinder {
    repository: Arc<UserRepository>,
}

impl UserBinder {
    #[constructor]
    pub fn create(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }
}

impl RouteParameterBinder for UserBinder {
    type Model = User;

    async fn bind(&self, value: String) -> Option<User> {
        self.repository.find(&value)
    }
}
