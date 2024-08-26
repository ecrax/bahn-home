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

pub(crate) mod datetime_with_timezone {
    use chrono::{DateTime, NaiveDateTime};
    use chrono_tz::Tz;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S>(v: &Option<DateTime<Tz>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(d) = v {
            let tz = d.timezone().to_string();
            let naive = d.naive_utc();
            s.serialize_some(&(naive, tz))
        } else {
            s.serialize_none()
        }
    }

    pub(crate) fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<Tz>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Option::<(NaiveDateTime, String)>::deserialize(d)?;
        if let Some((d, tz)) = v {
            let tz = tz
                .parse()
                .map_err(|_| serde::de::Error::custom("Failed to parse timezone"))?;
            Ok(Some(d.and_utc().with_timezone(&tz)))
        } else {
            Ok(None)
        }
    }
}
