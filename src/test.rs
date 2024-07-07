#[cfg(test)]
mod tests {
    use serde::de::Unexpected::Str;
    use crate::todo_struct::*;
    use crate::list_maintenance::{change_importance, mark_completed, remove_task};
    use super::*;

    #[test]
    fn remove_task_test() {

        let mut todo_list = vec![
            Todo::new(String::from("Task 1"), 1),
            Todo::new(String::from("Task 2"), 2),
            Todo::new(String::from("Task 3"), 3),
        ];

        remove_task(&mut todo_list, "Task 1");

        assert_eq!(todo_list.len(), 2);
        assert_eq!(todo_list[0].get_task(), "Task 2");
        assert_eq!(todo_list[1].get_task(), "Task 3");
    }

    #[test]
    fn mark_complete_test() {

        let mut todo_list = vec![
            Todo::new(String::from("Hello!"), 2),
            Todo::new(String::from("Oye!"), 1),
        ];

        mark_completed(&mut todo_list, "Oye!");

        assert_eq!(todo_list[1].get_status(), true);

    }

    #[test]
    fn change_importance_test() {

        let mut todo_list = vec![
            Todo::new(String::from("This is a task"), 1),
            Todo::new(String::from("This is another task"), 2),
        ];

        let returned_list = change_importance(todo_list, 2, "This is a task");

        assert_eq!(returned_list[0].get_importance(), 2);

    }

}