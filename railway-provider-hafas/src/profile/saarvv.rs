use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: Fix movement?

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Hochgeschwindigkeitszug"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IR: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterRegio"),
        short: Cow::Borrowed("IR"),
    };
    pub const RB: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regionalzug"),
        short: Cow::Borrowed("RB"),
    };
    pub const S_BAHN: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S-Bahn"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const S: Product = Product {
        // TODO: Correct?
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Saarbahn"),
        short: Cow::Borrowed("S"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const SCHIFF: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("Schiff"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anruf-Sammel-Taxi"),
        short: Cow::Borrowed("AST"),
    };
    pub const SCHULBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Schulbus"),
        short: Cow::Borrowed("Schulbus"),
    };

    // TODO: `1`, `2`, `4` bitmasks?
    pub const PRODUCTS: &[&Product] = &[
        &Product::unknown(),
        &Product::unknown(),
        &Product::unknown(),
        &SCHULBUS,
        &AST,
        &SCHIFF,
        &BUS,
        &S,
        &U,
        &S_BAHN,
        &RB,
        &IR,
        &IC,
        &ICE,
    ];
}

#[derive(Debug)]
pub struct SaarvvProfile;

impl Profile for SaarvvProfile {
    fn url(&self) -> &'static str {
        "https://saarfahrplan.de/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("HJtlubisvxiJxss")
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({"type":"AND","id":"ZPS-SAAR","v":"","name":"Saarvv"});
        req_json["ver"] = json!("1.40");
        req_json["auth"] = json!({"type":"AID","aid":"51XfsVqgbdA6oXzHrx75jhlocRg6Xe"});
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
        check_search(SaarvvProfile {}, "Norhe", "Norheim").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(SaarvvProfile {}, "15541", "10609").await
    }
}
