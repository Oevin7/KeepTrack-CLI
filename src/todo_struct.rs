use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone ,Debug)]
pub struct Todo {
    task: String,
    is_completed: bool,
    importance: i32,
    hidden : bool,
}

impl Todo {
    pub fn new(task: String, importance: i32) -> Self {
        Todo {
            task,
            is_completed: false,
            importance,
            hidden: false,
        }
    }

    pub fn change_status(&mut self) {
        self.is_completed = !self.is_completed
    }

    pub fn change_hidden(&mut self) {
        self.hidden = !self.hidden
    }

    pub fn get_task(&self) -> &str {
        self.task.as_str()
    }

    pub fn get_status(&self) -> bool {
        self.is_completed
    }

    pub fn get_importance(&self) -> i32 {
        self.importance
    }

    pub fn get_hidden(&self) -> bool {
        self.hidden
    }

    pub fn change_importance(&mut self, new_value: i32) -> i32 {
        if new_value < 1 || new_value > 4 {
            panic!("Value must be higher than 0 and lower than 4.");
        }

        self.importance = new_value;

        self.importance
    }

}

