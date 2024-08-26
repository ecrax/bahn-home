use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("MetroBus"),
        short: Cow::Borrowed("B"),
    };
    pub const R: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("MetroRapid"),
        short: Cow::Borrowed("R"),
    };
    pub const M: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("MetroRail"),
        short: Cow::Borrowed("M"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &M,
        &Product::unknown(),
        &B,
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &R,
    ];
}

#[derive(Debug)]
pub struct CmtaProfile;

impl Profile for CmtaProfile {
    fn url(&self) -> &'static str {
        "https://capmetro.hafas.cloud/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "en"
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
        req_json["client"] = json!({"type":"IPH","id":"CMTA","v":"2","name":"CapMetro"});
        req_json["ver"] = json!("1.40");
        req_json["auth"] = json!({"type":"AID","aid":"ioslaskdcndrjcmlsd"});
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
        check_search(CmtaProfile {}, "Plaza", "Plaza Saltillo").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(CmtaProfile {}, "000002370", "000005919").await
    }
}
