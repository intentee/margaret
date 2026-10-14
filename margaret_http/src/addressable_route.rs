use std::marker::PhantomData;
use std::sync::Arc;

use crate::build_url::build_url;
use crate::route_addressing::RouteAddressing;
use crate::route_position::RoutePosition;
use crate::route_props::RouteProps;

pub struct AddressableRoute<TProps, TReference> {
    matcher: Arc<matchit::Router<RoutePosition>>,
    origin: Arc<str>,
    position: RoutePosition,
    props: PhantomData<fn(TProps)>,
    reference: fn(String) -> TReference,
}

impl<TProps, TReference> AddressableRoute<TProps, TReference> {
    pub(crate) fn new(
        matcher: Arc<matchit::Router<RoutePosition>>,
        origin: Arc<str>,
        position: RoutePosition,
        reference: fn(String) -> TReference,
    ) -> Self {
        Self {
            matcher,
            origin,
            position,
            props: PhantomData,
            reference,
        }
    }
}

impl<TProps: RouteProps, TReference> AddressableRoute<TProps, TReference> {
    pub fn address(&self, props: TProps) -> RouteAddressing<TReference> {
        build_url(
            &self.origin,
            &self.matcher,
            self.position,
            props.into_segments().as_ref(),
        )
        .map(self.reference)
    }
}
