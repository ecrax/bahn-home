use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const BAHN_AND_S_BAHN: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Bahn & S-Bahn"),
        short: Cow::Borrowed("Bahn & S-Bahn"),
    };
    pub const U_BAHN: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U-Bahn"),
    };
    pub const STRASSENBAHN: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Straßenbahn"),
        short: Cow::Borrowed("Straßenbahn"),
    };
    pub const STADTBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Stadtbus"),
        short: Cow::Borrowed("Stadtbus"),
    };
    pub const REGIONALBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Regionalbus"),
        short: Cow::Borrowed("Regionalbus"),
    };
    pub const FERNBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Fernbus"),
        short: Cow::Borrowed("Fernbus"),
    };
    pub const SONSTIGE_BUSSE: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("sonstige Busse"),
        short: Cow::Borrowed("sonstige Busse"),
    };
    pub const SEIL_: Product = Product {
        mode: Mode::Cablecar,
        name: Cow::Borrowed("Seil-/Zahnradbahn"),
        short: Cow::Borrowed("Seil-/Zahnradbahn"),
    };
    pub const SCHIFF: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("Schiff"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anrufsammeltaxi"),
        short: Cow::Borrowed("AST"),
    };
    // TODO: Deduplicate bahn and s-bahn

    pub const PRODUCTS: &[&Product] = &[
        &BAHN_AND_S_BAHN,
        &BAHN_AND_S_BAHN,
        &U_BAHN,
        &Product::unknown(),
        &STRASSENBAHN,
        &FERNBUS,
        &REGIONALBUS,
        &STADTBUS,
        &SEIL_,
        &SCHIFF,
        &AST,
        &SONSTIGE_BUSSE,
    ];
}

#[derive(Debug)]
pub struct OoevvProfile;

impl Profile for OoevvProfile {
    fn url(&self) -> &'static str {
        "https://app.verkehrsauskunft.at/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Vienna
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("6633673735743766726667323938336A")
    }
    fn mic_mac(&self) -> bool {
        true
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        true
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({"type":"AND","id":"VAO"});
        req_json["ver"] = json!("1.27");
        req_json["ext"] = json!("VAO.11");
        req_json["auth"] = json!({"type": "USER", "aid": "and20201hf7mcf9bv3nv8g5f", "user": "mobile", "pw": "87a6f8ZbnBih32"});
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
        check_search(OoevvProfile {}, "Vien", "Vienna Hauptbahnhof").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(OoevvProfile {}, "444670100", "431507400").await
    }
}
