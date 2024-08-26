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
        name: Cow::Borrowed("InterCity, EuroCity, CityNightLine, InterRegio"),
        short: Cow::Borrowed("IC/EC/CNL/IR"),
    };
    pub const NAHV: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Nahverkehr"),
        short: Cow::Borrowed("Nahv"),
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
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anrufverkehr"),
        short: Cow::Borrowed("AST"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &IC, &NAHV, &S, &BUS, &SCHIFF, &U, &TRAM, &AST];
}

#[derive(Debug)]
pub struct VbnProfile;

impl Profile for VbnProfile {
    fn url(&self) -> &'static str {
        "https://fahrplaner.vbn.de/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("SP31mBufSyCLmNxp")
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
        req_json["client"] = json!({"type":"IPH","id":"VBN","v":"6000000","name":"vbn"});
        req_json["ver"] = json!("1.42");
        req_json["auth"] = json!({"type":"AID","aid":"kaoxIXLn03zCr2KR"});
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
        check_search(VbnProfile {}, "Brem", "Bremerhaven Hbf").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VbnProfile {}, "9014418", "9093627").await
    }
}
