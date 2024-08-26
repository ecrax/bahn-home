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

    pub const PRODUCTS: &[&Product] = &[
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &BUS,
    ];
}

#[derive(Debug)]
pub struct DartProfile;

impl Profile for DartProfile {
    fn url(&self) -> &'static str {
        "https://dart.hafas.de/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "en"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        None
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::America::Chicago
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
        req_json["ver"] = json!("1.35");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "XNFGL2aSkxfDeK8N4waOZnsdJ"
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
        check_search(DartProfile {}, "Main", "MAIN AVE/5TH ST").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(DartProfile {}, "100002702", "100004972").await
    }
}
