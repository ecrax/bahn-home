use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    // TODO: public-transport-enabler differentiates between 1 (high-speed) and 2 (suburban).

    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("Bahn & S-Bahn"),
        short: Cow::Borrowed("S/Zug"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const STR: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Strassenbahn"),
        short: Cow::Borrowed("Str"),
    };
    pub const FERNBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Fernbus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const REGIONALBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Regionalbus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const STADTBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Stadtbus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const SEIL_: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Seil-/Zahnradbahn"),
        short: Cow::Borrowed("Seil-/Zahnradbahn"),
    };
    pub const F: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("F"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &S,
        &S,
        &U,
        &Product::unknown(),
        &STR,
        &FERNBUS,
        &REGIONALBUS,
        &STADTBUS,
        &SEIL_,
        &F,
    ];
}

#[derive(Debug)]
pub struct SvvProfile;

impl Profile for SvvProfile {
    fn url(&self) -> &'static str {
        "https://fahrplan.salzburg-verkehr.at/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Vienna
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
        req_json["client"] = json!({"type":"WEB","id":"VAO","v":"","name":"webapp"});
        req_json["ver"] = json!("1.39");
        req_json["ext"] = json!("VAO.11");
        req_json["auth"] = json!({"type":"AID","aid":"wf7mcf9bv3nv8g5f"});
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
        check_search(SvvProfile {}, "Sal", "Salzburg Hauptbahnhof").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(SvvProfile {}, "455086100", "455082100").await
    }
}
