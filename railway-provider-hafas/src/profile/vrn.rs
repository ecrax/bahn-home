use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("regional train"),
        short: Cow::Borrowed("RE/RB"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("urban train"),
        short: Cow::Borrowed("S"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("subway"),
        short: Cow::Borrowed("U"),
    };
    pub const TRAM: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("tram"),
        short: Cow::Borrowed("Tram"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const TAXI: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("dial-a-ride"),
        short: Cow::Borrowed("taxi"),
    };
    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("long-distance train"),
        short: Cow::Borrowed("ICE/IC/EC/EN"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &ICE,
        &ICE,
        &RE,
        &S,
        &BUS,
        &Product::unknown(),
        &U,
        &TRAM,
        &TAXI,
    ];
}

#[derive(Debug)]
pub struct VrnProfile;

impl Profile for VrnProfile {
    fn url(&self) -> &'static str {
        "https://vrn.hafas.de/bin/mgate.exe"
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
        req_json["client"] = json!({"type":"IPH","id":"DB-REGIO-VRN","v":"6000400","name":"VRN"});
        req_json["ver"] = json!("1.34");
        req_json["ext"] = json!("DB.R19.04.a");
        req_json["auth"] = json!({"type":"AID","aid":"p091VRNZz79KtUz5"});
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
        check_search(VrnProfile {}, "Frei", "Freinsheim").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VrnProfile {}, "8000236", "8003932").await
    }
}
