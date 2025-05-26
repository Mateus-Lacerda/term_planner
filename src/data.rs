use std::fs::{read_to_string, write};
use std::path::PathBuf;

use dirs::config_dir;
use serde_json::{Result, from_str, to_string_pretty};

use crate::{
    colors::colored,
    task::{Task, TaskVec},
};

fn tasks_file_path() -> PathBuf {
    let mut dir = config_dir().expect("Não conseguiu descobrir XDG config dir");
    dir.push("term_planner");
    std::fs::create_dir_all(&dir).expect("Falha ao criar config dir");
    dir.push("tasks.json");
    dir
}

pub fn write_tasks(tasks: &TaskVec) {
    let ser = to_string_pretty(tasks);
    match ser {
        Ok(ser) => _ = write(tasks_file_path(), ser),
        Err(_) => println!("{}", colored("Erro!", "red")),
    }
}

pub fn get_tasks() -> Result<TaskVec> {
    let path = tasks_file_path();
    if !path.exists() {
        write(&path, "{\"tasks\": []}").expect("Falha ao criar tasks.json inicial");
    }
    let tasks = read_to_string(path).expect("Couldn't find or load that file.");
    let tasks: TaskVec = from_str(&tasks)?;
    Ok(tasks)
}

pub fn add_task() -> Result<()> {
    let result = get_tasks();
    match result {
        Ok(mut result) => {
            result.push(Task::new());
            write_tasks(&result);
        }
        Err(_) => println!("{}", colored("Erro!", "red")),
    }

    Ok(())
}
