use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Code from serde documentation
mod my_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
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
        date: &DateTime<Utc>,
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
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskVec {
    tasks: Vec<Task>
}

impl TaskVec {
    pub fn push(&mut self, mut task: Task) {
        let index = self.len();
        task.index = index;
        self.tasks.insert(index, task);
    }

    pub fn print_tasks(self) {
        for task in self.tasks.iter() {
            println!("{task}")
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn get(&self, idx: usize) -> String {
        if let Some(t) = self.tasks.get(idx) {
            format!("{} - ({})", t.description, t.due_date.to_string())
        } else {
            String::from("There was an error reading the task!")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    description: String,
    #[serde(with = "my_date_format")]
    due_date: DateTime<Utc>,
    index: usize,
}

impl Display for Task {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result { 
        println!("Task:\n{}\nDue date:\n{}", self.description, self.due_date.to_string());
        Ok(())
    }
}

impl Task {
    pub fn new(description: &str, due_date: DateTime<Utc>) -> Self {
        Task {
            index: 0,
            due_date: due_date,
            description: String::from(description)
        }
    }
}
