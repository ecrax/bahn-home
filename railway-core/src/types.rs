use std::borrow::Cow;
use std::collections::HashSet;

use chrono::DateTime;
use chrono::Duration;
use chrono_tz::Tz;
#[cfg(feature = "polylines")]
use geojson::FeatureCollection;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
/// A general location.
pub enum Location {
    /// An address.
    Address {
        /// The address.
        address: String,
        /// The latitude.
        latitude: f32,
        /// The longitude.
        longitude: f32,
    },
    /// Anything else.
    Point {
        /// A not further specified ID.
        id: Option<String>,
        /// Some name.
        name: Option<String>,
        /// Is this a point of interest.
        poi: Option<bool>,
        /// The latitude.
        latitude: f32,
        /// The longitude.
        longitude: f32,
    },
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Location::Address { address: a, .. }, Location::Address { address: b, .. }) => a == b,
            (Location::Point { id: Some(a), .. }, Location::Point { id: Some(b), .. }) => a == b,
            (_, _) => false,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// Any place.
pub enum Place {
    /// A [`Station`].
    Station(Station),
    /// A [`Location`].
    Location(Location),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
/// A station where trains drive from.
pub struct Station {
    /// A provider-specific, unique station id.
    pub id: String,
    /// A human-readable name
    pub name: Option<String>,
    /// Where the station is located.
    pub location: Option<Location>,
    /// The products served on the station.
    pub products: Vec<Product>,
}

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A selection of modes.
pub struct ProductsSelection(HashSet<Mode>);

impl Default for ProductsSelection {
    fn default() -> Self {
        Self::all()
    }
}

impl ProductsSelection {
    pub fn all() -> Self {
        // TODO: Automatically generate from enum?
        Self(HashSet::from([
            Mode::HighSpeedTrain,
            Mode::RegionalTrain,
            Mode::SuburbanTrain,
            Mode::Subway,
            Mode::Tram,
            Mode::Bus,
            Mode::Ferry,
            Mode::Ferry,
            Mode::Cablecar,
            Mode::OnDemand,
            Mode::Unknown,
        ]))
    }

