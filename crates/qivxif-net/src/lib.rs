use anyhow::Result;
use quinn::crypto::rustls::QuicClientConfig;
use quinn::rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer, ServerName, UnixTime};
use serde::{Serialize, de::DeserializeOwned};
use std::{net::SocketAddr, sync::Arc};

pub async fn send_json<T: Serialize>(writer: &mut quinn::SendStream, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    writer.write_all(&bytes).await?;
    writer.finish()?;
    Ok(())
}

pub async fn recv_json<T: DeserializeOwned>(reader: &mut quinn::RecvStream) -> Result<T> {
    let bytes = reader.read_to_end(1024 * 1024).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn server_config() -> Result<quinn::ServerConfig> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = CertificateDer::from(cert.cert);
    let key_der = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());
    let mut config = quinn::ServerConfig::with_single_cert(vec![cert_der], key_der.into())?;
    Arc::get_mut(&mut config.transport)
        .expect("new server config owns transport")
        .max_concurrent_uni_streams(0_u8.into());
    Ok(config)
}

pub fn client_endpoint() -> Result<quinn::Endpoint> {
    let bind = SocketAddr::from(([0, 0, 0, 0], 0));
    let mut endpoint = quinn::Endpoint::client(bind)?;
    let crypto = quinn::rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();
    endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(
        QuicClientConfig::try_from(crypto)?,
    )));
    Ok(endpoint)
}

#[derive(Debug)]
struct SkipServerVerification(Arc<quinn::rustls::crypto::CryptoProvider>);

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self(Arc::new(
            quinn::rustls::crypto::ring::default_provider(),
        )))
    }
}

impl quinn::rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<quinn::rustls::client::danger::ServerCertVerified, quinn::rustls::Error> {
        Ok(quinn::rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &quinn::rustls::DigitallySignedStruct,
    ) -> Result<quinn::rustls::client::danger::HandshakeSignatureValid, quinn::rustls::Error> {
        quinn::rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &quinn::rustls::DigitallySignedStruct,
    ) -> Result<quinn::rustls::client::danger::HandshakeSignatureValid, quinn::rustls::Error> {
        quinn::rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<quinn::rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}
