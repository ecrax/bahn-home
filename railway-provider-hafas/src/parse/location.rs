use crate::Profile;
use crate::{Location, ParseResult, Place, Station};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasCoords {
    x: i64,
    y: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasPlace {
    r#type: Option<String>,
    name: String,
    crd: HafasCoords,
    ext_id: Option<String>,
    p_cls: Option<u16>,
}

pub(crate) fn default_parse_coords(coords: HafasCoords) -> (f32, f32) {
    (coords.x as f32 / 1000000.0, coords.y as f32 / 1000000.0)
}

pub(crate) fn default_parse_place<P: Profile + ?Sized>(
    profile: &P,
    data: HafasPlace,
) -> ParseResult<Place> {
    let HafasPlace {
        r#type,
        name,
        crd,
        ext_id,
        p_cls,
    } = data;
    let coords = profile.parse_coords(crd);
    match r#type.as_deref() {
        Some("S") => {
            let id = ext_id.ok_or("Missing ext_id")?;
            Ok(Place::Station(Station {
                id: id.clone(),
                name: Some(name),
                products: p_cls
                    .map(|p_cls| profile.parse_products(p_cls).into_iter().cloned().collect())
                    .unwrap_or_default(),
                location: Some(Location::Point {
                    id: Some(id),
                    name: None,
                    latitude: coords.0,
                    longitude: coords.1,
                    poi: None,
                }),
            }))
        }
        Some("P") => Ok(Place::Location(Location::Point {
            id: ext_id,
            name: Some(name),
            latitude: coords.0,
            longitude: coords.1,
            poi: Some(true),
        })),
        Some("A") => Ok(Place::Location(Location::Address {
            address: name,
            latitude: coords.0,
            longitude: coords.1,
        })),
        other => Err(format!("Unknown location type: {:?}", other).into()),
    }
}
