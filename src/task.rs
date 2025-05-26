use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Datelike, FixedOffset, TimeDelta, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};

use crate::{colors::colored, data::write_tasks, input, integer_input};

// Code from serde documentation
mod my_date_format {
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
pub struct TaskVec {
    pub tasks: Vec<Task>,
}

impl TaskVec {
    pub fn push(&mut self, mut task: Task) {
        let index = self.len();
        task.index = index;
        self.tasks.insert(index, task);
    }

    pub fn print_tasks(&self) {
        for task in self.tasks.iter() {
            println!("{task}")
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    pub fn get_as_text(&self, idx: usize) -> String {
        if let Some(t) = self.tasks.get(idx) {
            let task = format!("{} - ({})", t.description, t.due_date);
            if !t.completed {
                task.to_string()
            } else {
                colored(&task, "green").to_string()
            }
        } else {
            String::from("There was an error reading the task!")
        }
    }

    pub fn remove(&mut self, idx: usize) {
        self.tasks.remove(idx);
        self.reindex();
        self.save();
    }

    pub fn get(&mut self, idx: usize) -> Option<&mut Task> {
        self.tasks.get_mut(idx)
    }

    pub fn reindex(&mut self) {
        for (counter, task) in self.tasks.iter_mut().enumerate() {
            task.index = counter;
        }
    }

    pub fn save(&self) {
        write_tasks(self);
    }

    pub fn change_status(&mut self, idx: usize) {
        if let Some(t) = &mut self.tasks.get_mut(idx) {
            t.completed = !t.completed;
            write_tasks(self);
        } else {
            println!("{}", colored("There was an error reading the task!", "red"));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    description: String,
    #[serde(with = "my_date_format")]
    due_date: DateTime<FixedOffset>,
    index: usize,
    notification_time: i64,
    pub completed: bool,
}

impl Display for Task {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        writeln!(_f, "{}", self.get_as_text())
    }
}

impl Task {
    pub fn get_as_text(&self) -> String {
        let task_text = format!(
            "{} - ({})",
            self.description,
            self.due_date.format("%d/%m/%Y %H:%M")
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

        println!("Describe your task:");
        let description: String = input!();
        println!("Select the day:");
        let day: u32 = integer_input!(now.day().to_string()) as u32;
        println!("Select the month:");
        let month: u32 = integer_input!(now.month().to_string()) as u32;
        println!("Select the year:");
        let year: i32 = integer_input!(now.year().to_string());
        println!("Select the hour:");
        let hour: u32 = integer_input!(now.hour().to_string()) as u32;
        println!("Select the minute:");
        let min: u32 = integer_input!(now.minute().to_string()) as u32;
        println!("How many minutes before do you want to be notified?");
        let notification_time: i64 = integer_input!(10) as i64;

        let due_date = Utc
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .earliest().expect("An unexpected error has occured!");
        let offset = FixedOffset::east_opt(3 * 3600).expect("");
        let due_date = due_date.with_timezone(&offset);
        Task {
            index: 0,
            due_date,
            description: String::from(description),
            completed: false,
            notification_time,
        }
    }

    pub fn update(&mut self) {
        println!("Leave it empty if you don't want to edit.");
        println!("Edit the description:");
        let description: String = input!(self.description);
        println!("Select the new day:");
        let day = integer_input!(self.due_date.day().to_string()) as u32;
        println!("Select the new month:");
        let month = integer_input!(self.due_date.month().to_string()) as u32;
        println!("Select the new year:");
        let year = integer_input!(self.due_date.year().to_string());
        println!("Select the new hour:");
        let hour = integer_input!(self.due_date.hour().to_string()) as u32;
        println!("Select the new minute:");
        let min = integer_input!(self.due_date.minute().to_string()) as u32;
        println!("Select the new notification time:");
        let notif_time = integer_input!(self.notification_time) as i64;

        if let Some(date) = Utc
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .earliest()
        {
            let offset = FixedOffset::east_opt(3 * 3600).expect("");
            let date = date.with_timezone(&offset);
            self.description = description.to_string();
            self.due_date = date;
            self.notification_time = notif_time;
        } else {
            println!("{}", colored("Error!", "red"))
        }
    }

    pub fn is_due(&self) -> bool {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);
        if let Some(td) = TimeDelta::new(60 * self.notification_time, 0) {
            self.due_date - now <= td
        } else {
            false
        }
    }
}
