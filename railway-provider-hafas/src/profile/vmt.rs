use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("long-distance train"),
        short: Cow::Borrowed("ICE/IC/EC"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("regional train"),
        short: Cow::Borrowed("RE/RB"),
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

    pub const PRODUCTS: &[&Product] = &[
        &ICE,
        &ICE,
        &ICE,
        &RE,
        &RE,
        &TRAM,
        &Product::unknown(),
        &Product::unknown(),
        &BUS,
    ];
}

#[derive(Debug)]
pub struct VmtProfile;

impl Profile for VmtProfile {
    fn url(&self) -> &'static str {
        "https://vmt.hafas.de/bin/ticketing/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "de"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("7x8d3n2a5m1b3c6z")
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
        req_json["client"] = json!({"type":"IPH","id":"HAFAS","v":"2040100","name":"VMT"});
        req_json["ver"] = json!("1.34");
        req_json["auth"] = json!({"type":"AID","aid":"t2h7u1e6r4i8n3g7e0n"});
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
        check_search(VmtProfile {}, "Zeil", "Zeilfeld").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(VmtProfile {}, "190014", "167280").await
    }
}
