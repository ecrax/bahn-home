use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const CABLE_CAR: Product = Product {
        mode: Mode::Cablecar,
        name: Cow::Borrowed("Cable car"),
        short: Cow::Borrowed("Cable car"),
    };
    pub const REGIONAL_TRAIN: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional trains (Caltrain, Capitol Corridor, ACE)"),
        short: Cow::Borrowed("regional trains"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const FERRY: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Ferry"),
        short: Cow::Borrowed("Ferry"),
    };
    pub const BART: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("BART"),
        short: Cow::Borrowed("BART"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &Product::unknown(),
        &Product::unknown(),
        &CABLE_CAR,
        &REGIONAL_TRAIN,
        &Product::unknown(),
        &BUS,
        &FERRY,
        &BART,
        &TRAM,
    ];
}

#[derive(Debug)]
pub struct BartProfile;

impl Profile for BartProfile {
    fn url(&self) -> &'static str {
        "https://planner.bart.gov/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "en"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::America::Los_Angeles
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
            "id": "BART",
            "name": "webapp"
        });
        req_json["ver"] = json!("1.40");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "kEwHkFUCIL500dym"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        "USD"
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(
            BartProfile {},
            "Rye, San Francisco",
            "Rye Bar, San Francisco",
        )
        .await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(BartProfile {}, "100013296", "100013295").await
    }
}
