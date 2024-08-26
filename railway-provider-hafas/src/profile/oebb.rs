use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: Fix weird POIs, Movement

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCityExpress & RailJet"),
        short: Cow::Borrowed("ICE/RJ"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const D: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Durchgangszug & EuroNight"),
        short: Cow::Borrowed("D/EN"),
    };
    pub const R: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional & RegionalExpress"),
        short: Cow::Borrowed("R/REX"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };
    pub const F: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Ferry"),
        short: Cow::Borrowed("F"),
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
    pub const ON_CALL: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("on-call transit, lifts, etc"),
        short: Cow::Borrowed("on-call/lift"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &IC,
        &IC,
        &D,
        &R,
        &S,
        &B,
        &F,
        &U,
        &T,
        &Product::unknown(),
        &ON_CALL,
        &D,
    ];
}

#[derive(Debug)]
pub struct OebbProfile;

impl Profile for OebbProfile {
    fn url(&self) -> &'static str {
        "https://fahrplan.oebb.at/bin/mgate.exe"
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
            "type": "IPH",
            "id": "OEBB",
            "v": "6030600",
            "name": "oebbPROD-ADHOC"
        });
        req_json["ver"] = json!("1.41");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "OWDL4fE4ixNiPBBm"
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
        check_search(OebbProfile {}, "Bri", "Brixlegg").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(OebbProfile {}, "1191801", "1190100").await
    }
}
