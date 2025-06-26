use std::ops::Range;
use std::vec::Vec;
use std::{collections::hash_map::HashMap};

use chrono::{FixedOffset, Utc};
use libc::exit;

use crate::{
    colors::colored,
    io_utils::{clean_terminal, get_kb_input},
    resources::Resources,
};

pub struct Options {
    options: Vec<usize>,
    options_map: HashMap<usize, String>,
    pub selected: usize,
    option_selected: bool,
    pub last_move: usize,
}

impl Options {
    pub fn print_option(&mut self, text_before: &str) -> i8 {
        let opt: usize = 0;
        let max = self.options.len();
        self.selected = 1;
        if opt < 1 {
            loop {
                self.print_tui();
                println!("{}", colored(text_before, "yellow"));
                for i in self.options.iter() {
                    let k = i;
                    let v = if let Some(val) = self.options_map.get(k) {
                        val
                    } else {
                        ""
                    };

                    // Tratamento de Barreiras
                    if self.selected > max {
                        self.selected = *k;
                    }
                    if self.selected == 0 {
                        self.selected = max;
                    }

                    // Efeito visual para opção selecionada
                    let opt = format!("‣ {k}: {v}");

                    if self.selected == *k {
                        println!(" {}", colored(&opt, "yellow"))
                    } else {
                        println!("{opt}");
                    }
                }
                let direction = get_kb_input();
                self.last_move = direction;
                self.selected = match direction {
                    1 => self.selected - 1,
                    2 => self.selected + 1,
                    3 => return 3,
                    4 => {
                        self.option_selected = true;
                        break;
                    }
                    10 => {
                        self.last_move = 4;
                        self.option_selected = true;
                        break;
                    }
                    113 => {
                        println!("{}", colored("So long...", "blue"));
                        unsafe { exit(1) }
                    }
                    _ => self.selected,
                };
            }
        }
        self.selected as i8
    }

    pub fn print_radio_option_unmarked(
        &mut self,
        text_before: &str,
        unique: bool,
    ) -> HashMap<usize, String> {
        let selected_map: HashMap<usize, String> = HashMap::new();
        return self.print_radio_option(text_before, unique, selected_map)
    }

    pub fn print_radio_option(
        &mut self,
        text_before: &str,
        unique: bool,
        mut selected_map: HashMap<usize, String>
    ) -> HashMap<usize, String> {
        let opt: usize = 0;
        let max = self.options.len();
        self.selected = 1;
        if opt < 1 {
            loop {
                self.print_tui();
                println!("{}", colored(text_before, "yellow"));

                if self.option_selected {
                    match selected_map.contains_key(&self.selected) {
                        true => {
                            selected_map.remove(&self.selected);
                        }
                        false => {
                            selected_map.insert(
                                self.selected,
                                self.options_map
                                    .get(&self.selected)
                                    .expect("Error!")
                                    .clone(),
                            );
                        }
                    }
                    if unique && *&selected_map.len() > 1 {
                        selected_map.retain(|x,_| x == &self.selected);
                    }
                }
                for i in self.options.iter() {
                    let k = i;
                    let v = if let Some(val) = self.options_map.get(k) {
                        val
                    } else {
                        ""
                    };

                    // Tratamento de Barreiras
                    if self.selected > max {
                        self.selected = *k;
                    }
                    if self.selected == 0 {
                        self.selected = max;
                    }

                    // Efeito visual para opção selecionada
                    let opt = if selected_map.contains_key(&k) {
                        format!("󰡖 {k}: {v}")
                    } else {
                        format!("󰄱 {k}: {v}")
                    };

                    if self.selected == *k {
                        println!(" {}", colored(&opt, "yellow"))
                    } else {
                        println!("{opt}");
                    }
                }
                let direction = get_kb_input();
                self.last_move = direction;
                self.option_selected = false;
                self.selected = match direction {
                    1 => self.selected - 1,
                    2 => self.selected + 1,
                    3 => return HashMap::from([(3, String::from("Cancel"))]),
                    4 => {
                        self.option_selected = true;
                        self.selected
                    }
                    10 => break,
                    32 => {
                        self.option_selected = true;
                        self.last_move = 4;
                        self.selected
                    }
                    113 => {
                        println!("{}", colored("So long...", "blue"));
                        unsafe { exit(1) }
                    }
                    // _ => {
                    //     println!("{direction}");
                    //     unsafe { exit(1) }
                    // }
                    _ => self.selected,
                };
            }
        }
        // for (k, v) in selected_map.iter() {
        //     println!("{k}, {v}");
        // }
        selected_map
    }

    pub fn print_ui_and_text(&mut self, text: &str) {
        self.print_tui();
        println!("{}", colored(text, "yellow"));
    }

    pub fn get_text_from_index(&self, idx: usize) -> &str {
        if let Some(opt) = self.options_map.get(&idx) {
            opt as &str
        } else {
            ""
        }
    }

    fn now(&self) -> String {
        let offset = FixedOffset::west_opt(3 * 3600).expect("");
        Utc::now()
            .with_timezone(&offset)
            .format("%d/%m/%Y %H:%M")
            .to_string()
    }

    fn print_tui(&self) {
        clean_terminal();
        println!("{}", colored("|----  TermPlanner ----|\n", "yellow"));
        println!("{}", colored("   Press q to exit...   ", "red"));
        println!("{}", colored(" Navigate with      \n", "green"));
        println!("{}", colored(&format!("     {}    ", self.now()), "green"));
        println!("{}", colored("|-----------------------|\n", "yellow"));
    }

    pub fn build(&mut self, options: Vec<String>) {
        let end = options.len();
        let rng = Range {
            start: 1,
            end: end + 1,
        };
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map.insert(*i, options[*i - 1].clone());
        }
    }

    pub fn build_from_tasks(&mut self, options: Resources) {
        let end = options.tasks_len();
        let rng = Range {
            start: 1,
            end: end + 1,
        };
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map.insert(*i, options.get_task_as_text(i - 1));
        }
    }

    pub fn build_from_schedules(&mut self, options: Resources) {
        let end = options.schedules_len();
        let rng = Range {
            start: 1,
            end: end + 1,
        };
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map
                .insert(*i, options.get_schedule_as_text(i - 1));
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            options: Vec::new(),
            options_map: HashMap::new(),
            selected: 0,
            option_selected: false,
            last_move: 10,
        }
    }
}
