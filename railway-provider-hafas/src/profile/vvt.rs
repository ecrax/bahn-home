use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const BAHN_AND_S: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Bahn & S-Bahn"),
        short: Cow::Borrowed("Bahn & S"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
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
    // TODO: Deduplicate Bahn and S-Bahn

    pub const PRODUCTS: &[&Product] = &[
        &BAHN_AND_S,
        &BAHN_AND_S,
        &U,
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
pub struct VvtProfile;

impl Profile for VvtProfile {
    fn url(&self) -> &'static str {
        "https://smartride.vvt.at/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Vienna
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
            "name": "webapp"
        });
        req_json["ver"] = json!("1.39");
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
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(VvtProfile {}, "Unter", "Unter-Danegg Ort").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VvtProfile {}, "476152300", "476603100").await
    }
}
