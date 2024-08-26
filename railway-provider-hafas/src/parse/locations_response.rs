use crate::parse::location::HafasPlace;
use crate::ParseResult;
use crate::Profile;
use rcore::LocationsResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasLocationsResponse {
    r#match: HafasLocationsResponseMatch,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLocationsResponseMatch {
    loc_l: Vec<HafasPlace>,
}

pub(crate) fn default_parse_locations_response<P: Profile + ?Sized>(
    profile: &P,
    data: HafasLocationsResponse,
) -> ParseResult<LocationsResponse> {
    Ok(data
        .r#match
        .loc_l
        .into_iter()
        .filter_map(|p| profile.parse_place(p).ok())
        .collect::<Vec<_>>())
}
