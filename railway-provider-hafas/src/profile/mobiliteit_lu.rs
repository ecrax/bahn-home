use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const TGV: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("local train (TGV/ICE)"),
        short: Cow::Borrowed("TGV/ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("national train (IC/RE/IRE)"),
        short: Cow::Borrowed("IC/RE/IRE"),
    };
    pub const RB: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("local train (RB/TER)"),
        short: Cow::Borrowed("RB/TER"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &TGV,
        &IC,
        &IC,
        &RB,
        &Product::unknown(),
        &BUS,
        &Product::unknown(),
        &Product::unknown(),
        &TRAM,
    ];
}

#[derive(Debug)]
pub struct MobiliteitLuProfile;

impl Profile for MobiliteitLuProfile {
    fn url(&self) -> &'static str {
        "https://cdt.hafas.de/gate"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Luxembourg
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
            "id": "MMILUX",
            "name": "webapp"
        });
        req_json["ver"] = json!("1.43");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "SkC81GuwuzL4e0"
        });
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
        check_search(MobiliteitLuProfile {}, "Lux", "Luxembourg, Wallis").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(MobiliteitLuProfile {}, "160904011", "200405060").await
    }
}
