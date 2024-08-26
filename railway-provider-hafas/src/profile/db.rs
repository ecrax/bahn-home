use crate::{Product, Profile};
use rcore::{Age, RemarkAssociation};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: parse Ausstattung, Öffnungszeiten, LocWithDetais, LoadFactors, LineWithAdditionalName,
// JourneyWithPrice, Hints, Codes, IBNR
// TODO: transform JourneysQuery

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
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const RE: Product = Product {
        mode: Mode::HighSpeedTrain,
        name: Cow::Borrowed("RegionalExpress & InterRegio"),
        short: Cow::Borrowed("RE/IR"),
    };
    pub const RB: Product = Product {
        mode: Mode::RegionalTrain,
        name: Cow::Borrowed("Regio"),
        short: Cow::Borrowed("RB"),
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
    pub const TAXI: Product = Product {
        mode: Mode::OnDemand,
        name: Cow::Borrowed("Group Taxi"),
        short: Cow::Borrowed("Taxi"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &RE, &RB, &S, &B, &F, &U, &T, &TAXI];
}

#[derive(Debug)]
pub struct DbProfile;

impl Profile for DbProfile {
    fn url(&self) -> &'static str {
        "https://reiseauskunft.bahn.de/bin/mgate.exe"
    }
    fn checksum_salt(&self) -> Option<&'static str> {
        Some("bdI8UVj40K5fvxwf")
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Berlin
    }
    fn refresh_journey_use_out_recon_l(&self) -> bool {
        true
    }
    fn salt(&self) -> bool {
        true
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["svcReqL"][0]["cfg"]["rtMode"] = json!("HYBRID");
        req_json["client"] = json!({
            "id": "DB",
            "v": "19040000",
            "type": "IPH",
            "name": "DB Navigator"
        });
        req_json["ext"] = json!("DB.R20.12.b");
        req_json["ver"] = json!("1.34");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "n91dB8Z77MLdoR0K"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "hafas-rs");
    }

    fn price_currency(&self) -> &'static str {
        "EUR"
    }

    fn remark_association(&self, code: &str) -> RemarkAssociation {
        match code {
            // Bikes limited
            "FB" |
            // Bike free
            "KF" |
            // Bike times limited
            "FS" => RemarkAssociation::Bike,

            // Places for wheelchairs
            "RO" |
            // Accessible equipment
            "RG" | "EA" | "ER" |
            // Ramp for wheelchairs
            "EH" |
            // Boarding aid at center of train
            "ZM" |
            // Accessible only at limited stations
            "SI" => RemarkAssociation::Accessibility,

            // Ticket machine in train
            "FM" | "FZ" |
            // Reservation upfront at service points and vending machines possible
            "RC" => RemarkAssociation::Ticket,

            // Power sockers
            "LS" => RemarkAssociation::Ticket,
            // Air conditioning
            "KL" => RemarkAssociation::AirConditioning,
            // WiFi
            "WV" => RemarkAssociation::WiFi,

            // Only second class
            "K2" => RemarkAssociation::OnlySecondClass,


            // Hamburg mobility info link
            "HM" |
            // Schleswig-Holstein mobility / accesibility info link
            "SM" |
            // RRX Rhein-Ruhr-Express
            "N " |


            // Not specified
            "" => RemarkAssociation::None,
            _ => RemarkAssociation::Unknown
        }
    }

    fn age_to_hafas(&self, age: Age) -> &'static str {
        match age.0 {
            0..=5 => "B",
            6..=14 => "K",
            15..=26 => "E",
            27..=64 => "E", // using "Y" currently yields an error in HAFAS API
            65.. => "E",    // using "Y" currently yields an error in HAFAS API
        }
    }
}

#[cfg(test)]
mod test {
    use crate::profile::test::{check_journey, check_search};
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn Error>> {
        check_search(DbProfile {}, "Bayr", "Bayreuth Hbf").await
    }

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        check_journey(DbProfile {}, "8011167", "8000261").await
    }
}
