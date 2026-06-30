pub trait ActorRole: Copy {
    fn to_int(&self) -> u32;

    fn is_at_least(&self, other: Self) -> bool {
        self.to_int() >= other.to_int()
    }
}

#[cfg(test)]
mod tests {
    use super::ActorRole;

    #[derive(Clone, Copy)]
    enum Rank {
        Member,
        Admin,
    }

    impl ActorRole for Rank {
        fn to_int(&self) -> u32 {
            match self {
                Rank::Member => 0,
                Rank::Admin => 1,
            }
        }
    }

    #[test]
    fn a_higher_rank_is_at_least_a_lower_one() {
        assert!(Rank::Admin.is_at_least(Rank::Member));
        assert!(Rank::Admin.is_at_least(Rank::Admin));
    }

    #[test]
    fn a_lower_rank_is_not_at_least_a_higher_one() {
        assert!(!Rank::Member.is_at_least(Rank::Admin));
    }
}
