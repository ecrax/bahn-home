use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const REGIONAL_TRAIN: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("regional train"),
        short: Cow::Borrowed("regional train"),
    };
    pub const URBAN_TRAIN: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("urban train"),
        short: Cow::Borrowed("urban train"),
    };
    pub const SUBWAY: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("subway"),
        short: Cow::Borrowed("subway"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("tram"),
        short: Cow::Borrowed("tram"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("bus"),
        short: Cow::Borrowed("bus"),
    };
    pub const DIAL_A_RIDE: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("dial-a-ride"),
        short: Cow::Borrowed("dial-a-ride"),
    };
    pub const LONG_DISTANCE_TRAIN: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("long-distance train"),
        short: Cow::Borrowed("long-distance train"),
    };
    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ICE"),
        short: Cow::Borrowed("ICE"),
    };
    pub const EC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("EC/IC"),
        short: Cow::Borrowed("EC/IC"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &EC,
        &LONG_DISTANCE_TRAIN,
        &REGIONAL_TRAIN,
        &URBAN_TRAIN,
        &BUS,
        &Product::unknown(),
        &SUBWAY,
        &TRAM,
        &DIAL_A_RIDE,
    ];
}

#[derive(Debug)]
pub struct MobilNrwProfile;

impl Profile for MobilNrwProfile {
    fn url(&self) -> &'static str {
        "https://nrw.hafas.de/bin/mgate.exe"
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
        req_json["client"] = json!({"type":"IPH","id":"DB-REGIO-NRW","v":"6000300","name":"NRW"});
        req_json["ver"] = json!("1.34");
        req_json["auth"] = json!({"type":"AID","aid":"Kdf0LNRWYg5k3499"});
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
        check_search(MobilNrwProfile {}, "Kreu", "Kreuztal").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(MobilNrwProfile {}, "8000076", "8000001").await
    }
}
