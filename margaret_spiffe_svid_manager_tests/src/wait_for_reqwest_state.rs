use reqwest::Client;

use margaret_spiffe_svid_manager::reqwest_client_holder::ReqwestClientHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

pub async fn wait_for_reqwest_state(
    subscription: &mut SyncHolderSubscription<Client>,
    expect_some: bool,
    reqwest_client_holder: &ReqwestClientHolder,
) {
    loop {
        subscription.changed().await;

        if reqwest_client_holder.get().is_some() == expect_some {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future as _;
    use std::task::Context;
    use std::task::Waker;

    use margaret_spiffe_svid_manager::reqwest_client_holder::ReqwestClientHolder;
    use reqwest::Client;

    use super::wait_for_reqwest_state;

    #[tokio::test]
    async fn keeps_waiting_past_states_that_do_not_match() {
        let reqwest_client_holder = ReqwestClientHolder::default();
        let mut subscription = reqwest_client_holder.subscribe();

        let mut context = Context::from_waker(Waker::noop());
        let mut wait_future = std::pin::pin!(wait_for_reqwest_state(
            &mut subscription,
            true,
            &reqwest_client_holder,
        ));

        reqwest_client_holder.set(None);
        assert!(wait_future.as_mut().poll(&mut context).is_pending());

        reqwest_client_holder.set(Some(Client::new()));
        assert!(wait_future.as_mut().poll(&mut context).is_ready());
    }
}
