use crate::named_item::NamedItem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemNamingArgument {
    ForeignKeyReferences,
    TickBehavior,
    TickInterval,
    UserModel,
}

impl ItemNamingArgument {
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::ForeignKeyReferences => "references",
            Self::TickBehavior => "behavior",
            Self::TickInterval => "interval",
            Self::UserModel => "user_model",
        }
    }

    #[must_use]
    pub fn named_item(self) -> NamedItem {
        match self {
            Self::ForeignKeyReferences | Self::UserModel => NamedItem::Type,
            Self::TickBehavior | Self::TickInterval => NamedItem::Value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ItemNamingArgument;
    use crate::named_item::NamedItem;

    #[test]
    fn a_model_argument_names_a_type() {
        assert_eq!(
            [
                ItemNamingArgument::ForeignKeyReferences,
                ItemNamingArgument::UserModel,
            ]
            .map(ItemNamingArgument::named_item),
            [NamedItem::Type, NamedItem::Type]
        );
    }

    #[test]
    fn a_tick_timer_argument_names_a_value() {
        assert_eq!(
            [
                ItemNamingArgument::TickBehavior,
                ItemNamingArgument::TickInterval,
            ]
            .map(ItemNamingArgument::named_item),
            [NamedItem::Value, NamedItem::Value]
        );
    }
}
