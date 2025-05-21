use std::vec::Vec;

use chrono::{TimeZone, Utc};

use term_planner::{
    input,
    integer_input,
    colors::colored,
    options::Options,
    data::{
        add_task,
        get_tasks
    },
    task::Task,
    io_utils::clean_terminal
};

fn add_task_from_input(
) {
    println!("Describe your task:");
    let description: String = input!();
    println!("Select the day:");
    let day: u32 = integer_input!() as u32;
    println!("Select the month:");
    let month: u32 = integer_input!() as u32;
    println!("Select the year:");
    let year: i32 = integer_input!();
    println!("Select the hour:");
    let hour: u32 = integer_input!() as u32;
    println!("Select the minute:");
    let min: u32 = integer_input!() as u32;

    if let Some(date) = Utc.with_ymd_and_hms(year, month, day, hour, min, 0).earliest() {
        let res = add_task(&description, date);
        match res {
            Ok(_) => println!("{}", colored("Task added!", "green")),
            Err(_) => println!("{}", colored("Error!", "red")),
        }
    } else { println!("{}", colored("Error!", "red")) }
}

fn edit_task_from_input(
    task: &mut Task
) {
    println!("Leave it empty if you don't want to edit.");
    println!("Edit the description:");
    let description: String = input!();
    println!("Select the new day:");
    let day: u32 = integer_input!() as u32;
    println!("Select the new month:");
    let month: u32 = integer_input!() as u32;
    println!("Select the new year:");
    let year: i32 = integer_input!();
    println!("Select the new hour:");
    let hour: u32 = integer_input!() as u32;
    println!("Select the new minute:");
    let min: u32 = integer_input!() as u32;

    if let Some(date) = Utc.with_ymd_and_hms(year, month, day, hour, min, 0).earliest() {
        task.update(&description, date);
    } else { println!("{}", colored("Error!", "red")) }
}

fn task_menu(text: &str, selected_task: i8) {
    let mut options = Options::default();
    let opt_lst = Vec::from(
        [
            String::from("Mark as done."),
            String::from("Edit."),
            String::from("Delete."),
        ]
    );

    options.build(opt_lst);
    let selected = options.print_option(text);
    match options.last_move {
        4 => {
            match selected {
                1 => {
                    let result = get_tasks();
                    match result {
                        Ok(mut res) => {
                            res.complete(selected_task as usize - 1);
                            println!("{}", colored("Task Updated!", "green"));
                            println!("Press any key to continue...");
                            _ = input!();
                        },
                        Err(_) => println!("{}", colored("Error!", "red"))
                    }
                },
                2 => {
                    options.print_ui_and_text(text);
                    let result = get_tasks();
                    match result {
                        Ok(mut res) => {
                            if let Some(t) = res.get(selected_task as usize - 1) { 
                                edit_task_from_input(t);
                                res.save();
                                println!("{}", colored("Task Updated!", "green"));
                                println!("Press any key to continue...");
                                _ = input!();

                            } else { println!("{}", colored("Error!", "red")) }
                        },
                        Err(_) => println!("{}", colored("Error!", "red"))
                    }
                },
                3 => {
                    // Delete the task
                }
                _ => show_tasks(),
            }
        },
        3 => show_tasks(),
        _ => show_tasks(),
    }
    menu()
}

fn show_tasks() {
    let result = get_tasks();
    match result {
        Ok(res) => {
            let mut options = Options::default();
            options.build_from_tasks(res);
            let res = options.print_option("Your tasks:");
            match options.last_move {
                4 => task_menu(options.get_text_from_index(res as usize), res),
                3 => menu(),
                _ => println!("{}", options.last_move)
                
            }
        },
        Err(_) => println!("{}", colored("Error!", "red"))
    }
}

fn menu() {
    let text = "Main Menu";
    let mut options = Options::default();
    let opt_lst = Vec::from(
        [
            String::from("Check tasks"),
            String::from("Add task"),
        ]
    );

    options.build(opt_lst);
    let selected = options.print_option(text);
    match selected {
        1 => show_tasks(),
        2 => {
            add_task_from_input();
            menu();
        }
        3 => menu(),
        _ => menu()
    }
}

fn main() {
    clean_terminal();
    menu();
}
