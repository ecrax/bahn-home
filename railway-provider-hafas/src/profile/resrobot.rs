use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };

    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Train"),
        short: Cow::Borrowed("IC"),
    };

    pub const IC_BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Express Bus"),
        short: Cow::Borrowed("IC_Bus"),
    };

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Highspeed"),
        short: Cow::Borrowed("ICE"),
    };

    pub const NIGHT_TRAIN: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Night train"),
        short: Cow::Borrowed("NightTrain"),
    };

    pub const PLANE: Product = Product {
        mode: Mode::Unknown,
        name: Cow::Borrowed("Air"),
        short: Cow::Borrowed("Plane"),
    };

    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("Commuter"),
        short: Cow::Borrowed("S"),
    };

    pub const SHIP: Product = Product {
        mode: Mode::Unknown,
        name: Cow::Borrowed("Ferry"),
        short: Cow::Borrowed("Ship"),
    };

    pub const TAXI: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Taxi"),
        short: Cow::Borrowed("Taxi"),
    };

    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };

    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("Metro"),
        short: Cow::Borrowed("U"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &PLANE,
        &ICE,
        &IC,
        &IC_BUS,
        &S,
        &U,
        &TRAM,
        &BUS,
        &SHIP,
        &TAXI,
        &NIGHT_TRAIN,
    ];
}

#[derive(Debug)]
pub struct ResrobotProfile;

impl Profile for ResrobotProfile {
    fn url(&self) -> &'static str {
        "https://reseplanerare.resrobot.se/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "sv"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Stockholm
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
        req_json["client"] = json!({"type":"WEB","id":"SAMTRAFIKEN","v":10001,"name":"webapp"});
        req_json["ver"] = json!("1.73");
        req_json["auth"] = json!({"type":"AID","aid":"h5o3n7f4t2m8l9x1"});
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        "SEK"
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(ResrobotProfile {}, "Stock", "Stockholm City station").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        // Stockholm City -> Arlanda C
        check_journey(ResrobotProfile {}, "740001617", "740000556").await
    }
}
