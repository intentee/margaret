use std::pin::Pin;

use anyhow::Error;
use futures_util::Stream;
use futures_util::stream::StreamExt as _;
use log::error;
use log::warn;
use spiffe::X509Context;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

pub async fn svid_rotate_loop(
    mut stream: Pin<Box<dyn Stream<Item = Result<X509Context, Error>> + Send>>,
    x509_context_tx: &Sender<X509Context>,
    cancellation_token: CancellationToken,
) {
    loop {
        tokio::select! {
            () = cancellation_token.cancelled() => {
                break;
            },
            x509_context_update = stream.next() => {
                match x509_context_update {
                    Some(Ok(x509_context)) => {
                        if let Err(err) = x509_context_tx.send(x509_context) {
                            error!("Unable to broadcast updated SVID context: {err:#?}");
                        }
                    }
                    Some(Err(err)) => {
                        error!("SPIRE client failed to fetch a new context update: {err:#?}");

                        break;
                    }
                    None => {
                        warn!("SPIRE context updates stream ended");

                        break;
                    }
                }
            }
        }
    }
}
