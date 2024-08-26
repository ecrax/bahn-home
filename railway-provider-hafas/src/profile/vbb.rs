use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

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
    pub const T: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("T"),
    };
    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };
    pub const F: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Fähre"),
        short: Cow::Borrowed("F"),
    };
    pub const E: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("IC/ICE"),
        short: Cow::Borrowed("E"),
    };
    pub const R: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("RB/RE"),
        short: Cow::Borrowed("R"),
    };

    pub const PRODUCTS: &[&Product] = &[&S, &U, &T, &B, &F, &E, &R];
}

#[derive(Debug)]
pub struct VbbProfile;

impl Profile for VbbProfile {
    fn url(&self) -> &'static str {
        "https://fahrinfo.vbb.de/bin/mgate.exe"
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
            "id": "VBB",
            "name": "VBB WebApp",
            "l": "vs_webapp_vbb"
        });
        req_json["ver"] = json!("1.45");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "hafas-vbb-webapp"
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
        check_search(VbbProfile {}, "Rose", "U Rosenthaler Platz (Berlin)").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VbbProfile {}, "900003201", "900024101").await
    }
}
