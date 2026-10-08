use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssertionSigning {
    Own,
    Pinned(JwsAlgorithm),
}
