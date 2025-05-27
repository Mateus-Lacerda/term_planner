use term_planner::{
    io_utils::clean_terminal,
    menus::main_menu,
    notify::{run_notification_service, send_notify},
};

fn resolve_args(cmd: &str) -> bool {
    match cmd {
        "--notify" => {
            run_notification_service();
            true
        }
        "--version" => {
            println!(version::version!());
            true
        }
        _ => false,
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(cmd) = args.next() {
        if resolve_args(&cmd) {
            return;
        }
    }
    send_notify("Task planner started!", false);
    clean_terminal();
    main_menu();
}
