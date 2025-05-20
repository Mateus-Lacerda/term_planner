use std::vec::Vec;

use chrono::{DateTime, TimeZone, Utc};

use term_planner::{
    colors::colored,
    options::Options,
    data::{
        add_task,
        get_tasks
    },
};

macro_rules! input {
    () => {{
        use std::io::{self, Write};
        print!(">> ");
        let mut buffer = String::new();
        // Garante que o prompt (se houver) aparece antes do input
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }};
}


fn add_task_from_input(
) {
    println!("Describe your task:");
    let description: String = input!();
    println!("Select the day:");
    let day: u32 = input!().parse().unwrap();
    println!("Select the month:");
    let month: u32 = input!().parse().unwrap();
    println!("Select the year:");
    let year: i32 = input!().parse().unwrap();
    println!("Select the hour:");
    let hour: u32 = input!().parse().unwrap();
    println!("Select the minute:");
    let min: u32 = input!().parse().unwrap();
    let date: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, day, hour, min, 0).unwrap();
    let res = add_task(&description, date);
    match res {
        Ok(_) => println!("{}", colored("Task added!", "green")),
        Err(_) => println!("{}", colored("Error!", "red")),
    }
    println!("Press any key to continue...");
    _ = input!();
}

fn main() {
    let text = "Main Menu";
    let mut options = Options::default();
    let opt_lst = Vec::from(
        [
            String::from("Check tasks"),
            String::from("Add task"),
            String::from("Remove tasks")
        ]
    );

    options.build(opt_lst);
    let selected = options.print_option(text);
    match selected {
        1 => {
            let result = get_tasks();
            match result {
                Ok(res) => {
                    let mut options = Options::default();
                    options.build_from_tasks(res);
                    let res = options.print_option("Your tasks:");
                    match res {
                        3 => { 
                            main() 
                        },
                        _ => () // TODO: Implement more logic here
                    }
                },
                Err(_) => println!("{}", colored("Error!", "red"))
            }
        },
        2 => {
            add_task_from_input();
            main();
        }
        3 => {
            println!("{}", colored("Not implemented!", "red"));
            println!("Press any key to continue...");
            _ = input!();
            main()
        }
        -1 => (),
        _ => main()
    }
}
