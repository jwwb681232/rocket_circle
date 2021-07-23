use chrono::{NaiveDateTime, TimeZone, Local};

pub trait NaiveDatetimeLocal{
    fn serialize_local(&self)->String;
}

impl NaiveDatetimeLocal for NaiveDateTime{
    fn serialize_local(&self) -> NaiveDateTime {
        let local = TimeZone::from_utc_datetime(&Local,&self).naive_local().to_string();
        NaiveDateTime::parse_from_str(local.as_str(), "")?
    }
}
