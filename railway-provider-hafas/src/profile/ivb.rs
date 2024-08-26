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
pub struct IvbProfile;

impl Profile for IvbProfile {
    fn url(&self) -> &'static str {
        "https://fahrplan.ivb.at/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Zurich
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
            "id": "VAO",
            "name": "webapp",
        });
        req_json["ver"] = json!("1.32");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "wf7mcf9bv3nv8g5f"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        "EUR"
    }

    fn custom_pem_bundle(&self) -> Option<&'static [u8]> {
        Some(include_bytes!("./custom-certificates/ivb.crt.pem"))
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(IvbProfile {}, "Hauptbahn", "Wien Hauptbahnhof").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(IvbProfile {}, "490134900", "470118700").await
    }
}
