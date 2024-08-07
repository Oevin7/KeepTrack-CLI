use std::cell::{Ref, RefCell};
use serde::{Deserialize, Serialize};
use crate::list_maintenance::{find_task_by_name, find_task_by_partial_name, is_full_name};
use crate::not_in_list_error::NotFoundInList;

#[derive(Serialize, Deserialize, Clone ,Debug, PartialEq)]
pub struct Todo {
    task: String,
    is_completed: bool,
    importance: i32,
    hidden : bool,
    tags : RefCell<Vec<String>>
}

impl Todo {
    pub fn new(task: String, importance: i32) -> Self {
        Todo {
            task,
            is_completed: false,
            importance,
            hidden: false,
            tags : RefCell::new(vec![]),
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
    pub fn get_tag_list(&self) -> &RefCell<Vec<String>> {
        &self.tags
    }
}

pub trait TagList<T> {
    fn find(&self, name : &str) -> Option<usize>;
}

impl TagList<usize> for Vec<Todo> {
    fn find(&self, name : &str) -> Option<usize> {
        if is_full_name(self, name) {
            let index = find_task_by_name(self, name);
            index
        } else {
            let index = find_task_by_partial_name(self, name);
            index
        }
    }
}

impl TagList<usize> for &Vec<Todo> {
    fn find(&self, name : &str) -> Option<usize> {
        if is_full_name(self, name) {
            let index = find_task_by_name(self, name);
            index
        } else {
            let index = find_task_by_partial_name(self, name);
            index
        }
    }
}


impl TagList<usize> for Todo {
    fn find(&self, tag: &str) -> Option<usize> {
        self.tags.borrow().iter().position(|t| t == tag || t.contains(tag))
    }
}

