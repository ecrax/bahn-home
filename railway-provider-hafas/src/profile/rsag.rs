use crate::{LoadFactor, ParseResult, Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

use super::HafasLoadFactor;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCityExpress"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IR: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterRegio/high-speed train"),
        short: Cow::Borrowed("IR/other"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("regional train"),
        short: Cow::Borrowed("RE/RB"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };
    pub const F: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("F"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const T: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("T"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Taxi/on-call vehicle"),
        short: Cow::Borrowed("AST"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &IR, &RE, &S, &B, &F, &U, &T, &AST];
}

#[derive(Debug)]
pub struct RsagProfile;

impl Profile for RsagProfile {
    fn url(&self) -> &'static str {
        "https://fahrplan.rsag-online.de/bin/mgate.exe"
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
        req_json["client"] = json!({"type":"WEB","id":"RSAG","v":"","name":"webapp"});
        req_json["ver"] = json!("1.42");
        req_json["ext"] = json!("VBN.2");
        req_json["auth"] = json!({"type":"AID","aid":"tF5JTs25rzUhGrrl"});
    }

    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        match h {
            11 => Ok(LoadFactor::LowToMedium),
            12 => Ok(LoadFactor::High),
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
        check_search(RsagProfile {}, "Bri", "Bristow").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(RsagProfile {}, "8010304", "8010153").await
    }
}
