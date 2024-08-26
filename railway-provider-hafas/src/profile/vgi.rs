use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const TRAIN_EXP: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("High-speed train"),
        short: Cow::Borrowed("Train"),
    };
    pub const TRAIN_REG: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional train"),
        short: Cow::Borrowed("Train"),
    };
    pub const ZUG: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Nahverkehrszug"),
        short: Cow::Borrowed("Zug"),
    };
    pub const FERRY: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Ferry"),
        short: Cow::Borrowed("Ferry"),
    };
    pub const SUBWAY: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("Subway"),
        short: Cow::Borrowed("Subway"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };
    pub const ON_DEMAND: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("On-demand traffic"),
        short: Cow::Borrowed("on demand"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &BUS, &TRAIN_EXP, &TRAIN_REG, &ZUG, &BUS, &FERRY, &SUBWAY, &TRAM, &ON_DEMAND,
    ];
}

#[derive(Debug)]
pub struct VgiProfile;

impl Profile for VgiProfile {
    fn url(&self) -> &'static str {
        "https://fpa.invg.de/bin/mgate.exe"
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
            json!({"type":"IPH","id":"INVG","v":"1040000","name":"invgPROD-APPSTORE-LIVE"});
        req_json["ver"] = json!("1.39");
        req_json["auth"] = json!({"type":"AID","aid":"GITvwi3BGOmTQ2a5"});
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
        check_search(VgiProfile {}, "Ingol", "Ingolstadt Audi").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VgiProfile {}, "8000183", "84999").await
    }
}
