pub(crate) mod duration {
    use chrono::Duration;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S>(v: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(d) = v {
            s.serialize_some(&d.num_minutes())
        } else {
            s.serialize_none()
        }
    }

    pub(crate) fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::<i64>::deserialize(d)?;
        Ok(v.map(Duration::minutes))
    }
}
