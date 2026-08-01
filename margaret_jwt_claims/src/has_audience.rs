use crate::audience::Audience;

pub trait HasAudience {
    fn audience(&self) -> &Audience;
}
