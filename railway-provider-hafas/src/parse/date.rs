use crate::ParseError;
use crate::ParseResult;
use crate::Profile;
use chrono::DateTime;
use chrono::Duration;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono_tz::OffsetComponents;

pub(crate) fn default_parse_date<P: Profile + ?Sized>(
    profile: &P,
    time: Option<String>,
    tz_offset: Option<i32>,
    date: &NaiveDate,
) -> ParseResult<Option<DateTime<FixedOffset>>> {
    let time = match time {
        Some(time) => time,
        None => return Ok(None),
    };

    let (dayoffset, time) = match time.len() {
        8 => {
            let iter = time.chars();
            let dayoffset = iter.clone().take(2).collect::<String>().parse()?;
            (dayoffset, iter.skip(2).collect::<String>())
        }
        6 => (0, time),
        len => return Err(format!("invalid time length. expected 6 or 8, got {}", len).into()),
    };

    let time = NaiveTime::parse_from_str(&time, "%H%M%S")?;
    let naive_dt = date.and_time(time) + Duration::days(dayoffset);

    let timezone = match tz_offset {
        Some(min) => FixedOffset::east_opt(min * 60),
        None => {
            match profile
                .timezone()
                .offset_from_local_datetime(&naive_dt)
                .single()
            {
                Some(off) => FixedOffset::east_opt(
                    (off.base_utc_offset() + off.dst_offset()).num_seconds() as i32,
                ),
                None => {
                    return Err("timestamp is missing offset and it can not be filled in".into())
                }
            }
        }
    };
    let dt = timezone
        .ok_or(ParseError::from("timezone offset too large"))?
        .from_local_datetime(&naive_dt)
        .unwrap(); // This will never panic for FixedOffset timezone
    Ok(Some(dt))
}
