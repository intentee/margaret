use std::ops::ControlFlow;

use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::compact_jws_parsing::CompactJwsParsing;
use crate::header_type::HeaderType;
use crate::jws_header::JwsHeader;
use crate::jws_rejection::JwsRejection;
use crate::parameter_value::ParameterValue;

fn decoded(
    segment: &str,
    rejection: impl FnOnce(base64ct::Error) -> JwsRejection,
) -> ControlFlow<JwsRejection, Vec<u8>> {
    match Base64UrlUnpadded::decode_vec(segment) {
        Ok(bytes) => ControlFlow::Continue(bytes),
        Err(source) => ControlFlow::Break(rejection(source)),
    }
}

pub struct CompactJws<'token> {
    pub(crate) header: JwsHeader,
    pub(crate) payload: Vec<u8>,
    pub(crate) signature: Vec<u8>,
    pub(crate) signing_input: &'token str,
}

impl<'token> CompactJws<'token> {
    #[must_use]
    pub fn parse(token: &'token str) -> CompactJwsParsing<'token> {
        match Self::parsed(token) {
            ControlFlow::Continue(jws) => CompactJwsParsing::Parsed(jws),
            ControlFlow::Break(rejection) => CompactJwsParsing::Rejected(rejection),
        }
    }

    fn parsed(token: &'token str) -> ControlFlow<JwsRejection, Self> {
        let Some((signing_input, signature_segment)) = token.rsplit_once('.') else {
            return ControlFlow::Break(JwsRejection::NotCompactJws);
        };
        let Some((header_segment, payload_segment)) = signing_input.split_once('.') else {
            return ControlFlow::Break(JwsRejection::NotCompactJws);
        };

        if payload_segment.contains('.') {
            return ControlFlow::Break(JwsRejection::NotCompactJws);
        }

        let header_bytes = decoded(header_segment, |source| JwsRejection::HeaderBase64 {
            source,
        })?;
        let header = match serde_json::from_slice(&header_bytes) {
            Ok(header) => header,
            Err(source) => return ControlFlow::Break(JwsRejection::HeaderMalformed { source }),
        };
        let payload = decoded(payload_segment, |source| JwsRejection::PayloadBase64 {
            source,
        })?;
        let signature = decoded(signature_segment, |source| JwsRejection::SignatureBase64 {
            source,
        })?;

        ControlFlow::Continue(Self {
            header,
            payload,
            signature,
            signing_input,
        })
    }

    #[must_use]
    pub fn alg(&self) -> &ParameterValue<JwsAlgorithm> {
        &self.header.alg
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub fn typ(&self) -> Option<&HeaderType> {
        self.header.typ.as_ref()
    }
}
