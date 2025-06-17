use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Datelike, FixedOffset, TimeDelta, TimeZone, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

use crate::{colors::colored, input, integer_input, options::Options, resources::{CustomWeekday, CustomWeekdayVec}};

// todo: implement custom logic for schedules...
#[derive(Serialize, Deserialize)]
pub struct Schedule {
    pub description: String,
    pub weekdays: CustomWeekdayVec,
    pub index: usize,
    notification_time: i64,
    pub completed: bool,
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
            completed: false,
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
        if !self.completed {
            task_text.to_string()
        } else {
            colored(&task_text, "green").to_string()
        }
    }

    pub fn new() -> Self {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);

        println!("Describe your schedule:");
        let description: String = input!();
        println!("Select the hour:");
        let hour: u32 = integer_input!(now.hour().to_string()) as u32;
        println!("Select the minute:");
        let min: u32 = integer_input!(now.minute().to_string()) as u32;
        println!("How many minutes before do you want to be notified?");
        let notification_time: i64 = integer_input!(10) as i64;
        
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

        opt.print_radio_option("Select the day(s) to be notified", false);

        // if let Some(due_date) = Utc
        //     .with_ymd_and_hms(year, month, day, hour, min, 0)
        //     .earliest()
        // {
        //     let offset = FixedOffset::east_opt(3 * 3600).expect("");
        //     let due_date = due_date.with_timezone(&offset);
        //     Schedule {
        //         index: 0,
        //         due_date,
        //         description,
        //         completed: false,
        //         notification_time,
        //     }
        // } else {
        //     Schedule::default()
        // }
        Schedule::default()
    }

    // pub fn update(&mut self) {
    //     println!("Leave it empty if you don't want to edit.");
    //     println!("Edit the description:");
    //     let description: String = input!(self.description);
    //     println!("Select the new day:");
    //     let day = integer_input!(self.due_date.day().to_string()) as u32;
    //     println!("Select the new month:");
    //     let month = integer_input!(self.due_date.month().to_string()) as u32;
    //     println!("Select the new year:");
    //     let year = integer_input!(self.due_date.year().to_string());
    //     println!("Select the new hour:");
    //     let hour = integer_input!(self.due_date.hour().to_string()) as u32;
    //     println!("Select the new minute:");
    //     let min = integer_input!(self.due_date.minute().to_string()) as u32;
    //     println!("Select the new notification time:");
    //     let notif_time = integer_input!(self.notification_time) as i64;
    //
    //     if let Some(date) = Utc
    //         .with_ymd_and_hms(year, month, day, hour, min, 0)
    //         .earliest()
    //     {
    //         let offset = FixedOffset::east_opt(3 * 3600).expect("");
    //         let date = date.with_timezone(&offset);
    //         self.description = description.to_string();
    //         self.due_date = date;
    //         self.notification_time = notif_time;
    //     } else {
    //         println!("{}", colored("Error!", "red"))
    //     }
    // }

    // pub fn is_due(&self) -> bool {
    //     let offset = FixedOffset::west_opt(3 * 3600).expect("");
    //     let now = Utc::now().with_timezone(&offset);
    //     if let Some(td) = TimeDelta::new(60 * self.notification_time, 0) {
    //         self.due_date - now <= td
    //     } else {
    //         false
    //     }
    // }
}
