use margaret_security::actor::Actor;

use crate::models::role::Role;

pub struct User {
    pub id: String,
    pub name: String,
    pub role: Role,
}

impl Actor for User {
    type Role = Role;

    fn identifier(&self) -> &str {
        &self.id
    }

    fn role(&self) -> Role {
        self.role
    }
}
