use crate::{Journey, TariffClass};

#[derive(Debug, Default)]
/// The options for [`Provider::refresh_journey`](crate::Provider::refresh_journey)
///
/// A provider can also ignore some of the options if this is not supported by the API.
pub struct RefreshJourneyOptions {
    /// Whether to include stopovers.
    pub stopovers: bool,
    #[cfg(feature = "polylines")]
    /// Whether to include polylines.
    pub polylines: bool,
    /// Include tickets.
    pub tickets: bool,
    /// What class to use.
    pub tariff_class: TariffClass,
    /// What language to query with.
    pub language: Option<String>,
}

/// The result for [`Provider::refresh_journey`](crate::Provider::refresh_journey)
pub type RefreshJourneyResponse = Journey;
