use crate::query_edge::QueryEdge;

pub(crate) struct QueryEdges<'model> {
    pub(crate) first: Box<QueryEdge<'model>>,
    pub(crate) rest: Vec<QueryEdge<'model>>,
}

impl<'model> QueryEdges<'model> {
    pub(crate) fn all(&self) -> impl Iterator<Item = &QueryEdge<'model>> {
        [self.first.as_ref()].into_iter().chain(self.rest.iter())
    }
}
