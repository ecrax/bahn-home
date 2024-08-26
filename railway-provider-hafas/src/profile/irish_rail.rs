use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity"),
        short: Cow::Borrowed("IC"),
    };
    pub const COMMUTER: Product = Product {
        // TODO: Correct?
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Commuter"),
        short: Cow::Borrowed("Commuter"),
    };
    pub const DART: Product = Product {
        // TODO: Correct?
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Dublin Area Rapid Transit"),
        short: Cow::Borrowed("DART"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const LUAS: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("LUAS Tram"),
        short: Cow::Borrowed("LUAS"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &Product::unknown(),
        &IC,
        &Product::unknown(),
        &COMMUTER,
        &DART,
        &BUS,
        &LUAS,
    ];
}

#[derive(Debug)]
pub struct IrishRailProfile;

impl Profile for IrishRailProfile {
    fn url(&self) -> &'static str {
        "https://journeyplanner.irishrail.ie/bin/mgate.exe"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("i5s7m3q9z6b4k1c2")
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Dublin
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }
    fn mic_mac(&self) -> bool {
        true
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "type": "IPA",
            "id": "IRISHRAIL",
            "v": "4000100",
            "name": "IrishRailPROD-APPSTORE"
        });
        req_json["ver"] = json!("1.33");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "P9bplgVCGnozdgQE"
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
        check_search(IrishRailProfile {}, "Ske", "Skerries").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(IrishRailProfile {}, "9909002", "9990840").await
    }
}
