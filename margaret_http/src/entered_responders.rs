use std::collections::HashSet;
use std::sync::Arc;

fn address_of<THandler: ?Sized>(handler: &Arc<THandler>) -> usize {
    Arc::as_ptr(handler).cast::<()>().addr()
}

#[derive(Default)]
pub struct EnteredResponders {
    addresses: HashSet<usize>,
}

impl EnteredResponders {
    pub(crate) fn starting_at<THandler: ?Sized>(handler: &Arc<THandler>) -> Self {
        Self {
            addresses: HashSet::from([address_of(handler)]),
        }
    }

    pub(crate) fn enter<THandler: ?Sized>(&mut self, handler: &Arc<THandler>) -> bool {
        self.addresses.insert(address_of(handler))
    }
}
