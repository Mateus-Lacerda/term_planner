use std::process::Command;

use crate::data::get_resources;

pub fn send_notify(message: &str, error: bool) {
    let urgency = if !error { "1" } else { "2" };

    let res = Command::new("dunstify")
        .arg(message)
        .arg("-u")
        .arg(urgency)
        .output();

    match res {
        Ok(res) => {
            println!("OK! {}", String::from_utf8_lossy(&res.stderr));
        }
        Err(_) => println!("Error sending notification!"),
    }
}

pub fn run_notification_service() {
    let resources = get_resources();
    match resources {
        Ok(t) => {
            let tasks_iter = t
                .tasks
                .iter()
                .filter(|task| !task.completed)
                .filter(|task| task.is_due());

            for task in tasks_iter {
                let msg = format!("Task is due: {}", task.get_as_text());
                println!("{msg}");
                send_notify(&msg, false);
            }

            let schedules_iter = t.schedules.iter().filter(|schedule| schedule.is_due());

            for schedule in schedules_iter {
                let msg = format!("Schedule is due: {}", schedule.get_as_text());
                println!("{msg}");
                send_notify(&msg, false);
            }
        }
        Err(_) => send_notify("Error loading tasks!", true),
    }
}
