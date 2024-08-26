#[cfg(feature = "avv-profile")]
pub mod avv;
#[cfg(feature = "bart-profile")]
pub mod bart;
#[cfg(feature = "bls-profile")]
pub mod bls;
#[cfg(feature = "cfl-profile")]
pub mod cfl;
#[cfg(feature = "cmta-profile")]
pub mod cmta;
#[cfg(feature = "dart-profile")]
pub mod dart;
#[cfg(feature = "db-profile")]
pub mod db;
#[cfg(feature = "insa-profile")]
pub mod insa;
#[cfg(feature = "irish-rail-profile")]
pub mod irish_rail;
#[cfg(feature = "ivb-profile")]
pub mod ivb;
#[cfg(feature = "kvb-profile")]
pub mod kvb;
#[cfg(feature = "mobil-nrw-profile")]
pub mod mobil_nrw;
#[cfg(feature = "mobiliteit-lu-profile")]
pub mod mobiliteit_lu;
#[cfg(feature = "nahsh-profile")]
pub mod nahsh;
#[cfg(feature = "nvv-profile")]
pub mod nvv;
#[cfg(feature = "oebb-profile")]
pub mod oebb;
#[cfg(feature = "ooevv-profile")]
pub mod ooevv;
#[cfg(feature = "pkp-profile")]
pub mod pkp;
#[cfg(feature = "rejseplanen-profile")]
pub mod rejseplanen;
#[cfg(feature = "rmv-profile")]
pub mod rmv;
#[cfg(feature = "rsag-profile")]
pub mod rsag;
#[cfg(feature = "saarvv-profile")]
pub mod saarvv;
#[cfg(feature = "salzburg-profile")]
pub mod salzburg;
#[cfg(feature = "sbahn-muenchen-profile")]
pub mod sbahn_muenchen;
#[cfg(feature = "vgi-profile")]
pub mod vgi;
// Currently broken due to: <https://github.com/public-transport/hafas-client/issues/284>
// #[cfg(feature = "sncf-profile")]
// pub mod sncf;
// Currently broken due to: HAFAS Kernel: Date outside of the timetable period.
// #[cfg(feature = "tpg-profile")]
// pub mod tpg;
#[cfg(feature = "resrobot-profile")]
pub mod resrobot;
#[cfg(feature = "svv-profile")]
pub mod svv;
#[cfg(feature = "vbb-profile")]
pub mod vbb;
#[cfg(feature = "vbn-profile")]
pub mod vbn;
#[cfg(feature = "verbundlinie-profile")]
pub mod verbundlinie;
#[cfg(feature = "vkg-profile")]
pub mod vkg;
#[cfg(feature = "vmt-profile")]
pub mod vmt;
#[cfg(feature = "vor-profile")]
pub mod vor;
#[cfg(feature = "vos-profile")]
pub mod vos;
#[cfg(feature = "vrn-profile")]
pub mod vrn;
#[cfg(feature = "vsn-profile")]
pub mod vsn;
#[cfg(feature = "vvt-profile")]
pub mod vvt;
#[cfg(feature = "vvv-profile")]
pub mod vvv;
// ADD PROFILE HERE

// TODO:
// BVG: Too many special things for now

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
#[cfg(feature = "polylines")]
use geojson::Feature;
use rcore::Age;
use rcore::ProductsSelection;
use rcore::RemarkAssociation;
use serde_json::Value;
use std::collections::HashMap;

use crate::Journey;
use crate::Leg;
use crate::Line;
use crate::LoadFactor;
use crate::Operator;
use crate::ParseResult;
use crate::Place;
use crate::Product;
use crate::Remark;
use crate::Stop;
use crate::TariffClass;

use rcore::JourneysResponse;
use rcore::LocationsResponse;

use crate::parse::arrival_or_departure::*;
use crate::parse::common::*;
use crate::parse::date::*;
use crate::parse::journey::*;
use crate::parse::journeys_response::*;
use crate::parse::leg::*;
use crate::parse::line::*;
use crate::parse::load_factor::*;
use crate::parse::location::*;
use crate::parse::locations_response::*;
use crate::parse::operator::*;
#[cfg(feature = "polylines")]
use crate::parse::polyline::*;
use crate::parse::products::*;
use crate::parse::remark::*;
use crate::parse::stopover::*;

