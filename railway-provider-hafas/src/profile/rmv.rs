use crate::{parse::load_factor::HafasLoadFactor, LoadFactor, ParseResult, Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCityExpress/Fernzug"),
        short: Cow::Borrowed("ICE"),
    };
    pub const EC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("EuroCity/InterCity/EuroNight/InterRegio"),
        short: Cow::Borrowed("EC/IC/EN/IR"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("RegionalExpress/Regionalbahn"),
        short: Cow::Borrowed("RE/RB"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Straßenbahn"),
        short: Cow::Borrowed("Tram"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const SCHIFF: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("Schiff"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anruf-Sammel-Taxi"),
        short: Cow::Borrowed("AST"),
    };
    pub const SEILBAHN: Product = Product {
        mode: Mode::Cablecar,
        name: Cow::Borrowed("Seilbahn"),
        short: Cow::Borrowed("Seilbahn"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE, &EC, &RE, &S, &U, &TRAM, &BUS, &BUS, &SCHIFF, &AST, &SEILBAHN,
    ];
}

#[derive(Debug)]
pub struct RmvProfile;

impl Profile for RmvProfile {
    fn url(&self) -> &'static str {
        "https://www.rmv.de/auskunft/bin/jp/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        true
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "type": "WEB",
            "id": "RMV",
            "name": "webapp"
        });
        req_json["ver"] = json!("1.44");
        req_json["ext"] = json!("RMV.1");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "x0k4ZR33ICN9CWmj"
        });
    }

    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        // TODO: Load factors correct?
        match h {
            10 => Ok(LoadFactor::LowToMedium),
            11 => Ok(LoadFactor::High),
            12 => Ok(LoadFactor::VeryHigh),
            13 => Ok(LoadFactor::ExceptionallyHigh),
            5 => Ok(LoadFactor::LowToMedium),
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
        check_search(RmvProfile {}, "Ham", "Hammelburg").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(RmvProfile {}, "3010011", "3011332").await
    }
}
