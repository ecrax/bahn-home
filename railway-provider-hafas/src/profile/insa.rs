use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCityExpress"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("RegionalExpress & RegionalBahn"),
        short: Cow::Borrowed("RE/RB"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const T: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("T"),
    };
    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };
    pub const TT: Product = Product {
        // TODO: Correct?
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Tourism Train"),
        short: Cow::Borrowed("TT"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &Product::unknown(), &RE, &S, &T, &B, &B, &TT];
}

#[derive(Debug)]
pub struct InsaProfile;

impl Profile for InsaProfile {
    fn url(&self) -> &'static str {
        "https://reiseauskunft.insa.de/bin/mgate.exe"
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

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "type": "IPH",
            "id": "NASA",
            "v": "4000200",
            "name": "nasaPROD"
        });
        req_json["ver"] = json!("1.44");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "nasa-apps"
        });
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
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
        check_search(InsaProfile {}, "Main", "Mainz Hbf").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(InsaProfile {}, "8000240", "8000105").await
    }
}
