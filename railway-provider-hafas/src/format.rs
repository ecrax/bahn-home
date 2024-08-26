use super::{Accessibility, Location, Place, Station, TariffClass};
use serde_json::json;

pub trait ToHafas<T> {
    fn to_hafas(&self) -> T;
}

fn format_coord(coordinate: f32) -> u64 {
    (coordinate * 1000000.0) as u64
}

fn format_identifier(components: Vec<(&str, &str)>) -> String {
    components
        .iter()
        .map(|(k, v)| format!("{}={}@", k, v))
        .collect::<Vec<_>>()
        .join("")
}

impl ToHafas<serde_json::Value> for Place {
    fn to_hafas(&self) -> serde_json::Value {
        match self {
            Place::Station(stop) => {
                let Station { id, .. } = stop;
                json!({
                    "type": "S",
                    "lid": format_identifier(vec![
                        ("A", "1"),
                        ("L", id),
                    ])
                })
            }
            Place::Location(location) => match location {
                Location::Address {
                    address,
                    latitude,
                    longitude,
                } => json!({
                    "type": "A",
                    "lid": format_identifier(vec![
                        ("A", "2"),
                        ("O", address),
                        ("X", &format_coord(*latitude).to_string()),
                        ("Y", &format_coord(*longitude).to_string()),
                    ])
                }),
                Location::Point {
                    id,
                    latitude,
                    longitude,
                    ..
                } => {
                    let x = format_coord(*latitude).to_string();
                    let y = format_coord(*longitude).to_string();
                    let mut lid = vec![("A", "4"), ("X", &x), ("Y", &y)];
                    if let Some(id) = id {
                        lid.push(("L", id));
                    }
                    json!({
                        "type": "P",
                        "lid": format_identifier(lid)
                    })
                }
            },
        }
    }
}

impl ToHafas<String> for Accessibility {
    fn to_hafas(&self) -> String {
        match self {
            Accessibility::r#None => "notBarrierfree",
            Accessibility::Partial => "limitedBarrierfree",
            Accessibility::Complete => "completeBarrierfree",
        }
        .to_string()
    }
}

impl ToHafas<u64> for TariffClass {
    fn to_hafas(&self) -> u64 {
        match *self {
            TariffClass::First => 1,
            TariffClass::Second => 2,
        }
    }
}
