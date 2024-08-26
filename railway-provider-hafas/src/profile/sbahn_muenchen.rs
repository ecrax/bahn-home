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
        name: Cow::Borrowed("InterCity/EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IRE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Interregio/Schnellzug"),
        short: Cow::Borrowed("IRE"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regio- und Nahverkehr"),
        short: Cow::Borrowed("RE/RB"),
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
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Straßenbahn"),
        short: Cow::Borrowed("Tram"),
    };
    pub const SAMMELTAXI: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anrufsammeltaxi"),
        short: Cow::Borrowed("Sammeltaxi"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &IC,
        &IRE,
        &RE,
        &S,
        &BUS,
        &Product::unknown(),
        &U,
        &TRAM,
        &SAMMELTAXI,
    ];
}

#[derive(Debug)]
pub struct SBahnMuenchenProfile;

impl Profile for SBahnMuenchenProfile {
    fn url(&self) -> &'static str {
        "https://s-bahn-muenchen.hafas.de/bin/540/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "en"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("ggnvMVV8RTt67gh1")
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
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
        req_json["client"] =
            json!({"type":"IPH","id":"DB-REGIO-MVV","v":"5010100","name":"MuenchenNavigator"});
        req_json["ver"] = json!("1.34");
        req_json["ext"] = json!("DB.R15.12.a");
        req_json["auth"] = json!({"type":"AID","aid":"d491MVVhz9ZZts23"});
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
        check_search(SBahnMuenchenProfile {}, "Arena", "Allianz-Arena, München").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(SBahnMuenchenProfile {}, "8004158", "8000261").await
    }
}
