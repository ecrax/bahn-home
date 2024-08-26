use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const TGV: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("TGV, ICE, EuroCity"),
        short: Cow::Borrowed("TGV/ICE/EC"),
    };
    pub const LOCAL: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("local trains"),
        short: Cow::Borrowed("local"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("tram"),
        short: Cow::Borrowed("tram"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("bus"),
        short: Cow::Borrowed("bus"),
    };
    pub const FUN: Product = Product {
        mode: Mode::Cablecar,
        name: Cow::Borrowed("Fun"),
        short: Cow::Borrowed("Fun"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &TGV,
        &TGV,
        &Product::unknown(),
        &LOCAL,
        &LOCAL,
        &BUS,
        &Product::unknown(),
        &Product::unknown(),
        &TRAM,
        &FUN,
    ];
}

#[derive(Debug)]
pub struct CflProfile;

impl Profile for CflProfile {
    fn url(&self) -> &'static str {
        "https://horaires.cfl.lu/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "fr"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Luxembourg
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] =
            json!({"type":"IPH","id":"HAFAS","v":"4000000","name":"cflPROD-STORE"});
        req_json["ver"] = json!("1.43");
        req_json["auth"] = json!({"type":"AID","aid":"ALT2vl7LAFDFu2dz"});
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
        check_search(CflProfile {}, "Pari", "PARIS NORD (France)").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(CflProfile {}, "9864348", "8800003").await
    }
}
