use crate::actor::Actor;

pub struct AuthenticatedActor<ActorType: Actor> {
    pub actor: ActorType,
}

impl<ActorType: Actor> AuthenticatedActor<ActorType> {
    pub fn new(actor: ActorType) -> Self {
        Self { actor }
    }
}

#[cfg(test)]
mod tests {
    use crate::actor::Actor;
    use crate::actor_role::ActorRole;

    use super::AuthenticatedActor;

    #[derive(Clone, Copy)]
    struct Rank;

    impl ActorRole for Rank {
        fn to_int(&self) -> u32 {
            1
        }
    }

    struct Person {
        id: String,
    }

    impl Actor for Person {
        type Role = Rank;

        fn identifier(&self) -> &str {
            &self.id
        }

        fn role(&self) -> Rank {
            Rank
        }
    }

    #[test]
    fn exposes_the_actor_through_a_public_field() {
        let authenticated = AuthenticatedActor::new(Person {
            id: "u-1".to_string(),
        });

        assert_eq!(authenticated.actor.identifier(), "u-1");
        assert_eq!(authenticated.actor.role().to_int(), 1);
    }

    #[test]
    fn destructures_into_the_owned_actor() {
        let AuthenticatedActor { actor } = AuthenticatedActor::new(Person {
            id: "u-2".to_string(),
        });

        assert_eq!(actor.identifier(), "u-2");
    }
}
