mod journeys;
mod locations;
mod refresh_journey;

pub use journeys::*;
pub use locations::*;
pub use refresh_journey::*;

use crate::{Journey, Place, Requester};
use async_trait::async_trait;

/// The core type definition specifying what a provider needs to do.
///
/// To implement the required methods, a provider usually queries an external but public API.
#[cfg_attr(feature = "rt-multi-thread", async_trait)]
#[cfg_attr(not(feature = "rt-multi-thread"), async_trait(?Send))]
pub trait Provider<R: Requester> {
    type Error: std::error::Error;

    /// Query a list of journeys.
    ///
    /// Be careful about timezones!
    /// The types given to you are annotated with an arbitrary timezone, your public API may only understand one specific timezone though.
    /// Ensure you correctly convert the given timezone to a timezone you require.
    /// For returning a date and time, you may choose an arbitrary timezone.
    async fn journeys(
        &self,
        from: Place,
        to: Place,
        opts: JourneysOptions,
    ) -> Result<JourneysResponse, crate::Error<R::Error, Self::Error>>;

    /// Autocomplete a location.
    ///
    /// This takes a query string and should return a list of locations which match the given string.
    async fn locations(
        &self,
        opts: LocationsOptions,
    ) -> Result<LocationsResponse, crate::Error<R::Error, Self::Error>>;

    /// Refresh a journey.
    ///
    /// This takes a previously queried journey and refreshes real-time data.
    /// A naive implementation may call [`Provider::journeys`] again and return the matching journey, this is a valid strategy if there is no API for refreshing a journey.
    async fn refresh_journey(
        &self,
        journey: &Journey,
        opts: RefreshJourneyOptions,
    ) -> Result<RefreshJourneyResponse, crate::Error<R::Error, Self::Error>>;
}
