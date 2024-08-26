use crate::{parse::load_factor::HafasLoadFactor, LoadFactor, ParseResult, Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCityExpress"),
        short: Cow::Borrowed("ICE"),
    };
    pub const EC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("EuroCity/InterCity"),
        short: Cow::Borrowed("EC/IC"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regionalzug"),
        short: Cow::Borrowed("RE/RB"),
    };
    pub const REGIOTRAM: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("RegioTram"),
        short: Cow::Borrowed("RegioTram"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const SAMMELTAXI: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("AnrufSammelTaxi"),
        short: Cow::Borrowed("Sammeltaxi"),
    };

    // TODO: public-transport-enabler differentiates between 8 (suburban) and 16 (subway)
    // TODO: public-transport-enabler also defines ferries and more regional trains.
    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &EC,
        &RE,
        &REGIOTRAM,
        &REGIOTRAM,
        &TRAM,
        &BUS,
        &BUS,
        &Product::unknown(),
        &SAMMELTAXI,
        &REGIOTRAM,
    ];
}

#[derive(Debug)]
pub struct NvvProfile;

impl Profile for NvvProfile {
    fn url(&self) -> &'static str {
        "https://auskunft.nvv.de/auskunft/bin/app/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        true
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] =
            json!({"type":"IPH","id":"NVV","v":"5000300","name":"NVVMobilPROD_APPSTORE"});
        req_json["ver"] = json!("1.45");
        req_json["ext"] = json!("NVV.6.0");
        req_json["auth"] = json!({"type":"AID","aid":"Kt8eNOH7qjVeSxNA"});
    }

    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        // TODO: Load factors correct?
        match h {
            10 => Ok(LoadFactor::LowToMedium),
            11 => Ok(LoadFactor::High),
            12 => Ok(LoadFactor::VeryHigh),
            13 => Ok(LoadFactor::ExceptionallyHigh),
            _ => Err(format!("Invalid load factor: {}", h).into()),
        }
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        "EUR"
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(NvvProfile {}, "Ban", "Banteln").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(NvvProfile {}, "2200073", "2200042").await
    }
}
