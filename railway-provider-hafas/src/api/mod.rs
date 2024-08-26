use async_trait::async_trait;
use chrono::Utc;
use rcore::{
    Journey, JourneysOptions, JourneysResponse, LocationsOptions, LocationsResponse, LoyaltyCard,
    Place, Provider, RefreshJourneyOptions, RefreshJourneyResponse, Requester, TransferOptions,
};
use serde_json::json;

use crate::{
    client::HafasClient,
    format::ToHafas,
    parse::{journeys_response::HafasJourneysResponse, locations_response::HafasLocationsResponse},
};

#[cfg_attr(feature = "rt-multi-thread", async_trait)]
#[cfg_attr(not(feature = "rt-multi-thread"), async_trait(?Send))]
impl<R: Requester> Provider<R> for HafasClient<R> {
    type Error = crate::Error;

    async fn locations(
        &self,
        opts: LocationsOptions,
    ) -> Result<LocationsResponse, rcore::Error<R::Error, Self::Error>> {
        let data: HafasLocationsResponse = self
            .request(json!({
                "svcReqL": [
                    {
                        "cfg": {
                            "polyEnc": "GPA"
                        },
                        "meth": "LocMatch",
                        "req": {
                            "input": {
                                "loc": {
                                    "type": "ALL",
                                    "name": format!("{}?", opts.query),
                                },
                                "maxLoc": opts.results,
                                "field": "S"
                            }
                        }
                    }
                ],
                "lang": opts.language.as_deref().unwrap_or_else(|| self.profile.language()),
            }))
            .await?;

        Ok(self
            .profile
            .parse_locations_response(data)
            .map_err(|e| rcore::Error::Provider(e.into()))?)
    }

    async fn journeys(
        &self,
        from: Place,
        to: Place,
        opts: JourneysOptions,
    ) -> Result<JourneysResponse, rcore::Error<R::Error, Self::Error>> {
        let timezone = self.profile.timezone();
        let (when, is_departure) = match (opts.departure, opts.arrival) {
            (Some(_), Some(_)) => Err(rcore::Error::Provider(Self::Error::InvalidInput(
                "departure and arrival are mutually exclusive".to_string(),
            )))?,
            (Some(departure), None) => (departure.with_timezone(&timezone), true),
            (None, Some(arrival)) => (arrival.with_timezone(&timezone), false),
            (None, None) => (Utc::now().with_timezone(&timezone), true),
        };

        let tariff_class = opts.tariff_class;

        let mut req = json!({
            "svcReqL": [
                {
                    "cfg": {
                        "polyEnc": "GPA"
                    },
                    "meth": "TripSearch",
                    "req": {
                        "ctxScr": null,
                        "getPasslist": opts.stopovers,
                        "maxChg": match opts.transfers {
                            TransferOptions::Unlimited => -1,
                            TransferOptions::Limited(i) => i as i64,
                        },
                        "minChgTime": opts.transfer_time.num_minutes(),
                        "numF": opts.results,
                        "depLocL": [ from.to_hafas() ],
                        "viaLocL": json!(opts.via.into_iter().map(|x| json!({ "loc": x.to_hafas() })).collect::<Vec<_>>()),
                        "arrLocL": [ to.to_hafas() ],
                        "jnyFltrL": [
                            {
                                "type": "PROD",
                                "mode": "INC",
                                "value": self.profile.products_to_hafas(&opts.products),
                            },
                            {
                                "type": "META",
                                "mode": "INC",
                                "meta": opts.accessibility.to_hafas(),
                            }
                        ],
                        "gisFltrL": [],
                        "getTariff": opts.tickets,
                        "ushrp": opts.start_with_walking,
                        "getPT": true,
                        "getIV": false,
                        "outFrwd": is_departure,
                        "outDate": when.format("%Y%m%d").to_string(),
                        "outTime": when.format("%H%M%S").to_string(),
                        "trfReq": {
                            "jnyCl": tariff_class.to_hafas(),
                            "tvlrProf": [
                                {
                                    "type": opts.passenger_age.map(|a| self.profile.age_to_hafas(a)).unwrap_or("E"),
                                    "redtnCard": opts.loyalty_card.map(LoyaltyCard::to_id),
                                }
                            ],
                            "cType": "PK"
                        }
                    }
                }
            ],
            "lang": opts.language.as_deref().unwrap_or("en"),
        });
        #[cfg(feature = "polylines")]
        {
            req["svcReqL"][0]["req"]["getPolyline"] = json!(opts.polylines);
        }
        #[cfg(not(feature = "polylines"))]
        {
            req["svcReqL"][0]["req"]["getPolyline"] = json!(false);
        }
        if let Some(r) = opts.later_than.or(opts.earlier_than) {
            req["svcReqL"][0]["req"]["ctxScr"] = json!(r);
        }
        if opts.bike_friendly {
            req["svcReqL"][0]["req"]["jnyFltrL"]
                .as_array_mut()
                .unwrap()
                .push(json!({"type": "BC", "mode": "INC"}))
        }
        let data = self.request(req).await?;

        Ok(self
            .profile
            .parse_journeys_response(data, tariff_class)
            .map_err(|e| rcore::Error::Provider(e.into()))?)
    }

    async fn refresh_journey(
        &self,
        journey: &Journey,
        opts: RefreshJourneyOptions,
    ) -> Result<RefreshJourneyResponse, rcore::Error<R::Error, Self::Error>> {
        let refresh_token = &journey.id;
        let tariff_class = opts.tariff_class;

        let mut req = json!({
            "svcReqL": [
                {
                    "cfg": {},
                    "meth": "Reconstruction",
                    "req": {
                        "getIST": true,
                        "getPasslist": opts.stopovers,
                        "getTariff": opts.tickets,
                    }
                }
            ],
            "lang": opts.language.as_deref().unwrap_or("en"),
        });
        if self.profile.refresh_journey_use_out_recon_l() {
            req["svcReqL"][0]["req"]["outReconL"] = json!([{ "ctx": refresh_token }]);
        } else {
            req["svcReqL"][0]["req"]["ctxRecon"] = json!(refresh_token);
        }
        #[cfg(feature = "polylines")]
        {
            req["svcReqL"][0]["req"]["getPolyline"] = json!(opts.polylines);
        }
        #[cfg(not(feature = "polylines"))]
        {
            req["svcReqL"][0]["req"]["getPolyline"] = json!(false);
        }
        let data: HafasJourneysResponse = self.request(req).await?;

        let mut journeys = self
            .profile
            .parse_journeys_response(data, tariff_class)
            .map_err(|e| rcore::Error::Provider(e.into()))?;
        Ok(journeys.journeys.remove(0))
    }
}
