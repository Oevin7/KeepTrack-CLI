use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use crate::file_management::{read_and_return, write_file};
use crate::todo_struct::*;

//Adds tasks to the list
pub fn add_to_list(task : Todo, mut list: &mut Vec<Todo>) -> Vec<Todo> {
    list.push(task);

    let return_list = list.clone();

    return_list

}

//Removes tasks from the list
pub fn remove_task(mut todo_list : &mut Vec<Todo>, task_to_remove : &str) {

    for task in 0..todo_list.len() {
        if todo_list[task].get_task() == task_to_remove {
            &mut todo_list.remove(task);
        }
    }

}

//Marks a task as completed
pub fn mark_completed(todo_list : &mut Vec<Todo>, completed_task : &str) -> Vec<Todo> {
    for mut task in &mut *todo_list {
        if task.get_task() == completed_task {
            task.change_status();
        }
    }

    let return_list = todo_list.to_owned();

    return_list

}

//Creates a new task when one is added
pub fn create_task(task : &str, importance : i32) -> Todo {
    let new_task = Todo::new(task.parse().unwrap(), importance);
    new_task
}

pub fn hide_task(mut todo_list : Vec<Todo>,task_to_hide : &str) {

    for mut task in todo_list {
        if task.get_task() == task_to_hide {
            task.change_hidden();
        }
    }

}

pub fn filter_tasks_by_importance(mut todo_list : Vec<Todo> ,importance : i32) {

    for mut task in todo_list {
        if task.get_importance() == importance {
            print_tasks(task);
        }
    }

}

//Changes the importance of a task
pub fn change_importance(todo_list : Vec<Todo>, new_importance : i32, name_of_task : &str) {

    for mut task in todo_list {
        if task.get_task() == name_of_task {
            task.change_importance(new_importance);
        }
    }

}

//Parses the commands to later handle the input
pub fn parse_commands(args : &[String]) -> Option<String> {
    let command = args.get(1).unwrap_or(&String::from("")).clone();

    Some(command)

}

//Lists the tasks that are on the list
pub fn list_tasks(todo_list : Vec<Todo>) {

    for task in todo_list {
        if !task.get_hidden() {
            print_tasks(task);
        }
    }

}

pub fn list_hidden(todo_list : Vec<Todo>) {

    for task in todo_list {
        if task.get_hidden() {
            print_tasks(task);
        }
    }

}

pub fn list_all(directory : &PathBuf) {
    let entries = fs::read_dir(directory).expect("Could not read directory.");
    let extension = OsStr::new("json");

    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();

        if path.is_file() && path.extension() == Some(extension) {
            match path.file_stem() {
                Some(name) => {
                    let lists = name.to_str().unwrap();

                    println!("{lists}");

                },
                None => println!("Could not get file name"),
            }
        }
    }
}


//Cleans up and removes all completed tasks
pub fn clean(path_to_file : &PathBuf) {
    let mut list = match read_and_return(path_to_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    list.retain(|task| !task.get_status());

    if let Err(e) = write_file(&list, path_to_file) {
        eprintln!("Could not write to file: {:?}. Error {}", path_to_file, e);
    }
}

pub fn print_tasks(tasks : Todo) {
    println!("Task: {}, Completed: {}, Importance: {}\n",
             tasks.get_task().replace("\n", ""),
             tasks.get_status(),
             tasks.get_importance());
}
