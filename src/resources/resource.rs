
use serde::{Deserialize, Serialize};

use crate::{
    colors::colored,
    data::write_resources,
    resources::{Schedule, Task},
};

#[derive(Serialize, Deserialize)]
pub struct Resources {
    pub tasks: Vec<Task>,
    pub schedules: Vec<Schedule>,
}

impl Resources {
    // General stuff
    pub fn save(&self) {
        write_resources(self);
    }

    // Task stuff
    pub fn push_task(&mut self, mut task: Task) {
        let index = self.tasks_len();
        task.index = index;
        self.tasks.insert(index, task);
    }

    pub fn print_tasks(&self) {
        for task in self.tasks.iter() {
            println!("{task}")
        }
    }

    pub fn tasks_len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_tasks_empty(&self) -> bool {
        self.tasks_len() == 0
    }

    pub fn get_task_as_text(&self, idx: usize) -> String {
        if let Some(t) = self.tasks.get(idx) {
            let task = format!("{} - ({})", t.description, t.due_date);
            if !t.completed {
                task.to_string()
            } else {
                colored(&task, "green").to_string()
            }
        } else {
            String::from("There was an error reading the task!")
        }
    }

    pub fn remove_task(&mut self, idx: usize) {
        self.tasks.remove(idx);
        self.reindex_tasks();
        self.save();
    }

    pub fn get_task(&mut self, idx: usize) -> Option<&mut Task> {
        self.tasks.get_mut(idx)
    }

    pub fn reindex_tasks(&mut self) {
        for (counter, task) in self.tasks.iter_mut().enumerate() {
            task.index = counter;
        }
    }

    pub fn change_task_status(&mut self, idx: usize) {
        if let Some(t) = &mut self.tasks.get_mut(idx) {
            t.completed = !t.completed;
            write_resources(self);
        } else {
            println!("{}", colored("There was an error reading the task!", "red"));
        }
    }

    // Schedule stuff
    pub fn push_schedule(&mut self, mut schedule: Schedule) {
        let index = self.schedules_len();
        schedule.index = index;
        self.schedules.insert(index, schedule);
    }

    pub fn print_schedules(&self) {
        for schedule in self.schedules.iter() {
            println!("{schedule}")
        }
    }

    pub fn schedules_len(&self) -> usize {
        self.schedules.len()
    }

    pub fn is_schedules_empty(&self) -> bool {
        self.schedules_len() == 0
    }

    pub fn get_schedule_as_text(&self, idx: usize) -> String {
        if let Some(s) = self.schedules.get(idx) {
            let schedule = format!("{} - ({})", s.description, s.weekdays.get_as_text());
            if !s.completed {
                schedule.to_string()
            } else {
                colored(&schedule, "green").to_string()
            }
        } else {
            String::from("There was an error reading the schedule!")
        }
    }

    pub fn remove_schedule(&mut self, idx: usize) {
        self.schedules.remove(idx);
        self.reindex_schedules();
        self.save();
    }

    pub fn get_schedule(&mut self, idx: usize) -> Option<&mut Schedule> {
        self.schedules.get_mut(idx)
    }

    pub fn reindex_schedules(&mut self) {
        for (counter, schedule) in self.schedules.iter_mut().enumerate() {
            schedule.index = counter;
        }
    }
}
