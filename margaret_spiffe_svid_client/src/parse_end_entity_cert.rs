use log::warn;
use rustls::CertificateError;
use rustls::Error;
use rustls::pki_types::CertificateDer;
use webpki::EndEntityCert;

pub fn parse_end_entity_cert<'cert>(
    end_entity: &'cert CertificateDer<'cert>,
) -> Result<EndEntityCert<'cert>, Error> {
    EndEntityCert::try_from(end_entity).map_err(|err| {
        warn!("Error converting into end entity cert: {err:#?}");

        Error::InvalidCertificate(CertificateError::BadEncoding)
    })
}
