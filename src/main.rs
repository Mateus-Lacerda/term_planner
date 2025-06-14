use std::vec::Vec;

use term_planner::{
    colors::colored,
    data::{add_task, get_tasks},
    input,
    io_utils::{clean_terminal, get_kb_input},
    notify::{run_notification_service, send_notify},
    options::Options,
    task::Task,
};

fn add_task_from_input() {
    let res = add_task();
    match res {
        Ok(_) => println!("{}", colored("Task added!", "green")),
        Err(_) => println!("{}", colored("Error!", "red")),
    }
}

fn edit_task_from_input(task: &mut Task) {
    task.update();
}

fn task_menu(text: &str, selected_task: i8) {
    let result = get_tasks();
    let d_or_ud = match result {
        Ok(mut res) => {
            let comp = res.get(selected_task as usize - 1).expect("").completed;
            if comp { "undone" } else { "done" }
        }
        Err(_) => "",
    };

    if d_or_ud.is_empty() {
        println!("{}", colored("Error!", "red"));
        println!("Press any key to continue...");
        _ = get_kb_input();
        menu();
    }

    let d_or_ud = format!("Mark as {}.", &d_or_ud);

    let mut options = Options::default();
    let opt_lst = Vec::from([d_or_ud, String::from("Edit."), String::from("Delete.")]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    match options.last_move {
        4 => match selected {
            1 => {
                let result = get_tasks();
                match result {
                    Ok(mut res) => {
                        res.change_status(selected_task as usize - 1);
                        println!("{}", colored("Task Updated!", "green"));
                        println!("Press any key to continue...");
                        _ = get_kb_input();
                    }
                    Err(_) => println!("{}", colored("Error!", "red")),
                }
            }
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
                            _ = get_kb_input();
                        } else {
                            println!("{}", colored("Error!", "red"))
                        }
                    }
                    Err(_) => println!("{}", colored("Error!", "red")),
                }
            }
            3 => {
                options.print_ui_and_text(text);
                let result = get_tasks();
                match result {
                    Ok(mut res) => {
                        println!(
                            "{}",
                            colored("Are you sure you want to delete the task? (y|n)", "red")
                        );
                        let opt = input!();
                        if opt.to_lowercase().starts_with("y") {
                            res.remove(selected_task as usize - 1);
                            println!("{}", colored("Success!", "green"));
                        } else {
                            println!("{}", colored("Operation canceled!", "green"));
                        }
                        println!("Press any key to continue...");
                        _ = get_kb_input();
                    }
                    Err(_) => println!("{}", colored("Error!", "red")),
                }
            }
            _ => show_tasks(),
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
            if res.is_empty() {
                println!(
                    "{}",
                    colored("Now it's the time you add some tasks!", "green")
                );
                println!("Press any key to continue...");
                _ = get_kb_input();
                menu();
            }
            let mut options = Options::default();
            options.build_from_tasks(res);
            let res = options.print_option("Your tasks:");
            match options.last_move {
                4 => task_menu(options.get_text_from_index(res as usize), res),
                3 => menu(),
                _ => println!("{}", options.last_move),
            }
        }
        Err(_) => println!("{}", colored("Error!", "red")),
    }
}

fn menu() {
    let text = "  Main Menu";
    let mut options = Options::default();
    let opt_lst = Vec::from([String::from("Check tasks"), String::from("Add task")]);

    options.build(opt_lst);
    let selected = options.print_option(text);
    match selected {
        1 => show_tasks(),
        2 => {
            add_task_from_input();
            menu();
        }
        3 => menu(),
        _ => menu(),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(cmd) = args.next() {
        if cmd == "--notify" {
            run_notification_service();
            return;
        }
    }
    send_notify("Task planner started!", false);
    clean_terminal();
    menu();
}
