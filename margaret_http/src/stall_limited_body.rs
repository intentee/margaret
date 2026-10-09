use std::io;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bytes::Bytes;
use http_body::Body;
use http_body::Frame;
use http_body::SizeHint;
use http_body_util::combinators::UnsyncBoxBody;

use crate::client_body_timeout::CLIENT_BODY_TIMEOUT;
use crate::stall_outcome::StallOutcome;
use crate::stall_timer::StallTimer;

pub(crate) struct StallLimitedBody {
    body: UnsyncBoxBody<Bytes, io::Error>,
    stall_timer: StallTimer,
}

impl StallLimitedBody {
    pub(crate) fn new(body: UnsyncBoxBody<Bytes, io::Error>) -> Self {
        Self {
            body,
            stall_timer: StallTimer::new(CLIENT_BODY_TIMEOUT),
        }
    }
}

impl Body for StallLimitedBody {
    type Data = Bytes;
    type Error = io::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        let this = self.get_mut();
        let frame = Pin::new(&mut this.body).poll_frame(context);

        this.stall_timer
            .poll_progress(context, frame)
            .map(|outcome| match outcome {
                StallOutcome::Progressed(frame) => frame,
                StallOutcome::Stalled => Some(Err(io::ErrorKind::TimedOut.into())),
            })
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}
