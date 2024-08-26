use crate::LoadFactor;
use crate::ParseResult;
use crate::Profile;
use crate::TariffClass;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HafasTariffClass {
    First,
    Second,
}

impl From<HafasTariffClass> for TariffClass {
    fn from(h: HafasTariffClass) -> TariffClass {
        match h {
            HafasTariffClass::First => TariffClass::First,
            HafasTariffClass::Second => TariffClass::Second,
        }
    }
}

pub type HafasLoadFactor = u8;

#[derive(Debug, Clone, Deserialize)]
pub struct HafasLoadFactorEntry {
    c: HafasTariffClass,
    r: HafasLoadFactor,
}

#[derive(Debug, Clone)]
pub struct LoadFactorEntry {
    pub class: TariffClass,
    pub load: LoadFactor,
}

pub fn default_parse_load_factor_entry<P: Profile + ?Sized>(
    profile: &P,
    h: HafasLoadFactorEntry,
) -> ParseResult<LoadFactorEntry> {
    Ok(LoadFactorEntry {
        class: h.c.into(),
        load: profile.parse_load_factor(h.r)?,
    })
}

pub fn default_parse_load_factor(h: HafasLoadFactor) -> ParseResult<LoadFactor> {
    match h {
        1 => Ok(LoadFactor::LowToMedium),
        2 => Ok(LoadFactor::High),
        3 => Ok(LoadFactor::VeryHigh),
        4 => Ok(LoadFactor::ExceptionallyHigh),
        _ => Err(format!("Invalid load factor: {}", h).into()),
    }
}
