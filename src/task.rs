use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    data::write_tasks,
    colors::colored
};

// Code from serde documentation
mod my_date_format {
    use chrono::{DateTime, FixedOffset, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<FixedOffset>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
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
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(dt, offset))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskVec {
    pub tasks: Vec<Task>
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

    pub fn get_as_text(&self, idx: usize) -> String {
        if let Some(t) = self.tasks.get(idx) {
            let task = format!("{} - ({})", t.description, t.due_date.to_string());
            if !t.completed {
                format!("{task}")
            } else {
                format!("{}", colored(&task, "green"))
            }
        } else {
            String::from("There was an error reading the task!")
        }
    }

    pub fn get(&mut self, idx: usize) -> Option<&mut Task> {
        self.tasks.get_mut(idx)
    }

    pub fn save(&self) {
        write_tasks(&self);
    }

    pub fn complete(&mut self, idx: usize) {
        if let Some(t) = &mut self.tasks.get_mut(idx) {
            t.completed = true;
            write_tasks(&self);
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
    pub completed: bool
}

impl Display for Task {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result { 
        println!("{}", self.get_as_text());
        Ok(())
    }
}

impl Task {
    pub fn get_as_text(&self) -> String {

        let task_text = format!("{} - ({})", self.description, self.due_date.to_string());
        if !self.completed {
            format!("{task_text}")
        } else {
            format!("{}", colored(&task_text, "green"))
        }
    }
    pub fn new(description: &str, due_date: DateTime<FixedOffset>) -> Self {
        Task {
            index: 0,
            due_date: due_date,
            description: String::from(description),
            completed: false
        }
    }

    pub fn update(&mut self, description: &str, due_date: DateTime<FixedOffset>) {
            self.description = description.to_string();
            self.due_date = due_date;
    }

    pub fn is_due(&self) -> bool {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        let now = Utc::now().with_timezone(&offset);
        if let Some(td) = TimeDelta::new(120, 0) {
            now - self.due_date >= td
        } else { false }
    }
}
