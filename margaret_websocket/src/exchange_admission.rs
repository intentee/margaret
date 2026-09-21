use crate::served_exchange::ServedExchange;

pub(crate) enum ExchangeAdmission {
    Admitted(ServedExchange),
    AlreadyOpen,
}
