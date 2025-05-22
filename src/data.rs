use std::fs::{read_to_string, write};

use chrono::{DateTime, FixedOffset};
use serde_json::{
    to_string_pretty,
    from_str,
    Result,
};

use crate::{
    colors::colored,
    task::{
        Task,
        TaskVec,
    }
};


pub fn write_tasks(tasks: &TaskVec) {
    let ser = to_string_pretty(tasks);
    match ser {
        Ok(ser) => _ = write("tasks.json", ser),
        Err(_) => println!("{}", colored("Erro!", "red"))
    }
}

pub fn get_tasks() -> Result<TaskVec> {
    let tasks = read_to_string("tasks.json").expect("Couldn't find or load that file.");
    let tasks: TaskVec = from_str(&tasks)?;
    Ok(tasks)
}

pub fn add_task(description: &str, due_date: DateTime<FixedOffset>) -> Result<()> {
    let result = get_tasks();
    match result {
        Ok(mut result) => {
            result.push(Task::new(description, due_date));
            write_tasks(&result);
        },
        Err(_) => println!("{}", colored("Erro!", "red"))
    }

    Ok(())
}
