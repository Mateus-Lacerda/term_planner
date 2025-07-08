use std::vec::Vec;

use crate::{
    colors::colored,
    data::{add_schedule, add_task, get_resources},
    input,
    io_utils::get_kb_input,
    options::Options,
};

enum Resource {
    TaskResource,
    ScheduleResource,
}

fn add_from_input(resource: Resource) {
    match resource {
        Resource::TaskResource => {
            let res = add_task();
            match res {
                Ok(_) => println!("{}", colored("Task added!", "green")),
                Err(_) => println!("{}", colored("Error adding task!", "red")),
            }
        }
        Resource::ScheduleResource => {
            let res = add_schedule();
            match res {
                Ok(_) => println!("{}", colored("Schedule added!", "green")),
                Err(_) => println!("{}", colored("Error adding schedule!", "red")),
            }
        }
    }
}

fn schedule_menu(text: &str, selected_schedule: i8) {
    let mut options = Options::default();
    let opt_lst = Vec::from([String::from("Edit."), String::from("Delete.")]);

    options.build(opt_lst);

    let selected = options.print_option(text);
    match options.last_move {
        4 => match selected {
            1 => {
                options.print_ui_and_text(text);
                let result = get_resources();
                match result {
                    Ok(mut res) => {
                        if let Some(s) = res.get_schedule(selected_schedule as usize - 1) {
                            s.update();
                            res.save();
                            println!("{}", colored("Schedule updated!", "green"));
                            println!("Press any key to continue...");
                            _ = get_kb_input();
                        } else {
                            println!("{}", colored("Error getting schedule!", "red"))
                        }
                    }
                    Err(_) => println!("{}", colored("Error getting resources!", "red")),
                }
            }
            2 => {
                options.print_ui_and_text(text);
                let result = get_resources();
                match result {
                    Ok(mut res) => {
                        println!(
                            "{}",
                            colored("Are you sure you want to delete the schedule? (y|n)", "red")
                        );
                        let opt = input!();
                        if opt.to_lowercase().starts_with("y") {
                            res.remove_schedule(selected_schedule as usize - 1);
                            println!("{}", colored("Success!", "green"));
                        } else {
                            println!("{}", colored("Operation canceled!", "green"));
                        }
                        println!("Press any key to continue...");
                        _ = get_kb_input();
                    }
                    Err(_) => println!("{}", colored("Error getting resources!", "red")),
                }
            }
            _ => show_schedules(),
        },
        _ => show_schedules(),
    }
    main_menu()
}

fn task_menu(text: &str, selected_task: i8) {
    let result = get_resources();
    let d_or_ud = match result {
        Ok(mut res) => {
            let comp = res
                .get_task(selected_task as usize - 1)
                .expect("")
                .completed;
            if comp { "undone" } else { "done" }
        }
        Err(_) => "",
    };

    if d_or_ud.is_empty() {
        println!("{}", colored("Error defining if task is done!", "red"));
        println!("Press any key to continue...");
        _ = get_kb_input();
        main_menu();
    }

    let d_or_ud = format!("Mark as {}.", &d_or_ud);

    let mut options = Options::default();
    let opt_lst = Vec::from([d_or_ud, String::from("Edit."), String::from("Delete.")]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    match options.last_move {
        4 => match selected {
            1 => {
                let result = get_resources();
                match result {
                    Ok(mut res) => {
                        res.change_task_status(selected_task as usize - 1);
                        println!("{}", colored("Task updated!", "green"));
                        println!("Press any key to continue...");
                        _ = get_kb_input();
                    }
                    Err(_) => println!("{}", colored("Error getting resources!", "red")),
                }
            }
            2 => {
                options.print_ui_and_text(text);
                let result = get_resources();
                match result {
                    Ok(mut res) => {
                        if let Some(t) = res.get_task(selected_task as usize - 1) {
                            t.update();
                            res.save();
                            println!("{}", colored("Task Updated!", "green"));
                            println!("Press any key to continue...");
                            _ = get_kb_input();
                        } else {
                            println!("{}", colored("Error getting task!", "red"))
                        }
                    }
                    Err(_) => println!("{}", colored("Error getting resources!", "red")),
                }
            }
            3 => {
                options.print_ui_and_text(text);
                let result = get_resources();
                match result {
                    Ok(mut res) => {
                        println!(
                            "{}",
                            colored("Are you sure you want to delete the task? (y|n)", "red")
                        );
                        let opt = input!();
                        if opt.to_lowercase().starts_with("y") {
                            res.remove_task(selected_task as usize - 1);
                            println!("{}", colored("Success!", "green"));
                        } else {
                            println!("{}", colored("Operation canceled!", "green"));
                        }
                        println!("Press any key to continue...");
                        _ = get_kb_input();
                    }
                    Err(_) => println!("{}", colored("Error getting resources!", "red")),
                }
            }
            _ => show_tasks(),
        },
        _ => show_tasks(),
    }
    main_menu()
}

fn show_tasks() {
    let result = get_resources();
    match result {
        Ok(res) => {
            if res.is_tasks_empty() {
                println!(
                    "{}",
                    colored("Now it's the time you add some tasks!", "green")
                );
                println!("Press any key to continue...");
                _ = get_kb_input();
                main_menu();
            }
            let mut options = Options::default();
            options.build_from_tasks(res);
            let res = options.print_option("Your tasks:");
            match options.last_move {
                4 => task_menu(options.get_text_from_index(res as usize), res),
                3 => main_menu(),
                _ => println!("{}", options.last_move),
            }
        }
        Err(_) => println!("{}", colored("Error getting resources!", "red")),
    }
}

fn show_schedules() {
    let result = get_resources();
    match result {
        Ok(res) => {
            if res.is_schedules_empty() {
                println!(
                    "{}",
                    colored("Now it's the time you add some schedules!", "green")
                );
                println!("Press any key to continue...");
                _ = get_kb_input();
                main_menu();
            }
            let mut options = Options::default();
            options.build_from_schedules(res);
            let res = options.print_option("Your tasks:");
            match options.last_move {
                4 => schedule_menu(options.get_text_from_index(res as usize), res),
                3 => main_menu(),
                _ => println!("{}", options.last_move),
            }
        }
        Err(_) => println!("{}", colored("Error getting resources!", "red")),
    }
}

fn tasks_menu() {
    let text = "Manage your tasks:";
    let mut options = Options::default();
    let opt_lst = Vec::from([String::from("Check tasks"), String::from("Add task")]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    match selected {
        1 => show_tasks(),
        2 => {
            add_from_input(Resource::TaskResource);
            main_menu();
        }
        _ => main_menu(),
    }
}

fn schedules_menu() {
    let text = "Manage your schedules:";
    let mut options = Options::default();
    let opt_lst = Vec::from([
        String::from("Check schedules"),
        String::from("Add schedule"),
    ]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    match selected {
        1 => show_schedules(),
        2 => {
            add_from_input(Resource::ScheduleResource);
            main_menu();
        }
        _ => main_menu(),
    }
}

pub fn main_menu() {
    let text = "󰍜 Main Menu";
    let mut options = Options::default();
    let opt_lst = Vec::from([
        String::from("  Manage tasks"),
        String::from("󰀡  Manage schedules"),
    ]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    // options.print_radio_option(text, false);
    match selected {
        1 => tasks_menu(),
        2 => schedules_menu(),
        _ => main_menu(),
    }
}
