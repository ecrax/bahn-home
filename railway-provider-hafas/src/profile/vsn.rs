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
        name: Cow::Borrowed("Fernzug"),
        short: Cow::Borrowed("IC/EC/CNL"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("RegionalExpress & InterRegio"),
        short: Cow::Borrowed("RE/IR"),
    };
    pub const NV: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Nahverhehr"),
        short: Cow::Borrowed("NV"),
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
    pub const F: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Schiff"),
        short: Cow::Borrowed("F"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const T: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Straßen-/Stadtbahn"),
        short: Cow::Borrowed("T"),
    };
    pub const AST: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Anruf-Sammel-Taxi"),
        short: Cow::Borrowed("AST"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &RE, &NV, &S, &BUS, &F, &U, &T, &AST];
}

#[derive(Debug)]
pub struct VsnProfile;

impl Profile for VsnProfile {
    fn url(&self) -> &'static str {
        "https://fahrplaner.vsninfo.de/hafas/mgate.exe"
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
        req_json["client"] = json!({"type":"IPA","id":"VSN","v":"5030100","name":"vsn"});
        req_json["ver"] = json!("1.42");
        req_json["auth"] = json!({"type":"AID","aid":"Mpf5UPC0DmzV8jkg"});
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
        check_search(VsnProfile {}, "Morin", "Moringen Schule").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VsnProfile {}, "9014418", "9093627").await
    }
}
