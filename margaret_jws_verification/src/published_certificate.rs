use std::ops::ControlFlow;

use aws_lc_rs::digest;
use base64ct::Base64;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use rustls_pki_types::CertificateDer;
use webpki::EndEntityCert;

use crate::certificate_rejection::CertificateRejection;
use crate::verification_material::VerificationMaterial;

fn confirm_thumbprint(
    thumbprint: Option<String>,
    algorithm: &'static digest::Algorithm,
    certificate: &[u8],
    unreadable: impl FnOnce(base64ct::Error) -> CertificateRejection,
    mismatch: CertificateRejection,
) -> ControlFlow<CertificateRejection> {
    let Some(thumbprint) = thumbprint else {
        return ControlFlow::Continue(());
    };

    match Base64UrlUnpadded::decode_vec(&thumbprint) {
        Ok(expected) if expected == digest::digest(algorithm, certificate).as_ref() => {
            ControlFlow::Continue(())
        }
        Ok(_) => ControlFlow::Break(mismatch),
        Err(source) => ControlFlow::Break(unreadable(source)),
    }
}

pub(crate) struct PublishedCertificate {
    pub(crate) chain: Vec<String>,
    pub(crate) sha1_thumbprint: Option<String>,
    pub(crate) sha256_thumbprint: Option<String>,
}

impl PublishedCertificate {
    pub(crate) fn attests(
        self,
        material: &VerificationMaterial,
    ) -> ControlFlow<CertificateRejection> {
        let Self {
            chain,
            sha1_thumbprint,
            sha256_thumbprint,
        } = self;
        let Some(first) = chain.into_iter().next() else {
            return ControlFlow::Break(CertificateRejection::EmptyChain);
        };
        let certificate = match Base64::decode_vec(&first) {
            Ok(certificate) => CertificateDer::from(certificate),
            Err(source) => {
                return ControlFlow::Break(CertificateRejection::CertificateBase64 { source });
            }
        };
        let end_entity = match EndEntityCert::try_from(&certificate) {
            Ok(end_entity) => end_entity,
            Err(source) => {
                return ControlFlow::Break(CertificateRejection::CertificateMalformed { source });
            }
        };

        material.attested_by(end_entity.subject_public_key_info().as_ref())?;
        confirm_thumbprint(
            sha1_thumbprint,
            &digest::SHA1_FOR_LEGACY_USE_ONLY,
            &certificate,
            |source| CertificateRejection::Sha1ThumbprintBase64 { source },
            CertificateRejection::Sha1ThumbprintMismatch,
        )?;
        confirm_thumbprint(
            sha256_thumbprint,
            &digest::SHA256,
            &certificate,
            |source| CertificateRejection::Sha256ThumbprintBase64 { source },
            CertificateRejection::Sha256ThumbprintMismatch,
        )
    }
}
