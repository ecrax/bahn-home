use crate::{Requester, RequesterBuilder};
use async_trait::async_trait;
use hyper::client::HttpConnector;
use hyper::{Body, Method, Request};
use hyper_rustls::builderstates::WantsProtocols2;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use std::collections::HashMap;

#[derive(Clone)]
pub struct HyperRustlsRequester(hyper::Client<HttpsConnector<HttpConnector>, Body>);

#[cfg_attr(feature = "rt-multi-thread", async_trait)]
#[cfg_attr(not(feature = "rt-multi-thread"), async_trait(?Send))]
impl Requester for HyperRustlsRequester {
    type Error = HyperRustlsRequesterError;

    async fn get(
        &self,
        url: &url::Url,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<Vec<u8>, Self::Error> {
        self.request(Method::GET, url, body, headers).await
    }

    async fn post(
        &self,
        url: &url::Url,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<Vec<u8>, Self::Error> {
        self.request(Method::POST, url, body, headers).await
    }
}

pub struct HyperRustlsRequesterBuilder(
    hyper::client::Builder,
    HttpsConnectorBuilder<WantsProtocols2>,
);

impl RequesterBuilder for HyperRustlsRequesterBuilder {
    type Requester = HyperRustlsRequester;

    fn with_pem_bundle(mut self, bytes: &[u8]) -> Self {
        let mut bytes = &bytes[..];
        // TODO: This only allows calling this function once.
        self.1 = HttpsConnectorBuilder::new()
            .with_tls_config({
                let mut store = rustls::RootCertStore::empty();
                let certs = rustls_pemfile::certs(&mut bytes);
                for cert in certs.into_iter() {
                    if let Ok(cert) = cert {
                        if let Err(e) = store.add(&rustls::Certificate(cert.to_vec())) {
                            log::error!("Failed to add certificate: {}", e);
                        }
                    } else {
                        log::error!("Failed to read certificate");
                    }
                }

                rustls::ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(store)
                    .with_no_client_auth()
            })
            .https_or_http()
            .enable_http1();
        self
    }

    fn build(self) -> Self::Requester {
        let https = self.1.build();
        let client = self.0.build(https);
        HyperRustlsRequester(client)
    }
}

impl Default for HyperRustlsRequesterBuilder {
    fn default() -> Self {
        Self(
            hyper::Client::builder(),
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1(),
        )
    }
}

impl HyperRustlsRequester {
    pub fn new() -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build(https);
        Self(client)
    }

    async fn request(
        &self,
        method: hyper::Method,
        url: &url::Url,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<Vec<u8>, HyperRustlsRequesterError> {
        log::trace!(
            "{}: URL: {}, Body: {}, Headers: {:?}",
            method,
            url,
            String::from_utf8_lossy(body),
            headers
        );
        let body = body.to_vec();
        let mut req = Request::builder().method(method).uri(url.as_str());

        for (k, v) in headers {
            req = req.header(k, v);
        }

        let req = req.body(Body::from(body)).unwrap();

        let (parts, resp_body) = self.0.request(req).await?.into_parts();
        let bytes = hyper::body::to_bytes(resp_body).await?;

        if parts.status.is_success() {
            Ok(bytes.to_vec())
        } else {
            Err(HyperRustlsRequesterError::NoSuccessStatusCode(
                parts.status.as_u16(),
                parts.status.canonical_reason(),
                bytes.to_vec(),
            ))
        }
    }
}

impl Default for HyperRustlsRequester {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum HyperRustlsRequesterError {
    /// hyper failed.
    Hyper(hyper::Error),
    /// Got a status code which is no success.
    /// Contains the status code, the "canonical reason" and the body bytes.
    NoSuccessStatusCode(u16, Option<&'static str>, Vec<u8>),
}

impl std::fmt::Display for HyperRustlsRequesterError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Hyper(e) => write!(fmt, "hyper error: {}", e),
            Self::NoSuccessStatusCode(code, Some(reason), _) => {
                write!(fmt, "unsuccessful status code {} ({})", code, reason)
            }
            Self::NoSuccessStatusCode(code, None, _) => {
                write!(fmt, "unsuccessful status code {}", code)
            }
        }
    }
}

impl std::error::Error for HyperRustlsRequesterError {}

impl From<hyper::Error> for HyperRustlsRequesterError {
    fn from(e: hyper::Error) -> HyperRustlsRequesterError {
        Self::Hyper(e)
    }
}