    pub fn contains(&self, mode: &Mode) -> bool {
        self.0.contains(mode)
    }
}

impl From<HashSet<Mode>> for ProductsSelection {
    fn from(modes: HashSet<Mode>) -> Self {
        Self(modes)
    }
}

impl From<ProductsSelection> for HashSet<Mode> {
    fn from(modes: ProductsSelection) -> Self {
        modes.0
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// The product a [`Line`] uses.
pub struct Product {
    /// The mode of transport.
    pub mode: Mode,
    /// A name.
    pub name: Cow<'static, str>,
    /// A shorter name (usually one character).
    pub short: Cow<'static, str>,
}

impl Product {
    pub const fn unknown() -> Self {
        Self {
            mode: Mode::Unknown,
            name: Cow::Borrowed("Unknown"),
            short: Cow::Borrowed("Unknown"),
        }
    }
}

/// Different modes of transport.
///
/// See also <https://gitlab.com/oeffi/public-transport-enabler/-/blob/fffd2c9acb290c583efecfc88b4e5be56e48e6da/src/de/schildbach/pte/dto/Product.java>.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    HighSpeedTrain,
    RegionalTrain,
    SuburbanTrain,
    Subway,
    Tram,
    Bus,
    Ferry,
    Cablecar,
    OnDemand,
    Unknown,
    // TODO: Walking?
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Different kinds of loyalty cards supported.
pub enum LoyaltyCard {
    BahnCard25Class1,
    BahnCard25Class2,
    BahnCard50Class1,
    BahnCard50Class2,
    Vorteilscard,
    HalbtaxaboRailplus,
    Halbtaxabo,
    VoordeelurenaboRailplus,
    Voordeelurenabo,
    SHCard,
    Generalabonnement,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// The age of a person in years.
pub struct Age(pub u64);

impl LoyaltyCard {
    /// See <https://gist.github.com/juliuste/202bb04f450a79f8fa12a2ec3abcd72d>.
    pub fn from_id(value: u8) -> Option<Self> {
        match value {
            1 => Some(LoyaltyCard::BahnCard25Class1),
            2 => Some(LoyaltyCard::BahnCard25Class2),
            3 => Some(LoyaltyCard::BahnCard50Class1),
            4 => Some(LoyaltyCard::BahnCard50Class2),
            9 => Some(LoyaltyCard::Vorteilscard),
            10 => Some(LoyaltyCard::HalbtaxaboRailplus),
            11 => Some(LoyaltyCard::Halbtaxabo),
            12 => Some(LoyaltyCard::VoordeelurenaboRailplus),
            13 => Some(LoyaltyCard::Voordeelurenabo),
            14 => Some(LoyaltyCard::SHCard),
            15 => Some(LoyaltyCard::Generalabonnement),
            _ => None,
        }
    }

    pub fn to_id(self) -> u8 {
        match self {
            LoyaltyCard::BahnCard25Class1 => 1,
            LoyaltyCard::BahnCard25Class2 => 2,
            LoyaltyCard::BahnCard50Class1 => 3,
            LoyaltyCard::BahnCard50Class2 => 4,
            LoyaltyCard::Vorteilscard => 9,
            LoyaltyCard::HalbtaxaboRailplus => 10,
            LoyaltyCard::Halbtaxabo => 11,
            LoyaltyCard::VoordeelurenaboRailplus => 12,
            LoyaltyCard::Voordeelurenabo => 13,
            LoyaltyCard::SHCard => 14,
            LoyaltyCard::Generalabonnement => 15,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// What class to travel with.
pub enum TariffClass {
    /// First class.
    First,
    /// Second class.
    Second,
}

impl Default for TariffClass {
    fn default() -> Self {
        Self::Second
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// How accessible a trip should be.
pub enum Accessibility {
    /// Accessibility is not required.
    r#None,
    /// Partial accessibility is required.
    Partial,
    /// Complete accessibility is required.
    Complete,
}

impl Default for Accessibility {
    fn default() -> Self {
        Self::None
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How full a [`Leg`] is.
pub enum LoadFactor {
    /// Not full.
    LowToMedium,
    /// Full.
    High,
    /// Very full.
    VeryHigh,
    /// Extremely full.
    ExceptionallyHigh,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A line serving a [`Leg`].
pub struct Line {
    /// The line name.
    pub name: Option<String>,
    /// The line number.
    pub fahrt_nr: Option<String>,
    /// The mode of transport.
    pub mode: Mode,
    /// The product of the transport.
    pub product: Product,
    /// The operator of the line.
    pub operator: Option<Operator>,
    /// The nameof the product.
    pub product_name: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// At what frequency the train drives.
pub struct Frequency {
    /// Mimimum duration between the same train.
    #[cfg_attr(feature = "serde", serde(with = "crate::serialize::duration"))]
    pub minimum: Option<Duration>,
    /// Maximum duration between the same train.
    #[cfg_attr(feature = "serde", serde(with = "crate::serialize::duration"))]
    pub maximum: Option<Duration>,
    /// How often this iteration occures.
    pub iterations: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// A leg of a journey.
pub struct Leg {
    /// The origin to enter the train from.
    pub origin: Place,
    /// The destination to exit the train from.
    pub destination: Place,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The real-time departure time.
    pub departure: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The scheduled departure time.
    pub planned_departure: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The real-time arrival time.
    pub arrival: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The scheduled arrival time.
    pub planned_arrival: Option<DateTime<Tz>>,
    /// Whether this leg is reachable from the previous one.
    pub reachable: bool,
    /// A unique ID for the trip.
    pub trip_id: Option<String>,
    /// The line serving this leg.
    pub line: Option<Line>,
    /// The direction of the leg.
    pub direction: Option<String>,
    /// The real-time arrival platform.
    pub arrival_platform: Option<String>,
    /// The scheduled arrival platform.
    pub planned_arrival_platform: Option<String>,
    /// The real-time departure platform.
    pub departure_platform: Option<String>,
    /// The scheduled departure platform.
    pub planned_departure_platform: Option<String>,
    /// The frequency of the leg.
    pub frequency: Option<Frequency>,
    /// Whether this leg was cancelled.
    pub cancelled: bool,
    /// Intermediate locations for the leg.
    pub intermediate_locations: Vec<IntermediateLocation>,
    /// The load of the leg.
    pub load_factor: Option<LoadFactor>,
    /// Remarks on the leg.
    pub remarks: Vec<Remark>,
    /// The polyline of the leg.
    #[cfg(feature = "polylines")]
    pub polyline: Option<FeatureCollection>,
    /// Whether this leg needs to be walked.
    pub walking: bool,
    /// Whether this leg requires transfer.
    pub transfer: bool,
    /// How long this leg is.
    pub distance: Option<u64>,
}

impl Leg {
    // An ID of a leg based on attributes that should not change e.g. when refreshed.
    pub fn id(&self) -> String {
        format!(
            "{};{};{};{};{}",
            self.trip_id.as_ref().map(|s| &s[..]).unwrap_or_default(),
            self.planned_departure
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            self.planned_arrival
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            self.planned_departure_platform
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_default(),
            self.planned_arrival_platform
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_default()
        )
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// Almost always, we have `IntermediateLocation::Stop`. As the documentation for this lint suggests, boxing the larger variant which is almost always used is counterproductive.
#[allow(clippy::large_enum_variant)]
/// An intermediate locaion of a leg.
pub enum IntermediateLocation {
    /// A place where the train stops to let out passengers.
    Stop(Stop),
    /// A named railway track the train is passing over.
    Railway(Place),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// A place where the train stops.
pub struct Stop {
    /// The place of the stop.
    pub place: Place,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The real-time departure time.
    pub departure: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The scheduled departure time.
    pub planned_departure: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The real-time arrival time.
    pub arrival: Option<DateTime<Tz>>,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialize::datetime_with_timezone")
    )]
    /// The scheduled arrival time.
    pub planned_arrival: Option<DateTime<Tz>>,
    /// The real-time arrival platform.
    pub arrival_platform: Option<String>,
    /// The scheduled arrival platform.
    pub planned_arrival_platform: Option<String>,
    /// The real-time departure platform.
    pub departure_platform: Option<String>,
    /// The real-time departure platform.
    pub planned_departure_platform: Option<String>,
    /// Whether this stop is cancelled.
    pub cancelled: bool,
    /// Remarks specific to this stop.
    pub remarks: Vec<Remark>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// The price of the trip.
pub struct Price {
    /// The amount.
    pub amount: f64,
    /// The currency.
    pub currency: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// A single journey from source to destination.
pub struct Journey {
    /// A unique journey id.
    pub id: String,
    /// The legs that make up the journey.
    pub legs: Vec<Leg>,
    /// The price of the journey.
    pub price: Option<Price>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// The operator serving a [`Line`].
pub struct Operator {
    /// An unique id.
    pub id: String,
    /// The name.
    pub name: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// What type a remark has.
pub enum RemarkType {
    /// It is a hint, e.g. information that bikes are not allowed.
    Hint,
    /// It is a status, e.g. that the trip is cancelled.
    Status,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// What to associate a [`Remark`] with.
pub enum RemarkAssociation {
    /// E.g. bikes allowed, disallowed, limited.
    Bike,
    /// E.g. accessible equipment in the train.
    Accessibility,
    /// E.g. one can buy tickets in the train.
    Ticket,
    /// There is a power socket in the train.
    Power,
    /// There is air conditioning on the train.
    AirConditioning,
    /// There is WiFi on the train.
    WiFi,
    /// There is no first class on this train.
    OnlySecondClass,
    /// The remark code specifies an association, but this could not yet be decoded.
    Unknown,
    /// The remark did not specify an association.
    None,
}

impl Default for RemarkAssociation {
    fn default() -> Self {
        Self::Unknown
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A remark on a leg or stopover.
pub struct Remark {
    /// The provider-specific code for it.
    pub code: String,
    /// Text to display.
    pub text: String,
    /// What kind of remark it has.
    pub r#type: RemarkType,
    /// What to associate the remark with.
    pub association: RemarkAssociation,
    /// A short summary of the remark.
    pub summary: Option<String>,
    /// What trip this remark is about.
    pub trip_id: Option<String>,
}
