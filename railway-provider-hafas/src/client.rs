use super::{Error, ParseError, Profile, Requester};
use log::debug;
use md5::{Digest, Md5};
use rcore::RequesterBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct HafasClient<R: Requester> {
    pub(crate) profile: Arc<Box<dyn Profile>>,
    requester: Arc<R>,
}

impl<R: Requester> HafasClient<R> {
    pub fn new<P: 'static + Profile, RB: RequesterBuilder<Requester = R>>(
        profile: P,
        mut requester: RB,
    ) -> Self {
        if let Some(certificate) = profile.custom_pem_bundle() {
            requester = requester.with_pem_bundle(certificate);
        }
        HafasClient {
            profile: Arc::new(Box::new(profile)),
            requester: Arc::new(requester.build()),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HafasResponseInner {
    err: Option<String>,
    err_txt: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HafasResponseOuter {
    err: Option<String>,
    err_txt: Option<String>,
    svc_res_l: Option<Vec<HafasResponseInner>>,
}

#[derive(Deserialize)]
struct HafasResponseInnerOk<T> {
    res: T,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HafasResponseOuterOk<T> {
    svc_res_l: Vec<HafasResponseInnerOk<T>>,
}

impl<R: Requester> HafasClient<R> {
    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        mut req_json: Value,
    ) -> Result<T, rcore::Error<R::Error, crate::Error>> {
        self.profile.prepare_body(&mut req_json);
        debug!(
            "{}",
            serde_json::to_string_pretty(&req_json)
                .map_err(|e| rcore::Error::Provider(e.into()))?
        );
        let req_str =
            serde_json::to_string(&req_json).map_err(|e| rcore::Error::Provider(e.into()))?;
        // TODO: Error?
        let mut url = url::Url::parse(self.profile.url()).expect("Failed to parse provider URL");

        if let Some(salt) = self.profile.checksum_salt() {
            if self.profile.salt() {
                let mut hasher = Md5::new();
                hasher.update(&req_str);
                hasher.update(salt);
                let checksum = hasher
                    .finalize()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join("");
                url.query_pairs_mut().append_pair("checksum", &checksum);
            }
            if self.profile.mic_mac() {
                let mut hasher_mic = Md5::new();
                hasher_mic.update(&req_str);
                let mic = hasher_mic
                    .finalize()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join("");
                let mut hasher_mac = Md5::new();
                hasher_mac.update(&mic);
                if let Ok(s) = hex::decode(salt) {
                    hasher_mac.update(s);
                } else {
                    hasher_mac.update(salt);
                }
                let mac = hasher_mac
                    .finalize()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join("");
                url.query_pairs_mut()
                    .append_pair("mic", &mic)
                    .append_pair("mac", &mac);
            }
        }

        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        headers.insert("Accept", "application/json");
        self.profile.prepare_headers(&mut headers);

        let bytes = self
            .requester
            .post(&url, req_str.as_bytes(), headers)
            .await
            .map_err(rcore::Error::Request)?;
        debug!(
            "Response: {}",
            serde_json::to_string(
                &serde_json::from_slice::<serde_json::Value>(&bytes)
                    .map_err(|e| rcore::Error::Provider(e.into()))?
            )
            .map_err(|e| rcore::Error::Provider(e.into()))?
        );

        {
            let data =
                serde_json::from_slice(&bytes).map_err(|e| rcore::Error::Provider(e.into()))?;
            let HafasResponseOuter {
                err,
                err_txt,
                svc_res_l,
            } = data;
            if let Some(some_err) = err {
                if some_err != "OK" {
                    return Err(rcore::Error::Provider(Error::Hafas {
                        text: err_txt.unwrap_or_else(|| format!("Code {}", &some_err)),
                        code: some_err,
                    }));
                }
            }
            let HafasResponseInner { err, err_txt } = svc_res_l
                .map(|mut x| x.remove(0))
                .ok_or_else(|| ParseError::from("missing svcResL"))
                .map_err(|e| rcore::Error::Provider(e.into()))?;
            if let Some(some_err) = err {
                if some_err != "OK" {
                    // TODO: Parse to better errors.
                    return Err(rcore::Error::Provider(Error::Hafas {
                        text: err_txt.unwrap_or_else(|| format!("Code {}", &some_err)),
                        code: some_err,
                    }));
                }
            }
        }

        {
            let mut data: HafasResponseOuterOk<T> =
                serde_json::from_slice(&bytes).map_err(|e| rcore::Error::Provider(e.into()))?;
            let HafasResponseInnerOk { res } = data.svc_res_l.remove(0);
            Ok(res)
        }
    }
}
