use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    // TODO
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const STADTBAHN: Product = Product {
        // TODO: Maybe suburban?
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Stadtbahn"),
        short: Cow::Borrowed("Stadtbahn"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const TAXIBUS: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Taxibus"),
        short: Cow::Borrowed("Taxibus"),
    };
    // TODO: This is bitmask 46 for some reason. Why?
    // pub const REGIONAL: Product = Product {
    //     mode: Mode::RegionalTrain,
    //     name: Cow::Borrowed("Regionalverkehr"),
    //     short: Cow::Borrowed("Regionalverkehr"),
    // };
    pub const FERN: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Fernverkehr"),
        short: Cow::Borrowed("Fernverkehr"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &S,
        &STADTBAHN,
        &Product::unknown(),
        &BUS,
        &Product::unknown(),
        &FERN,
        &Product::unknown(),
        &Product::unknown(),
        &TAXIBUS,
    ];
}

#[derive(Debug)]
pub struct KvbProfile;

impl Profile for KvbProfile {
    fn url(&self) -> &'static str {
        "https://auskunft.kvb.koeln/gate"
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
            "id": "HAFAS",
            "name": "webapp",
            "l": "vs_webapp"
        });
        req_json["ver"] = json!("1.42");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "Rt6foY5zcTTRXMQs"
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
        check_search(KvbProfile {}, "Hauptbahn", "Köln Hbf").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(KvbProfile {}, "900593000", "900000687").await
    }
}