pub trait Profile: Send + Sync {
    fn url(&self) -> &'static str;
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }
    fn prepare_body(&self, req_json: &mut Value);
    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>);
    fn price_currency(&self) -> &'static str;
    fn timezone(&self) -> chrono_tz::Tz;
    fn language(&self) -> &'static str {
        "en"
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        false
    }
    fn mic_mac(&self) -> bool {
        false
    }
    fn salt(&self) -> bool {
        false
    }

    fn products(&self) -> &'static [&'static Product];

    fn custom_pem_bundle(&self) -> Option<&'static [u8]> {
        None
    }

    fn products_to_hafas(&self, selection: &ProductsSelection) -> u16 {
        let products = self.products();

        let mut result = 0;
        for (i, product) in products.iter().enumerate() {
            if selection.contains(&product.mode) {
                result |= 1 << i;
            }
        }

        result
    }

    fn age_to_hafas(&self, _age: Age) -> &'static str {
        "E"
    }

    fn parse_common(
        &self,
        data: HafasCommon,
        tariff_class: TariffClass,
    ) -> ParseResult<CommonData> {
        default_parse_common(self, data, tariff_class)
    }
    fn parse_arrival_or_departure(
        &self,
        data: HafasArrivalOrDeparture,
        date: &NaiveDate,
    ) -> ParseResult<ArrivalOrDeparture> {
        default_parse_arrival_or_departure(self, data, date)
    }
    fn parse_stopover(
        &self,
        data: HafasStopover,
        common: &CommonData,
        date: &NaiveDate,
    ) -> ParseResult<Stop> {
        default_parse_stopover(self, data, common, date)
    }
    fn parse_remark(&self, data: HafasRemark) -> ParseResult<Remark> {
        default_parse_remark(data, |c| self.remark_association(c))
    }
    fn remark_association(&self, code: &str) -> RemarkAssociation {
        if code.is_empty() {
            RemarkAssociation::None
        } else {
            RemarkAssociation::Unknown
        }
    }
    fn parse_products(&self, p_cls: u16) -> Vec<&'static Product> {
        default_parse_products(p_cls, self.products())
    }
    fn parse_product(&self, p_cls: u16) -> ParseResult<&'static Product> {
        default_parse_product(p_cls, self.products())
    }
    #[cfg(feature = "polylines")]
    fn parse_polyline(&self, data: HafasPolyline) -> ParseResult<Vec<Feature>> {
        default_parse_polyline(data)
    }
    fn parse_operator(&self, data: HafasOperator) -> ParseResult<Operator> {
        default_parse_operator(data)
    }
    fn parse_locations_response(
        &self,
        data: HafasLocationsResponse,
    ) -> ParseResult<LocationsResponse> {
        default_parse_locations_response(self, data)
    }
    fn parse_coords(&self, data: HafasCoords) -> (f32, f32) {
        default_parse_coords(data)
    }
    fn parse_place(&self, data: HafasPlace) -> ParseResult<Place> {
        default_parse_place(self, data)
    }
    fn parse_line(&self, data: HafasLine, operators: &[Operator]) -> ParseResult<Line> {
        default_parse_line(self, data, operators)
    }
    fn parse_leg(
        &self,
        data: HafasLeg,
        common: &CommonData,
        date: &NaiveDate,
    ) -> ParseResult<Option<Leg>> {
        default_parse_leg(self, data, common, date)
    }
    fn parse_journeys_response(
        &self,
        data: HafasJourneysResponse,
        tariff_class: TariffClass,
    ) -> ParseResult<JourneysResponse> {
        default_parse_journeys_response(self, data, tariff_class)
    }
    fn parse_date(
        &self,
        time: Option<String>,
        tz_offset: Option<i32>,
        date: &NaiveDate,
    ) -> ParseResult<Option<DateTime<FixedOffset>>> {
        default_parse_date(self, time, tz_offset, date)
    }
    fn parse_load_factor_entry(&self, h: HafasLoadFactorEntry) -> ParseResult<LoadFactorEntry> {
        default_parse_load_factor_entry(self, h)
    }
    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        default_parse_load_factor(h)
    }
    fn parse_journey(&self, data: HafasJourney, common: &CommonData) -> ParseResult<Journey> {
        default_parse_journey(self, data, common)
    }
}

