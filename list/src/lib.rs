pub mod list {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize ,Debug)]
    pub struct Todo {
        task: String,
        is_completed: bool,
        importance: i32,
    }

    impl Todo {
        pub fn new(task : String, is_completed : bool, importance : i32) -> Self {
            Self {
                task,
                is_completed,
                importance,
            }
        }

        pub fn mark_complete(&mut self) {
            self.is_completed = true;
        }

        pub fn mark_incomplete(&mut self) {
            self.is_completed = false;
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

        pub fn change_importance(&mut self, new_value : i32) -> i32 {
            if new_value > 0 || new_value < 4 {
                panic!("Value must be higher than 0 and lower than 4.");
            }

            self.importance = new_value;

            self.importance

        }

    }

}
