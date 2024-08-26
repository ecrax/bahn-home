use crate::parse::arrival_or_departure::{HafasArrivalOrDeparture, HafasPlatform};
use crate::parse::common::CommonData;
use crate::ParseResult;
use crate::Profile;
use crate::Stop;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasStopover {
    loc_x: usize,
    a_t_z_offset: Option<i32>,
    a_time_s: Option<String>,
    a_time_r: Option<String>,
    a_platf_s: Option<String>,
    a_platf_r: Option<String>,
    a_pltf_s: Option<HafasPlatform>,
    a_pltf_r: Option<HafasPlatform>,
    a_cncl: Option<bool>,
    d_t_z_offset: Option<i32>,
    d_time_s: Option<String>,
    d_time_r: Option<String>,
    d_platf_s: Option<String>,
    d_platf_r: Option<String>,
    d_pltf_s: Option<HafasPlatform>,
    d_pltf_r: Option<HafasPlatform>,
    d_cncl: Option<bool>,
    msg_l: Option<Vec<HafasStopoverMsg>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasStopoverMsg {
    rem_x: usize,
}

pub(crate) fn default_parse_stopover<P: Profile + ?Sized>(
    profile: &P,
    data: HafasStopover,
    common: &CommonData,
    date: &NaiveDate,
) -> ParseResult<Stop> {
    let HafasStopover {
        loc_x,
        a_t_z_offset,
        a_time_s,
        a_time_r,
        a_platf_s,
        a_platf_r,
        a_pltf_s,
        a_pltf_r,
        a_cncl,
        d_t_z_offset,
        d_time_s,
        d_time_r,
        d_platf_s,
        d_platf_r,
        d_pltf_s,
        d_pltf_r,
        d_cncl,
        msg_l,
    } = data;
    let stop = common
        .places
        .get(loc_x)
        .and_then(|x| x.clone())
        .ok_or_else(|| format!("Invalid place index {}", loc_x))?;
    let dep = profile.parse_arrival_or_departure(
        HafasArrivalOrDeparture {
            t_z_offset: d_t_z_offset,
            time_s: d_time_s,
            time_r: d_time_r,
            platf_s: d_platf_s,
            platf_r: d_platf_r,
            pltf_s: d_pltf_s,
            pltf_r: d_pltf_r,
            cncl: d_cncl,
        },
        date,
    )?;
    let arr = profile.parse_arrival_or_departure(
        HafasArrivalOrDeparture {
            t_z_offset: a_t_z_offset,
            time_s: a_time_s,
            time_r: a_time_r,
            platf_s: a_platf_s,
            platf_r: a_platf_r,
            pltf_s: a_pltf_s,
            pltf_r: a_pltf_r,
            cncl: a_cncl,
        },
        date,
    )?;

    let mut cancelled = None;
    if let Some(true) = dep.cancelled {
        cancelled = Some(true);
    }
    if let Some(true) = arr.cancelled {
        cancelled = Some(true);
    }

    let remarks = msg_l
        .map(|x| {
            x.into_iter()
                .filter_map(|x| {
                    common
                        .remarks
                        .get(x.rem_x)
                        .cloned()
                        .ok_or_else(|| format!("Invalid remark index: {}", x.rem_x).into())
                        .transpose()
                })
                .collect::<ParseResult<_>>()
        })
        .transpose()?;

    Ok(Stop {
        place: stop,
        departure: dep.time.map(|t| t.with_timezone(&profile.timezone())),
        planned_departure: dep
            .planned_time
            .map(|t| t.with_timezone(&profile.timezone())),
        arrival: arr.time.map(|t| t.with_timezone(&profile.timezone())),
        planned_arrival: arr
            .planned_time
            .map(|t| t.with_timezone(&profile.timezone())),
        arrival_platform: arr.platform,
        planned_arrival_platform: arr.planned_platform,
        departure_platform: dep.platform,
        planned_departure_platform: dep.planned_platform,
        cancelled: cancelled.unwrap_or_default(),
        remarks: remarks.unwrap_or_default(),
    })
}
