use std::{collections::HashMap, fmt::{Display, Formatter, Result}};

use chrono::{DateTime, Datelike, FixedOffset, TimeDelta, TimeZone, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

use crate::{colors::colored, input, integer_input, options::Options, resources::{CustomWeekday, CustomWeekdayVec}};

// todo: implement custom logic for schedules...
#[derive(Serialize, Deserialize)]
pub struct Schedule {
    pub description: String,
    pub weekdays: CustomWeekdayVec,
    pub index: usize,
    notification_time: u32,
    hour: u32,
    minute: u32
}

impl Display for Schedule {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        writeln!(_f, "{}", self.get_as_text())
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            description: String::from(""),
            index: 0,
            weekdays: CustomWeekdayVec { days: Vec::new() },
            notification_time: 10,
            hour: 0,
            minute: 0
        }
    }
}

impl Schedule {
    pub fn get_as_text(&self) -> String {
        let task_text = format!(
            "{} - ({})",
            self.description,
            self.weekdays.get_as_text()
        );
        task_text.to_string()
    }

    pub fn new() -> Self {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);

        println!("Describe your schedule:");
        let description: String = input!();
        println!("Select the hour (24 hour format):");
        let hour: u32 = integer_input!(now.hour().to_string()) as u32;
        println!("Select the minute:");
        let minute: u32 = integer_input!(now.minute().to_string()) as u32;
        println!("How many minutes before do you want to be notified?");
        let notification_time: u32 = integer_input!(10) as u32;
        
        let weekday_options_list = Vec::from([
            String::from("Sunday"),
            String::from("Monday"),
            String::from("Tuesday"),
            String::from("Wednesday"),
            String::from("Thursday"),
            String::from("Friday"),
            String::from("Saturday"),
        ]);

        let mut opt = Options::default();
        opt.build(weekday_options_list);

        let weekdays_map = opt.print_radio_option_unmarked("Select the day(s) to be notified", false);
        let mut weekdays = CustomWeekdayVec::default();
        for (_, v) in weekdays_map.iter() {
            let day = CustomWeekday::from(v as &str);
            weekdays.add_day(day);
        }

        Schedule { description, weekdays, index: 0, notification_time, hour, minute }

    }

    pub fn update(&mut self) {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);

        println!("Describe your schedule:");
        self.description = input!();
        println!("Select the hour (24 hour format):");
        self.hour = integer_input!(now.hour().to_string()) as u32;
        println!("Select the minute:");
        self.minute = integer_input!(now.minute().to_string()) as u32;
        println!("How many minutes before do you want to be notified?");
        self.notification_time = integer_input!(10) as u32;
        
        let weekday_options_list = Vec::from([
            String::from("Sunday"),
            String::from("Monday"),
            String::from("Tuesday"),
            String::from("Wednesday"),
            String::from("Thursday"),
            String::from("Friday"),
            String::from("Saturday"),
        ]);

        let mut opt = Options::default();
        opt.build(weekday_options_list);

        let mut selected_map = HashMap::new();

        for day in self.weekdays.days.iter() {
            selected_map.insert(day.value() as usize, String::from(day.name()));
        }

        let weekdays_map = opt.print_radio_option(
            "Select the day(s) to be notified", false, selected_map
        );

        let mut weekdays = CustomWeekdayVec::default();
        for (_, v) in weekdays_map.iter() {
            let day = CustomWeekday::from(v as &str);
            weekdays.add_day(day);
        }
        self.weekdays = weekdays;
    }

    pub fn is_due(&self) -> bool {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);
        if !((now.time().hour() as i64 - self.hour as i64) < 0) {
            if !((now.time().minute() as i64 - self.minute as i64) < self.notification_time as i64) {
                return false;
            }
        }
        true
    }
}
