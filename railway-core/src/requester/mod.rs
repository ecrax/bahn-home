#[cfg(feature = "hyper-requester")]
mod hyper;
#[cfg(feature = "hyper-requester")]
pub use hyper::*;

use async_trait::async_trait;
use std::collections::HashMap;

/// A general trait to query data from the internet.
///
/// This makes requests to a specified API with a specified body and headers, returning the bytes received.
/// It only supports limited HTTP(s) capabilities for now, further capabilities may be added if required.
#[cfg_attr(feature = "rt-multi-thread", async_trait)]
#[cfg_attr(not(feature = "rt-multi-thread"), async_trait(?Send))]
pub trait Requester: Send + Sync {
    /// Errors this requester may produce.
    type Error: std::error::Error;

    /// Make a GET request.
    async fn get(
        &self,
        url: &url::Url,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Make a POST request.
    async fn post(
        &self,
        url: &url::Url,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<Vec<u8>, Self::Error>;
}

/// Build a [`Requester`].
///
/// This allows a provider to specify e.g. custom certificates.
pub trait RequesterBuilder {
    /// The [`Requester`] built by this builder.
    type Requester: Requester;

    /// Add a custom certificate, formatted as a PEM-bundle which should be accepted.
    fn with_pem_bundle(self, bytes: &[u8]) -> Self;

    /// Build the [`Requester`].
    fn build(self) -> Self::Requester;
}
