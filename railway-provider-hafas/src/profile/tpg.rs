use crate::{Product, Profile};
use serde_json::{json, Value};
use std::collections::HashMap;

mod products {
    use crate::{Mode, Product};
    use std::borrow::Cow;

    pub const TGV: Product = Product {
        id: Cow::Borrowed("tgv"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[1]),
        name: Cow::Borrowed("TGV"),
        short: Cow::Borrowed("TGV"),
    };
    pub const INTERCITES: Product = Product {
        id: Cow::Borrowed("intercites"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[2]),
        name: Cow::Borrowed("Intercites"),
        short: Cow::Borrowed("Intercites"),
    };
    pub const IR: Product = Product {
        id: Cow::Borrowed("ir"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[4]),
        name: Cow::Borrowed("IR"),
        short: Cow::Borrowed("IR"),
    };
    pub const TRAIN_DIRECT: Product = Product {
        id: Cow::Borrowed("train-direct"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[8]),
        name: Cow::Borrowed("Train direct"),
        short: Cow::Borrowed("Train direct"),
    };
    pub const BATEAU: Product = Product {
        id: Cow::Borrowed("bateau"),
        mode: Mode::Watercraft,
        bitmasks: Cow::Borrowed(&[16]),
        name: Cow::Borrowed("Bateau"),
        short: Cow::Borrowed("Bateau"),
    };
    pub const REGIO_EXPRESS: Product = Product {
        id: Cow::Borrowed("regio-express"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[32]),
        name: Cow::Borrowed("Regio Express"),
        short: Cow::Borrowed("Regio Express"),
    };
    pub const BUS: Product = Product {
        id: Cow::Borrowed("bus"),
        mode: Mode::Bus,
        bitmasks: Cow::Borrowed(&[64]),
        name: Cow::Borrowed("Bus"),
        short: Cow::Borrowed("Bus"),
    };
    pub const TRANSPORT_A_CABLES: Product = Product {
        id: Cow::Borrowed("transport-a-cables"),
        mode: Mode::Gondola,
        bitmasks: Cow::Borrowed(&[128]),
        name: Cow::Borrowed("Transport a cables"),
        short: Cow::Borrowed("Transport a cables"),
    };
    pub const TRAM: Product = Product {
        id: Cow::Borrowed("tram"),
        mode: Mode::Train,
        bitmasks: Cow::Borrowed(&[512]),
        name: Cow::Borrowed("Tram"),
        short: Cow::Borrowed("Tram"),
    };

    pub const PRODUCTS: &[&Product] = &[
        &TGV,
        &INTERCITES,
        &IR,
        &TRAIN_DIRECT,
        &BATEAU,
        &REGIO_EXPRESS,
        &BUS,
        &TRANSPORT_A_CABLES,
        &TRAM,
    ];
}

#[derive(Debug)]
pub struct TpgProfile;

impl Profile for TpgProfile {
    fn url(&self) -> &'static str {
        "https://tpg-webapp.hafas.de/bin/mgate.exe"
    }
    fn language(&self) -> &'static str {
        "fr"
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
        req_json["client"] = json!({"type":"WEB","id":"HAFAS","v":"","name":"webapp"});
        req_json["ver"] = json!("1.40");
        req_json["auth"] = json!({"type":"AID","aid":"9CZsdl5PqX8n5D6b"});
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
    use crate::{
        api::journeys::JourneysOptions, client::HafasClient,
        requester::hyper::HyperRustlsRequester, Place, Stop,
    };
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn test_path_available() -> Result<(), Box<dyn Error>> {
        let client = HafasClient::new(TpgProfile {}, HyperRustlsRequester::new());
        let journeys = client
            .journeys(
                Place::Stop(Stop {
                    id: "100449".to_string(),
                    ..Default::default()
                }),
                Place::Stop(Stop {
                    id: "100451".to_string(),
                    ..Default::default()
                }),
                JourneysOptions::default(),
            )
            .await?;
        assert!(!journeys.journeys.is_empty());
        Ok(())
    }
}
