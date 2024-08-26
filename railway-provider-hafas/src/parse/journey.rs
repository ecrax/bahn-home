use crate::parse::common::CommonData;
use crate::parse::leg::HafasLeg;
use crate::Journey;
use crate::ParseResult;
use crate::Price;
use crate::Profile;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasJourneyFarePrice {
    amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct HafasJourneyFare {
    price: Option<HafasJourneyFarePrice>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasJourneyFareSet {
    #[serde(default)]
    fare_l: Vec<HafasJourneyFare>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasJourneyTrfRes {
    #[serde(default)]
    fare_set_l: Vec<HafasJourneyFareSet>,
}

#[derive(Debug, Deserialize)]
pub struct HafasJourneyRecon {
    ctx: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasJourney {
    date: String,
    ctx_recon: Option<String>,
    recon: Option<HafasJourneyRecon>,
    sec_l: Vec<HafasLeg>,
    trf_res: Option<HafasJourneyTrfRes>,
}

pub(crate) fn default_parse_journey<P: Profile + ?Sized>(
    profile: &P,
    data: HafasJourney,
    common: &CommonData,
) -> ParseResult<Journey> {
    let HafasJourney {
        date,
        recon,
        ctx_recon,
        sec_l,
        trf_res,
    } = data;

    let date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;

    let lowest_price = trf_res
        .map(|x| x.fare_set_l)
        .unwrap_or_default()
        .into_iter()
        .flat_map(|x| x.fare_l)
        .filter_map(|x| x.price)
        .map(|x| x.amount)
        .filter(|x| *x > 0)
        .min()
        .map(|x| Price {
            currency: profile.price_currency().to_string(),
            amount: x as f64 / 100.0,
        });

    let legs: Vec<_> = sec_l
        .into_iter()
        .filter_map(|x| profile.parse_leg(x, common, &date).transpose())
        .filter(|l| {
            !l.as_ref()
                .is_ok_and(|l| l.walking && l.planned_departure == l.planned_arrival)
        }) // Filter out 0-minute walks; those must have been introduced mid-2024 to Hafas, but we don't know why.
        .collect::<ParseResult<_>>()?;

    Ok(Journey {
        id: recon
            .and_then(|x| x.ctx)
            .or(ctx_recon)
            .unwrap_or_else(|| legs.iter().map(|l| l.id() + "|").collect()),
        legs,
        price: lowest_price,
    })
}
