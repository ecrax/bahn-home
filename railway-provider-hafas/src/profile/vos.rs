use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ICE"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("IC/EC"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IR: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("IR, sonstiger Schnellzug"),
        short: Cow::Borrowed("IR"),
    };
    pub const NAHVERKEHR: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Nahverkehr"),
        short: Cow::Borrowed("Nahverkehr"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
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
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anrufverkehr"),
        short: Cow::Borrowed("AST"),
    };

    pub const PRODUCTS: &[&Product] =
        &[&ICE, &IC, &IR, &NAHVERKEHR, &S, &BUS, &SCHIFF, &U, &T, &AST];
}

#[derive(Debug)]
pub struct VosProfile;

impl Profile for VosProfile {
    fn url(&self) -> &'static str {
        "https://fahrplan.vos.info/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
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
        req_json["client"] = json!({"type":"WEB","id":"SWO","name":"webapp"});
        req_json["ver"] = json!("1.42");
        req_json["auth"] = json!({"type":"AID","aid":"PnYowCQP7Tp1V"});
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
        check_search(VosProfile {}, "Frer", "Freren Markt").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VosProfile {}, "9071733", "9071574").await
    }
}
