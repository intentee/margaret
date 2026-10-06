pub(crate) enum StallOutcome<TOutput> {
    Progressed(TOutput),
    Stalled,
}
