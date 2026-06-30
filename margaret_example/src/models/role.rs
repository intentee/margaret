use margaret_security::actor_role::ActorRole;

#[derive(Clone, Copy)]
pub enum Role {
    Member,
    Moderator,
    Admin,
}

impl ActorRole for Role {
    fn to_int(&self) -> u32 {
        match self {
            Role::Member => 0,
            Role::Moderator => 1,
            Role::Admin => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_security::actor_role::ActorRole;

    use super::Role;

    #[test]
    fn ranks_increase_from_member_to_admin() {
        assert!(Role::Admin.is_at_least(Role::Moderator));
        assert!(Role::Moderator.is_at_least(Role::Member));
        assert!(!Role::Member.is_at_least(Role::Moderator));
        assert!(!Role::Moderator.is_at_least(Role::Admin));
        assert_eq!(Role::Member.to_int(), 0);
        assert_eq!(Role::Moderator.to_int(), 1);
        assert_eq!(Role::Admin.to_int(), 2);
    }
}
