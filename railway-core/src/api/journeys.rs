use chrono::{DateTime, Duration};
use chrono_tz::Tz;

use crate::{Accessibility, Age, Journey, LoyaltyCard, Place, ProductsSelection, TariffClass};

#[derive(Debug, Clone)]
/// The options for [`Provider::journeys`](crate::Provider::journeys)
///
/// A provider can also ignore some of the options if this is not supported by the API.
pub struct JourneysOptions {
    /// Intermediate place to route with.
    pub via: Vec<Place>,
    /// Specify to route earlier than, from [`JourneysResponse::earlier_ref`].
    pub earlier_than: Option<String>,
    /// Specify to route earlierthan , from [`JourneysResponse::later_ref`].
    pub later_than: Option<String>,
    /// How many results to include.
    pub results: u64,
    /// Whether to include stopovers.
    pub stopovers: bool,
    /// Whether to include polylines.
    #[cfg(feature = "polylines")]
    pub polylines: bool,
    /// Route in a bike-friendly way.
    pub bike_friendly: bool,
    /// Include tickets.
    pub tickets: bool,
    /// Allow to walk to the initial station.
    pub start_with_walking: bool,
    /// How accessible the journey must be.
    pub accessibility: Accessibility,
    /// How often it is allowed to transfer.
    pub transfers: TransferOptions,
    /// How long must the transfers be.
    pub transfer_time: Duration,
    /// When should the journey arrive, or earlier.
    pub arrival: Option<DateTime<Tz>>,
    /// When should the journey depart, or later.
    pub departure: Option<DateTime<Tz>>,
    /// What products to route with.
    pub products: ProductsSelection,
    /// What class to use.
    pub tariff_class: TariffClass,
    /// What language to query with.
    pub language: Option<String>,
    /// What loyalty cards does the passenger have.
    pub loyalty_card: Option<LoyaltyCard>,
    /// What age does the passenger have.
    pub passenger_age: Option<Age>,
}

/// How often is a journey allowed to transfer.
#[derive(Debug, Clone, Default)]
pub enum TransferOptions {
    /// Allow unlimited transfers.
    #[default]
    Unlimited,
    /// Allow transfers only to a limited number.
    Limited(u64),
}

impl Default for JourneysOptions {
    fn default() -> Self {
        Self {
            via: Default::default(),
            earlier_than: Default::default(),
            later_than: Default::default(),
            results: 5,
            stopovers: Default::default(),
            #[cfg(feature = "polylines")]
            polylines: Default::default(),
            bike_friendly: Default::default(),
            tickets: true,
            start_with_walking: true,
            accessibility: Default::default(),
            transfers: TransferOptions::default(),
            transfer_time: Duration::zero(),
            arrival: Default::default(),
            departure: Default::default(),
            products: Default::default(),
            tariff_class: TariffClass::Second,
            language: Default::default(),
            loyalty_card: Default::default(),
            passenger_age: Default::default(),
        }
    }
}

/// The response for [`Provider::journeys`](crate::Provider::journeys)
#[derive(Debug, Clone)]
pub struct JourneysResponse {
    /// Reference to query earlier, used in [`JourneysOptions::earlier_than`].
    pub earlier_ref: Option<String>,
    /// Reference to query later, used in [`JourneysOptions::later_than`].
    pub later_ref: Option<String>,
    /// The list of journeys which is the result of the request.
    pub journeys: Vec<Journey>,
}
