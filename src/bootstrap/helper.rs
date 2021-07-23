use chrono::{NaiveDateTime, TimeZone, Local};

pub trait NaiveDatetimeLocal{
    fn serialize_local(&self)->String;
}

impl NaiveDatetimeLocal for NaiveDateTime{
    fn serialize_local(&self) -> String {
        let local_string = TimeZone::from_utc_datetime(&Local,&self).naive_local().to_string();
        local_string
        //NaiveDateTime::parse_from_str(local.as_str(), "")?
    }
}
