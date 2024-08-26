use crate::parse::common::HafasCommon;
use crate::parse::journey::HafasJourney;
use crate::ParseResult;
use crate::Profile;
use crate::TariffClass;
use rcore::JourneysResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasJourneysResponse {
    out_ctx_scr_b: Option<String>,
    out_ctx_scr_f: Option<String>,
    out_con_l: Vec<HafasJourney>,
    common: HafasCommon,
}

pub(crate) fn default_parse_journeys_response<P: Profile + ?Sized>(
    profile: &P,
    data: HafasJourneysResponse,
    tariff_class: TariffClass,
) -> ParseResult<JourneysResponse> {
    let HafasJourneysResponse {
        out_ctx_scr_b,
        out_ctx_scr_f,
        out_con_l,
        common,
    } = data;
    let common_data = profile.parse_common(common, tariff_class)?;

    Ok(JourneysResponse {
        earlier_ref: out_ctx_scr_b,
        later_ref: out_ctx_scr_f,
        journeys: out_con_l
            .into_iter()
            .map(|x| profile.parse_journey(x, &common_data))
            .collect::<ParseResult<_>>()?,
    })
}
