use std::ffi::OsStr;
use std::{fs, io};
use std::path::PathBuf;
use crate::file_management::{read_and_return, write_file};
use crate::todo_struct::*;
use crate::user_handling::input;

//Adds tasks to the list
pub fn add_to_list(task : Todo, list: &mut Vec<Todo>) -> Vec<Todo> {
    list.push(task);

    let return_list = list.clone();

    return_list

}

//Removes tasks from the list
pub fn remove_task(todo_list : &mut Vec<Todo>, index : usize) {

    todo_list.remove(index);

}

//Marks a task as completed
pub fn mark_completed(todo_list : &mut Vec<Todo>, completed_task : usize) {

    todo_list[completed_task].change_status();

}

//Creates a new task when one is added
pub fn create_task(task : &str, importance : i32) -> Todo {
    let new_task = Todo::new(task.parse().unwrap(), importance);
    new_task
}

pub fn hide_task(todo_list : &mut Vec<Todo>,task_to_hide : usize) {

    todo_list[task_to_hide].change_hidden();

}

pub fn filter_tasks_by_importance(todo_list : Vec<Todo> ,importance : i32) {

    for task in todo_list.clone() {
        if task.get_importance() == importance {
            print_tasks(task);
        }
    }
}

pub fn filter_tasks_by_status(todo_list : Vec<Todo>) {
    let mut status = false;

    loop {
        println!("Would you like to see completed or uncompleted tasks? (completed or c | uncompleted or u)");
        let user_in = input().unwrap();

        if user_in.trim() == "completed" || user_in.trim() == "c" {
            status = true;
            break
        } else if user_in.trim() == "uncompleted" || user_in.trim() == "u" {
            break
        } else {
            println!("Invalid input. Please try again.");
        }
    }

    for task in todo_list {
        if task.get_status() == status {
            print_tasks(task);
        }
    }

}

//Changes the importance of a task
pub fn change_importance(mut todo_list : &mut Vec<Todo>, new_importance : i32, index: usize)  {

    todo_list[index].change_importance(new_importance);

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

pub fn find_task_by_partial_name(todo_list: &Vec<Todo>, partial_name: &str) -> Option<usize> {
    todo_list.iter().position(|task| task.get_task().contains(partial_name))
}

pub fn find_task_by_name(todo_list: &Vec<Todo>, name: &str) -> Option<usize> {
    todo_list.iter().position(|task| task.get_task().to_lowercase() == name.to_lowercase())
}

pub fn is_full_name(todo_list: &Vec<Todo>, name: &str) -> bool {
    let full_name = find_task_by_name(todo_list, name);

    match full_name {
        Some(_) => true,
        None => false,
    }

}

pub fn match_task(todo_list: &Vec<Todo>, task: &str) -> Result<usize, io::Error> {

    if is_full_name(todo_list, task) {
        let index = find_task_by_name(todo_list, task).unwrap();
        Ok(index)
    } else {
        let index = find_task_by_partial_name(todo_list, task).unwrap();
        Ok(index)
    }

}