use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

// TODO: Profiles correct?
mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const ICE: Product = Product {
        id: Cow::Borrowed("nationalExpress"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[1]),
        name: Cow::Borrowed("InterCityExpress"),
        short: Cow::Borrowed("ICE"),
    };
    pub const IC: Product = Product {
        id: Cow::Borrowed("national"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[2]),
        name: Cow::Borrowed("InterCity & EuroCity"),
        short: Cow::Borrowed("IC/EC"),
    };
    pub const RE: Product = Product {
        id: Cow::Borrowed("regionalExp"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[4]),
        name: Cow::Borrowed("RegionalExpress & InterRegio"),
        short: Cow::Borrowed("RE/IR"),
    };
    pub const RB: Product = Product {
        id: Cow::Borrowed("regional"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[8]),
        name: Cow::Borrowed("Regio"),
        short: Cow::Borrowed("RB"),
    };
    pub const S: Product = Product {
        id: Cow::Borrowed("suburban"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[16]),
        name: Cow::Borrowed("S-Bahn"),
        short: Cow::Borrowed("S"),
    };
    pub const B: Product = Product {
        id: Cow::Borrowed("bus"),
        mode: Mode::Bus,
        bitmasks: Cow::Borrowed(&[32]),
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("B"),
    };
    pub const F: Product = Product {
        id: Cow::Borrowed("ferry"),
        mode: Mode::Watercraft,
        bitmasks: Cow::Borrowed(&[64]),
        name: Cow::Borrowed("Ferry"),
        short: Cow::Borrowed("F"),
    };
    pub const U: Product = Product {
        id: Cow::Borrowed("subway"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[128]),
        name: Cow::Borrowed("U-Bahn"),
        short: Cow::Borrowed("U"),
    };
    pub const T: Product = Product {
        id: Cow::Borrowed("tram"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[256]),
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("T"),
    };
    pub const TAXI: Product = Product {
        id: Cow::Borrowed("taxi"),
        mode: Mode::Taxi,
        bitmasks: Cow::Borrowed(&[512]),
        name: Cow::Borrowed("Group Taxi"),
        short: Cow::Borrowed("Taxi"),
    };

    pub const PRODUCTS: &[&Product] = &[&ICE, &IC, &RE, &RB, &S, &B, &F, &U, &T, &TAXI];
}

#[derive(Debug)]
pub struct SncfProfile;

impl Profile for SncfProfile {
    fn url(&self) -> &'static str {
        "https://sncf-maps.hafas.de/bin/maps-ng/mgate.exe"
    }
    fn timezone(&self) -> chrono_tz::Tz {
        chrono_tz::Europe::Paris
    }

    fn products(&self) -> &'static [&'static Product] {
        products::PRODUCTS
    }

    fn prepare_body(&self, req_json: &mut Value) {
        req_json["client"] = json!({
            "id": "SNCF_LIVEMAP",
            "type": "WEB",
            "name": "webapp",
            "l": "vs_webapp"
        });
        req_json["id"] = json!("6tm47gqmkkk7hgcs");
        req_json["ver"] = json!("1.18");
        req_json["auth"] = json!({
            "type": "AID",
            "aid": "hf7mcf9bv3nv8g5f"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str {
        ""
    }
}

#[cfg(test)]
mod test {
    use crate::{
        api::journeys::JourneysOptions, client::HafasClient,
        requester::hyper::HyperRustlsRequester, Place, Stop,
    };
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        let client = HafasClient::new(SncfProfile {}, HyperRustlsRequester::new());
        let journeys = client
            .journeys(
                Place::Stop(Stop {
                    id: "008734243".to_string(),
                    ..Default::default()
                }),
                Place::Stop(Stop {
                    id: "008775774".to_string(),
                    ..Default::default()
                }),
                JourneysOptions::default(),
            )
            .await?;
        assert!(!journeys.journeys.is_empty());
        Ok(())
    }
}
