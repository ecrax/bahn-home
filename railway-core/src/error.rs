use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

/// An error in the API.
#[derive(Debug)]
pub enum Error<R, P> {
    /// Error requesting data using the [`Requester`](crate::Requester).
    Request(R),
    /// Any [`Provider`](crate::Provider)-specific error, e.g. failing to parse the response from the API.
    Provider(P), // TODO: Journey not found, ...
}

impl<R: Display, P: Display> Display for Error<R, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Request(e) => write!(f, "error requesting data: {}", e),
            Self::Provider(e) => write!(f, "provider specific error: {}", e),
        }
    }
}

impl<R: StdError, P: StdError> StdError for Error<R, P> {}
