use std::collections::hash_map::HashMap;
use std::ops::Range;
use std::vec::Vec;

use crate::{
    colors::colored,
    input::{
        get_kb_input,
        clean_terminal
    },
    task::TaskVec,
};

pub struct Options {
    options: Vec<usize>,
    options_map: HashMap<usize, String>,
    pub selected: usize,
    option_selected: bool
}

impl Options {
    pub fn print_option(&mut self, text_before: &str) -> i8 {
        let opt: usize = 0;
        let max = self.options.len();
        self.selected = 1;
        while opt < 1 {
            self.print_gui();
            println!("{}", colored(text_before, "yellow"));
            for i in self.options.iter() {
                let k = i;
                let v = if let Some(val) = self.options_map.get(k) {
                    val
                } else { "" };

                // Tratamento de Barreiras
                if self.selected > max {
                    self.selected = *k;
                }
                if self.selected == 0 {
                    self.selected = max;
                }

                // Efeito visual para opção selecionada
                let opt = format!("» {k}: {v}");

                if self.selected == *k {
                    println!(" {}", colored(&opt, "green"))
                } else {
                    println!("{opt}");
                }

            }
            let direction = get_kb_input();
            self.selected = match direction {
                1 => self.selected - 1,
                2 => self.selected + 1,
                3 => return 3,
                4 => return self.selected as i8,
                10 => {
                    self.option_selected = true;
                    break
                },
                120 => {
                    println!("{}", colored("So long...", "blue"));
                    break
                },
                _ => self.selected,
            };
        }
        if self.option_selected {
            // let v = if let Some(val) = self.options_map.get(&self.selected) {
            //     val
            // } else { "" };
            // println!("Option Selected: {}", v);
            self.selected as i8
        } else {
            -1
        }
    }
    
    fn print_gui(&self) {
        clean_terminal();
        println!("{}", colored("|----- TermPlanner -----|", "yellow"));
        println!("{}", colored("Press `x` to exit...", "red"));
        println!("{}", colored("Navigate with the arrows", "green"));
        println!("{}", colored("|-----------------------|\n", "yellow"));
    }

    pub fn build(&mut self, options: Vec<String>) {
        let end = options.len();
        let rng = Range {start: 1, end: end+1};
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map.insert(*i, options[*i-1].clone());
        }
    }

    pub fn build_from_tasks(&mut self, options: TaskVec) {
        let end = options.len();
        let rng = Range {start: 1, end: end+1};
        self.options.extend(rng);
        for i in self.options.iter() {
            self.options_map.insert(*i, options.get(i-1));
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            options: Vec::new(),
            options_map: HashMap::new(),
            selected: 0,
            option_selected: false
        }
    }
}
