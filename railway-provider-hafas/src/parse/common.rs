use crate::parse::line::HafasLine;
use crate::parse::load_factor::HafasLoadFactorEntry;
use crate::parse::load_factor::LoadFactorEntry;
use crate::parse::location::HafasPlace;
use crate::parse::operator::HafasOperator;
#[cfg(feature = "polylines")]
use crate::parse::polyline::HafasPolyline;
use crate::parse::remark::HafasRemark;
use crate::Line;
use crate::ParseResult;
use crate::Place;
use crate::Profile;
use crate::Remark;
use crate::TariffClass;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasCommon {
    loc_l: Vec<HafasPlace>,
    prod_l: Vec<HafasLine>,
    op_l: Option<Vec<HafasOperator>>,
    tcoc_l: Option<Vec<HafasLoadFactorEntry>>,
    rem_l: Option<Vec<HafasRemark>>,
    #[cfg(feature = "polylines")]
    poly_l: Option<Vec<HafasPolyline>>,
}

#[derive(Debug)]
pub struct CommonData {
    pub tariff_class: TariffClass,
    pub places: Vec<Option<Place>>,
    pub lines: Vec<Option<Line>>,
    pub load_factors: Vec<LoadFactorEntry>,
    pub remarks: Vec<Option<Remark>>,
    #[cfg(feature = "polylines")]
    pub polylines: Vec<Vec<geojson::Feature>>,
}

pub(crate) fn default_parse_common<P: Profile + ?Sized>(
    profile: &P,
    data: HafasCommon,
    tariff_class: TariffClass,
) -> ParseResult<CommonData> {
    let HafasCommon {
        loc_l,
        prod_l,
        op_l,
        tcoc_l,
        rem_l,
        #[cfg(feature = "polylines")]
        poly_l,
    } = data;
    let operators: Vec<_> = op_l
        .map(|x| {
            x.into_iter()
                .map(|x| profile.parse_operator(x))
                .collect::<ParseResult<_>>()
        })
        .transpose()?
        .unwrap_or_default();
    Ok(CommonData {
        tariff_class,
        places: loc_l
            .into_iter()
            .map(|x| profile.parse_place(x).ok())
            .collect(),
        lines: prod_l
            .into_iter()
            .map(|x| profile.parse_line(x, &operators).ok())
            .collect(),
        load_factors: tcoc_l
            .unwrap_or_default()
            .into_iter()
            .map(|x| profile.parse_load_factor_entry(x))
            .collect::<ParseResult<_>>()?,
        remarks: rem_l
            .unwrap_or_default()
            .into_iter()
            .map(|x| profile.parse_remark(x).ok())
            .collect(),
        #[cfg(feature = "polylines")]
        polylines: poly_l
            .map(|x| {
                x.into_iter()
                    .map(|x| profile.parse_polyline(x))
                    .collect::<ParseResult<_>>()
            })
            .transpose()?
            .unwrap_or_default(),
    })
}