impl<T: Profile + ?Sized> Profile for Box<T> {
    fn url(&self) -> &'static str {
        (**self).url()
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        (**self).checksum_salt()
    }
    fn prepare_body(&self, req_json: &mut Value) {
        (**self).prepare_body(req_json)
    }
    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        (**self).prepare_headers(headers)
    }
    fn price_currency(&self) -> &'static str {
        (**self).price_currency()
    }
    fn timezone(&self) -> chrono_tz::Tz {
        (**self).timezone()
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        (**self).refresh_journey_use_out_recon_l()
    }
    fn mic_mac(&self) -> bool {
        (**self).mic_mac()
    }
    fn salt(&self) -> bool {
        (**self).salt()
    }

    fn products(&self) -> &'static [&'static Product] {
        (**self).products()
    }

    fn parse_common(
        &self,
        data: HafasCommon,
        tariff_class: TariffClass,
    ) -> ParseResult<CommonData> {
        (**self).parse_common(data, tariff_class)
    }
    fn parse_arrival_or_departure(
        &self,
        data: HafasArrivalOrDeparture,
        date: &NaiveDate,
    ) -> ParseResult<ArrivalOrDeparture> {
        (**self).parse_arrival_or_departure(data, date)
    }
    fn parse_stopover(
        &self,
        data: HafasStopover,
        common: &CommonData,
        date: &NaiveDate,
    ) -> ParseResult<Stop> {
        (**self).parse_stopover(data, common, date)
    }
    fn parse_remark(&self, data: HafasRemark) -> ParseResult<Remark> {
        (**self).parse_remark(data)
    }
    fn parse_products(&self, p_cls: u16) -> Vec<&'static Product> {
        (**self).parse_products(p_cls)
    }
    fn parse_product(&self, p_cls: u16) -> ParseResult<&'static Product> {
        (**self).parse_product(p_cls)
    }
    #[cfg(feature = "polylines")]
    fn parse_polyline(&self, data: HafasPolyline) -> ParseResult<Vec<Feature>> {
        (**self).parse_polyline(data)
    }
    fn parse_operator(&self, data: HafasOperator) -> ParseResult<Operator> {
        (**self).parse_operator(data)
    }
    fn parse_locations_response(
        &self,
        data: HafasLocationsResponse,
    ) -> ParseResult<LocationsResponse> {
        (**self).parse_locations_response(data)
    }
    fn parse_coords(&self, data: HafasCoords) -> (f32, f32) {
        (**self).parse_coords(data)
    }
    fn parse_place(&self, data: HafasPlace) -> ParseResult<Place> {
        (**self).parse_place(data)
    }
    fn parse_line(&self, data: HafasLine, operators: &[Operator]) -> ParseResult<Line> {
        (**self).parse_line(data, operators)
    }
    fn parse_leg(
        &self,
        data: HafasLeg,
        common: &CommonData,
        date: &NaiveDate,
    ) -> ParseResult<Option<Leg>> {
        (**self).parse_leg(data, common, date)
    }
    fn parse_journeys_response(
        &self,
        data: HafasJourneysResponse,
        tariff_class: TariffClass,
    ) -> ParseResult<JourneysResponse> {
        (**self).parse_journeys_response(data, tariff_class)
    }
    fn parse_date(
        &self,
        time: Option<String>,
        tz_offset: Option<i32>,
        date: &NaiveDate,
    ) -> ParseResult<Option<DateTime<FixedOffset>>> {
        (**self).parse_date(time, tz_offset, date)
    }
    fn parse_load_factor_entry(&self, h: HafasLoadFactorEntry) -> ParseResult<LoadFactorEntry> {
        (**self).parse_load_factor_entry(h)
    }
    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        (**self).parse_load_factor(h)
    }
    fn parse_journey(&self, data: HafasJourney, common: &CommonData) -> ParseResult<Journey> {
        (**self).parse_journey(data, common)
    }
}

#[cfg(test)]
pub mod test {
    use crate::{client::HafasClient, Location, Place, Profile, Station};
    use rcore::{HyperRustlsRequesterBuilder, Provider};
    use rcore::{JourneysOptions, LocationsOptions};

    pub async fn check_search<S: AsRef<str>, P: Profile + 'static>(
        profile: P,
        search: S,
        expected: S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = HafasClient::new(profile, HyperRustlsRequesterBuilder::default());
        let locations = client
            .locations(LocationsOptions {
                query: search.as_ref().to_string(),
                ..Default::default()
            })
            .await?;
        let results = locations
            .into_iter()
            .flat_map(|p| match p {
                Place::Station(s) => s.name,
                Place::Location(Location::Address { address, .. }) => Some(address),
                Place::Location(Location::Point { name, .. }) => name,
            })
            .collect::<Vec<_>>();
        assert!(
            results.iter().find(|s| s == &expected.as_ref()).is_some(),
            "expected {} to be contained in {:#?}",
            expected.as_ref(),
            results
        );
        Ok(())
    }

    pub async fn check_journey<S: AsRef<str>, P: Profile + 'static>(
        profile: P,
        from: S,
        to: S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = HafasClient::new(profile, HyperRustlsRequesterBuilder::default());
        let journeys = client
            .journeys(
                Place::Station(Station {
                    id: from.as_ref().to_string(),
                    ..Default::default()
                }),
                Place::Station(Station {
                    id: to.as_ref().to_string(),
                    ..Default::default()
                }),
                JourneysOptions::default(),
            )
            .await?;
        assert!(
            !journeys.journeys.is_empty(),
            "expected journey from {} to {} to exist",
            from.as_ref(),
            to.as_ref()
        );
        Ok(())
    }
}
