use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const REGIONALZUG: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regionalzug"),
        short: Cow::Borrowed("Regionalzug"),
    };
    pub const FERNZUG: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Fernzug"),
        short: Cow::Borrowed("Fernzug"),
    };
    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ICE/Thalys"),
        short: Cow::Borrowed("ICE/Thalys"),
    };
    pub const FERNBUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Fernbus"),
        short: Cow::Borrowed("Fernbus"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const U: Product = Product {
        mode: Mode::Subway,
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const STRASSENBAHN: Product = Product {
        mode: Mode::Tram,
        name: Cow::Borrowed("Straßenbahn"),
        short: Cow::Borrowed("Straßenbahn"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const BUS_V: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus, Verstärkerfahrt"),
        short: Cow::Borrowed("Bus V"),
    };
    pub const BEDARFSVERKEHR: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Bedarfsverkehr"),
        short: Cow::Borrowed("Bedarfsverkehr"),
    };
    pub const FAEHRE: Product = Product {
        mode: Mode::Ferry,
        name: Cow::Borrowed("Fähre"),
        short: Cow::Borrowed("Fähre"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &REGIONALZUG,
        &FERNZUG,
        &ICE,
        &FERNBUS,
        &S,
        &U,
        &STRASSENBAHN,
        &BUS,
        &BUS_V,
        &BEDARFSVERKEHR,
        &FAEHRE,
    ];
}

#[derive(Debug)]
pub struct AvvProfile;

impl Profile for AvvProfile {
    fn url(&self) -> &'static str {
        "https://auskunft.avv.de/bin/mgate.exe"
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
        req_json["client"] = json!({"type":"WEB","id":"AVV_AACHEN","v":"","name":"webapp"});
        req_json["ver"] = json!("1.26");
        req_json["auth"] = json!({"type":"AID","aid":"4vV1AcH3N511icH"});
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
        check_search(AvvProfile {}, "Bayr", "Bayernallee").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(AvvProfile {}, "1057", "1397").await
    }
}
