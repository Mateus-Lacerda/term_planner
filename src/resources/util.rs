use serde::{Deserialize, Serialize};

// Code from serde documentation
pub mod my_date_format {
    use chrono::{DateTime, FixedOffset, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
            dt, offset,
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub enum CustomWeekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

impl CustomWeekday {
    pub fn value(&self) -> i32 {
        match *self {
            CustomWeekday::Sunday => 0,
            CustomWeekday::Monday => 1,
            CustomWeekday::Tuesday => 2,
            CustomWeekday::Wednesday => 3,
            CustomWeekday::Thursday => 4,
            CustomWeekday::Friday => 5,
            CustomWeekday::Saturday => 6,
        }
    }
    pub fn name(&self) -> &str {
        match *self {
            CustomWeekday::Sunday => "Sunday",
            CustomWeekday::Monday => "Monday",
            CustomWeekday::Tuesday => "Tuesday",
            CustomWeekday::Wednesday => "Wednesday",
            CustomWeekday::Thursday => "Thursday",
            CustomWeekday::Friday => "Friday",
            CustomWeekday::Saturday => "Saturday",
        }
    }
    fn id(&self) -> &str {
        match *self {
            CustomWeekday::Sunday => "SU",
            CustomWeekday::Monday => "MO",
            CustomWeekday::Tuesday => "TU",
            CustomWeekday::Wednesday => "WE",
            CustomWeekday::Thursday => "TH",
            CustomWeekday::Friday => "FR",
            CustomWeekday::Saturday => "SA",
        }
    }
}

impl From<&str> for CustomWeekday {
    fn from(value: &str) -> Self {
        match value {
            "Sunday" => CustomWeekday::Sunday,
            "Monday" => CustomWeekday::Monday,
            "Tuesday" => CustomWeekday::Tuesday,
            "Wednesday" => CustomWeekday::Wednesday,
            "Thursday" => CustomWeekday::Thursday,
            "Friday" => CustomWeekday::Friday,
            "Saturday" => CustomWeekday::Saturday,
            _ => CustomWeekday::Sunday,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomWeekdayVec {
    pub days: Vec<CustomWeekday>
}

impl CustomWeekdayVec {
    pub fn get_as_text(&self) -> String {
        let mut text = String::new();
        for day in self.days.iter() {
            if text.is_empty() {
                text = String::from(day.id());
            } else {
                text = format!(
                    "{}, {}",
                    text,
                    String::from(day.id())
                );
            }
        }
        text
    }
    pub fn add_day(&mut self, day: CustomWeekday) {
        if self.days.iter().filter(|x| x.id() == day.id()).next().is_none() {
            self.days.push(day);
        }
        self.days.sort_by_key(|x| x.value())
    }
    pub fn remove_day(&mut self, day: CustomWeekday) {
        for (idx, present_day) in self.days.iter().enumerate() {
            if day.id() == present_day.id() {
                self.days.remove(idx);
                break;
            }
        }
    }
}

impl Default for CustomWeekdayVec {
    fn default() -> Self {
        CustomWeekdayVec { days: Vec::new() }
    }
}
