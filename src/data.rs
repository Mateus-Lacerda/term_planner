use std::fs::{read_to_string, write};
use std::path::PathBuf;

use dirs::config_dir;
use serde_json::{Result, from_str, to_string_pretty};

use crate::{
    colors::colored,
    resources::{Resources, Schedule, Task},
};

fn resources_file_path() -> PathBuf {
    let mut dir = config_dir().expect("Não conseguiu descobrir XDG config dir");
    dir.push("term_planner");
    std::fs::create_dir_all(&dir).expect("Falha ao criar config dir");
    dir.push("tasks.json");
    dir
}

pub fn write_resources(resources: &Resources) {
    let ser = to_string_pretty(resources);
    match ser {
        Ok(ser) => _ = write(resources_file_path(), ser),
        Err(_) => println!("{}", colored("Erro!", "red")),
    }
}

pub fn get_resources() -> Result<Resources> {
    let path = resources_file_path();
    if !path.exists() {
        write(&path, "{\"tasks\": [], \"schedules\": []}")
            .expect("Falha ao criar tasks.json inicial");
    }
    let resources = read_to_string(path).expect("Couldn't find or load that file.");
    let resources: Resources = from_str(&resources)?;
    Ok(resources)
}

pub fn add_task() -> Result<()> {
    let result = get_resources();
    match result {
        Ok(mut result) => {
            result.push_task(Task::new());
            write_resources(&result);
        }
        Err(_) => println!("{}", colored("Erro!", "red")),
    }

    Ok(())
}

pub fn add_schedule() -> Result<()> {
    let result = get_resources();
    match result {
        Ok(mut result) => {
            result.push_schedule(Schedule::new());
            write_resources(&result);
        }
        Err(_) => println!("{}", colored("Erro!", "red")),
    }

    Ok(())
}
