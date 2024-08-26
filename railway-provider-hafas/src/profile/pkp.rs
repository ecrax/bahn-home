use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: Trim stop name

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const EIC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ExpressInterCity & ExpressInterCity Premium & InterCityExpress"),
        short: Cow::Borrowed("EIC/EIP/ICE"),
    };
    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity & Twoje Linie Kolejowe & EuroCity & EuroNight"),
        short: Cow::Borrowed("IC/TLK/EC/EN"),
    };
    pub const R: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional"),
        short: Cow::Borrowed("R"),
    };
    pub const B: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };

    pub const PRODUCTS: &[&Product] = &[&EIC, &EIC, &IC, &R, &Product::unknown(), &B];
}

#[derive(Debug)]
pub struct PkpProfile;

impl Profile for PkpProfile {
    fn url(&self) -> &'static str {
        "https://mobil.rozklad-pkp.pl:8019/bin/mgate.exe"
    }

    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Warsaw
    }

    fn language(&self) -> &'static str {
        "pl"
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "type": "AND",
            "id": "HAFAS"
        });
        req_json["ver"] = json!("1.21");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "DrxJYtYZQpEBCtcb"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "Dalvik/2.1.0");
    }

    fn price_currency(&self) -> &'static str {
        "PLN"
    }
}

#[cfg(test)]
mod test {
    use crate::profile::test::{check_journey, check_search};
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(PkpProfile {}, "Warsa", "Warsaw Airport").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(PkpProfile {}, "5100069", "5100028").await
    }
}
