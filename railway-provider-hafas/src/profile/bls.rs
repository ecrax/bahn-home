use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ICE"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("IC/EC"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IR: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("IR"),
        short: Cow::Borrowed("IR"),
    };
    pub const NAHVERKEHR: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Nahverkehr"),
        short: Cow::Borrowed("Nahverkehr"),
    };
    pub const SCHIFF: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("Schiff"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const SEILBAHN: Product = Product {
        mode: Mode::Cablecar,
        name: Cow::Borrowed("Seilbahn"),
        short: Cow::Borrowed("Seilbahn"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };
    pub const AUTOVERLAD: Product = Product {
        // TODO: Correct?
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Autoverlad"),
        short: Cow::Borrowed("Autoverlad"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &IC,
        &IR,
        &NAHVERKEHR,
        &SCHIFF,
        &S,
        &BUS,
        &SEILBAHN,
        &Product::unknown(),
        &Product::unknown(),
        &TRAM,
        &Product::unknown(),
        &AUTOVERLAD,
    ];
}

#[derive(Debug)]
pub struct BlsProfile;

impl Profile for BlsProfile {
    fn url(&self) -> &'static str {
        "https://bls.hafas.de/bin/mgate.exe"
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
        req_json["client"] = json!({"type":"WEB","id":"HAFAS","v":"","name":"webapp"});
        req_json["ver"] = json!("1.46");
        req_json["auth"] = json!({"type":"AID","aid":"3jkAncud78HSoqclmN54812A"});
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
        check_search(BlsProfile {}, "Bayr", "Bayon").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(BlsProfile {}, "8590093", "8578932").await
    }
}
