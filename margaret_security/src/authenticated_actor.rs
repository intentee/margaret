use crate::actor::Actor;

pub enum AuthenticatedActor<ActorType: Actor> {
    Anonymous,
    Session(ActorType),
}
