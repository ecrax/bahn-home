use crate::{LoadFactor, ParseResult, Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

use super::HafasLoadFactor;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const IC: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("InterCity"),
        short: Cow::Borrowed("IC"),
    };
    pub const ICL: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("ICL"),
        short: Cow::Borrowed("ICL"),
    };
    pub const RE: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regional"),
        short: Cow::Borrowed("RE"),
    };
    pub const S: Product = Product {
        mode: Mode::SuburbanTrain,
        name: Cow::Borrowed("S-Tog A/B/Bx/C/E/F/H"),
        short: Cow::Borrowed("S"),
    };
    pub const BUS: Product = Product {
        mode: Mode::Bus,
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };

    pub const PRODUCTS: &[&Product] = &[&IC, &ICL, &RE, &Product::unknown(), &S, &BUS];
}

#[derive(Debug)]
pub struct RejseplanenProfile;

impl Profile for RejseplanenProfile {
    fn url(&self) -> &'static str {
        "https://mobilapps.rejseplanen.dk/bin/iphone.exe"
    }
    fn language(&self) -> &'static str {
        "dk"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Copenhagen
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
        req_json["client"] = json!({"type":"AND","id":"DK","v":"","name":""});
        req_json["ver"] = json!("1.43");
        req_json["ext"] = json!("DK.9");
        req_json["auth"] = json!({"type":"AID","aid":"irkmpm9mdznstenr-android"});
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        "EUR"
    }

    fn parse_load_factor(&self, h: HafasLoadFactor) -> ParseResult<LoadFactor> {
        // TODO: Load factors correct?
        match h {
            5 => Ok(LoadFactor::LowToMedium),
            11 => Ok(LoadFactor::High),
            12 => Ok(LoadFactor::VeryHigh),
            13 => Ok(LoadFactor::ExceptionallyHigh),
            _ => Err(format!("Invalid load factor: {}", h).into()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::profile::test::{check_journey, check_search};

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(RejseplanenProfile {}, "Rej", "Rejsby St.").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(RejseplanenProfile {}, "8600626", "8600020").await
    }
}
