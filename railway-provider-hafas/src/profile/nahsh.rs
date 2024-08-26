use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: Fix location, Parse journey with tickets, fix movement

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("High-speed rail"),
        short: Cow::Borrowed("ICE/HSR"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const IR: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("Interregional"),
        short: Cow::Borrowed("IR"),
    };
    pub const RB: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional & RegionalExpress"),
        short: Cow::Borrowed("RB/RE"),
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
        name: Cow::Borrowed("On-call transit"),
        short: Cow::Borrowed("on-call"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &IR, &RB, &S, &B, &F, &U, &T, &ON_CALL];
}

#[derive(Debug)]
pub struct NahSHProfile;

impl Profile for NahSHProfile {
    fn url(&self) -> &'static str {
        "https://nah.sh.hafas.de/bin/mgate.exe"
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
    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "type": "IPH",
            "id": "NAHSH",
            "v": "3000700",
            "name": "NAHSHPROD"
        });
        req_json["ver"] = json!("1.30");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "r0Ot9FLFNAFxijLW"
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
        check_search(NahSHProfile {}, "Nahe", "Nahe Dorfstraße").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(NahSHProfile {}, "8000103", "8000199").await
    }
}
