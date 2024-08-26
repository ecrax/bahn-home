use crate::parse::arrival_or_departure::{HafasArrivalOrDeparture, HafasPlatform};
use crate::parse::common::CommonData;
use crate::parse::stopover::HafasStopover;
use crate::ParseResult;
use crate::Profile;
use crate::{Frequency, Leg};
use chrono::{Duration, NaiveDate};
#[cfg(feature = "polylines")]
use geojson::FeatureCollection;
use rcore::{IntermediateLocation, Stop};
use serde::{Deserialize, Serialize};

#[cfg(feature = "polylines")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyPolyG {
    poly_x_l: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyLoad {
    tcoc_x: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegFreq {
    min_c: Option<u64>,
    max_c: Option<u64>,
    num_c: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJny {
    jid: Option<String>,
    is_rchbl: Option<bool>,
    dir_txt: Option<String>,
    prod_x: Option<usize>,
    stop_l: Option<Vec<HafasStopover>>,
    msg_l: Option<Vec<HafasLegJnyMsg>>,
    #[cfg(feature = "polylines")]
    poly_g: Option<HafasLegJnyPolyG>,
    d_trn_cmp_s_x: Option<HafasLegJnyLoad>,
    freq: Option<HafasLegFreq>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyMsg {
    rem_x: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegArr {
    a_t_z_offset: Option<i32>,
    a_time_s: Option<String>,
    a_time_r: Option<String>,
    a_platf_s: Option<String>,
    a_platf_r: Option<String>,
    a_pltf_s: Option<HafasPlatform>,
    a_pltf_r: Option<HafasPlatform>,
    a_cncl: Option<bool>,
    loc_x: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegDep {
    d_t_z_offset: Option<i32>,
    d_time_s: Option<String>,
    d_time_r: Option<String>,
    d_platf_s: Option<String>,
    d_platf_r: Option<String>,
    d_pltf_s: Option<HafasPlatform>,
    d_pltf_r: Option<HafasPlatform>,
    d_cncl: Option<bool>,
    loc_x: usize,
}

/// See <https://gist.github.com/derhuerst/77481f7a6115054f398799006c7062ef#file-rest-1-23-xsd-L1693>
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum HafasLegType {
    #[serde(rename = "JNY")]
    Journey,
    #[serde(rename = "WALK")]
    Walk,
    #[serde(rename = "TRSF")]
    Transfer,
    // TODO: Bike, KISS (Car), Park, Taxi
    #[serde(rename = "DEVI")]
    Deviation,
    #[serde(rename = "CHKI")]
    CheckIn,
    #[serde(rename = "TETA")]
    TeleTaxi,
}

#[derive(Debug, Deserialize)]
pub struct HafasLegGis {
    dist: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct HafasLeg {
    dep: HafasLegDep,
    arr: HafasLegArr,
    jny: Option<HafasLegJny>,
    gis: Option<HafasLegGis>,
    r#type: HafasLegType,
    hide: Option<bool>,
}

pub(crate) fn default_parse_leg<P: Profile + ?Sized>(
    profile: &P,
    data: HafasLeg,
    common: &CommonData,
    date: &NaiveDate,
) -> ParseResult<Option<Leg>> {
    let HafasLeg {
        mut dep,
        mut arr,
        jny,
        gis,
        r#type,
        hide,
    } = data;

    if let Some(true) = hide {
        return Ok(None);
    }

    let origin = common
        .places
        .get(dep.loc_x)
        .cloned()
        .ok_or_else(|| format!("Invalid place index: {}", arr.loc_x))?
        .ok_or_else(|| format!("Parse error place index: {}", arr.loc_x))?;
    let destination = common
        .places
        .get(arr.loc_x)
        .cloned()
        .ok_or_else(|| format!("Invalid place index: {}", arr.loc_x))?
        .ok_or_else(|| format!("Parse error place index: {}", arr.loc_x))?;

    if r#type == HafasLegType::Walk && dep.d_t_z_offset != arr.a_t_z_offset {
        if dep.d_t_z_offset == Some(0) {
            dep.d_t_z_offset = arr.a_t_z_offset;
        }
        if arr.a_t_z_offset == Some(0) {
            arr.a_t_z_offset = dep.d_t_z_offset;
        }
    }

    let dep = profile.parse_arrival_or_departure(
        HafasArrivalOrDeparture {
            t_z_offset: dep.d_t_z_offset,
            time_s: dep.d_time_s,
            time_r: dep.d_time_r,
            platf_s: dep.d_platf_s,
            platf_r: dep.d_platf_r,
            pltf_s: dep.d_pltf_s,
            pltf_r: dep.d_pltf_r,
            cncl: dep.d_cncl,
        },
        date,
    )?;
    let arr = profile.parse_arrival_or_departure(
        HafasArrivalOrDeparture {
            t_z_offset: arr.a_t_z_offset,
            time_s: arr.a_time_s,
            time_r: arr.a_time_r,
            platf_s: arr.a_platf_s,
            platf_r: arr.a_platf_r,
            pltf_s: arr.a_pltf_s,
            pltf_r: arr.a_pltf_r,
            cncl: arr.a_cncl,
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

    let mut line = None;
    let mut reachable = None;
    let mut trip_id = None;
    let mut direction = None;
    let mut stopovers: Option<Vec<Stop>> = None;
    let mut load_factor = None;
    let mut remarks = None;
    #[cfg(feature = "polylines")]
    let mut polyline = None;
    let mut walking = None;
    let mut transfer = None;
    let mut distance = None;
    let mut frequency = None;

    match r#type {
        HafasLegType::Journey | HafasLegType::TeleTaxi => {
            let HafasLegJny {
                prod_x,
                is_rchbl,
                jid,
                dir_txt,
                stop_l,
                msg_l,
                #[cfg(feature = "polylines")]
                poly_g,
                d_trn_cmp_s_x,
                freq,
            } = jny.ok_or("Missing jny field")?;
            line = prod_x
                .map(|x| -> ParseResult<_> {
                    Ok(common
                        .lines
                        .get(x)
                        .cloned()
                        .ok_or_else(|| format!("Invalid line index: {}", x))?
                        .ok_or_else(|| format!("Parse error line index: {}", x))?)
                })
                .transpose()?;
            reachable = is_rchbl;
            trip_id = jid;
            direction = dir_txt;
            stopovers = stop_l
                .map(|x| {
                    x.into_iter()
                        .map(|x| profile.parse_stopover(x, common, date))
                        .collect::<ParseResult<_>>()
                })
                .transpose()?;
            remarks = msg_l
                .map(|x| {
                    x.into_iter()
                        .filter_map(|x| x.rem_x)
                        .filter_map(|x| {
                            common
                                .remarks
                                .get(x)
                                .cloned()
                                .ok_or_else(|| format!("Invalid remark index: {}", x).into())
                                .transpose()
                        })
                        .collect::<ParseResult<_>>()
                })
                .transpose()?;
            frequency = freq.map(|freq| Frequency {
                minimum: freq.min_c.map(|i| Duration::minutes(i as i64)),
                maximum: freq.max_c.map(|i| Duration::minutes(i as i64)),
                iterations: freq.num_c,
            });

            #[cfg(feature = "polylines")]
            {
                polyline = poly_g
                    .map(|poly_g| -> ParseResult<_> {
                        let mut features = vec![];
                        for x in poly_g.poly_x_l {
                            let mut polyline = common
                                .polylines
                                .get(x)
                                .ok_or_else(|| format!("Invalid polyline index: {}", x))?
                                .clone();
                            features.append(&mut polyline);
                        }
                        Ok(FeatureCollection {
                            features,
                            bbox: None,
                            foreign_members: None,
                        })
                    })
                    .transpose()?;
            }
            load_factor = d_trn_cmp_s_x
                .map(|x: HafasLegJnyLoad| -> ParseResult<_> {
                    let mut entries = vec![];
                    for i in x.tcoc_x {
                        entries.push(
                            common
                                .load_factors
                                .get(i)
                                .ok_or_else(|| format!("Invalid load factor index: {}", i))?
                                .clone(),
                        );
                    }
                    Ok(entries
                        .into_iter()
                        .find(|x| x.class == common.tariff_class)
                        .map(|x| x.load))
                })
                .transpose()?
                .and_then(|x| x);
        }
        HafasLegType::Walk => {
            walking = Some(true);
            distance = gis.and_then(|x| x.dist);
        }
        HafasLegType::Transfer | HafasLegType::Deviation => {
            transfer = Some(true);
        }
        HafasLegType::CheckIn => {}
    }

    Ok(Some(Leg {
        origin,
        destination,
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
        frequency,
        cancelled: cancelled.unwrap_or_default(),
        line,
        reachable: reachable.unwrap_or(true),
        trip_id,
        direction,
        intermediate_locations: stopovers
            .into_iter()
            .flat_map(|s| s.into_iter())
            .map(IntermediateLocation::Stop)
            .collect(),
        load_factor,
        remarks: remarks.unwrap_or_default(),
        #[cfg(feature = "polylines")]
        polyline,
        walking: walking.unwrap_or_default(),
        transfer: transfer.unwrap_or_default(),
        distance,
    }))
}
