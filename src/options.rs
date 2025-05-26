use std::collections::hash_map::HashMap;
use std::ops::Range;
use std::vec::Vec;

use chrono::{FixedOffset, Utc};
use libc::exit;

use crate::{
    colors::colored,
    io_utils::{clean_terminal, get_kb_input},
    task::TaskVec,
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
                self.print_gui();
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
                    120 => {
                        println!("{}", colored("So long...", "blue"));
                        unsafe { exit(1) }
                    }
                    _ => self.selected,
                };
            }
        }
        self.selected as i8
    }

    pub fn print_ui_and_text(&mut self, text: &str) {
        self.print_gui();
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

    fn print_gui(&self) {
        clean_terminal();
        println!("{}", colored("|----  TermPlanner ----|\n", "yellow"));
        println!("{}", colored("   Press  to exit...   ", "red"));
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

    pub fn build_from_tasks(&mut self, options: TaskVec) {
        let end = options.len();
        let rng = Range {
            start: 1,
            end: end + 1,
        };
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map.insert(*i, options.get_as_text(i - 1));
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
