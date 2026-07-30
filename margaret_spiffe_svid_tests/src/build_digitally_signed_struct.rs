use rustls::DigitallySignedStruct;
use rustls::SignatureScheme;
use rustls::internal::msgs::codec::Codec as _;
use rustls::internal::msgs::codec::Reader;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_digitally_signed_struct(
    scheme: SignatureScheme,
    signature: &[u8],
) -> DigitallySignedStruct {
    let mut bytes = Vec::new();

    scheme.encode(&mut bytes);
    (u16::try_from(signature.len()).unwrap()).encode(&mut bytes);
    bytes.extend_from_slice(signature);

    let mut reader = Reader::init(&bytes);

    DigitallySignedStruct::read(&mut reader).unwrap()
}
