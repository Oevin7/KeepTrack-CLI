mod user_handling;
mod file_management;

use std::{env, fs, io};
use std::ffi::OsStr;
use text_io::read;
use list::list::Todo;
use std::fs::File;
use std::io::{ Read, Write};
use std::path::{Path, PathBuf};
use colored::Colorize;
use serde_json::to_writer;
use user_handling::{input, read_flag_values};
use file_management::{read_and_return, write_file, delete_file, auto_clean_flag, create_file, write_flag_values};
use crate::file_management::{get_absolute_path, write_current_list};

fn main() {
    let auto_clean : bool = read_flag_values().unwrap();

    let home_dir = dir::home_dir().unwrap();

    let mut full_dir = home_dir.join(".keeptrack-cli").join("lists");

    let file_path = &full_dir.clone();
    let path = Path::new(&file_path);

    let current_list_path = file_path.join("current_list.txt");
    let default_list = String::from("todo_list.json");

    if !current_list_path.exists() {
        write_current_list(&default_list);
    }

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Directory created successfully!"),
        Err(e) => eprintln!("Problem creating directory: {:?}", e),
    }

    if !path.exists() {
        fs::create_dir(&path).expect("Failed to create directory");
    }

    let current_list = get_absolute_path(default_list, file_path);

    let mut todo_list : Vec<Todo> = read_and_return(&current_list).unwrap();

    let args : Vec<String> = env::args().collect();

    let command = parse_commands(&args);

    handle_command(&mut todo_list, file_path, path.exists(), auto_clean, command, &current_list);


}

//Adds tasks to the list
fn add_to_list(task : Todo, list: &mut Vec<Todo>) {
    list.push(task);
}

//Removes tasks from the list
fn remove_task(todo_list : &mut Vec<Todo>, task_to_remove : &str) {

    for task in 0..todo_list.len() {
        if todo_list[task].get_task() == task_to_remove {
            todo_list.remove(task);
        }
    }

}

//Marks a task as completed
fn mark_completed(todo_list : &mut Vec<Todo>, completed_task : &str) {

    for mut task in todo_list {
        if task.get_task() == completed_task {
            task.change_status();
        }
    }

}

//Creates a new task when one is added
fn create_task(task : &str, importance : i32) -> Todo {
    let new_task = Todo::new(task.parse().unwrap(), false, importance, false);
    new_task
}

fn hide_task(todo_list : &mut Vec<Todo>,task_to_hide : &str) {

    for task in todo_list {
        if task.get_task() == task_to_hide {
            task.change_hidden();
        }
    }

}

fn filter_tasks_by_importance(todo_list : &mut Vec<Todo> ,importance : i32) {

    for task in todo_list {
        if task.get_importance() == importance {
            print_tasks(task);
        }
    }

}

//Changes the importance of a task
fn change_importance(todo_list : &mut Vec<Todo>, new_importance : i32, name_of_task : &str) {

    for task in todo_list {
        if task.get_task() == name_of_task {
            task.change_importance(new_importance);
        }
    }

}

//Parses the commands to later handle the input
fn parse_commands(args : &[String]) -> Option<String> {
    let command = args.get(1).unwrap_or(&String::from("")).clone();

    Some(command)

}

//Handles the commands that were parsed
fn handle_command(todo_list : &mut Vec<Todo>, file_path : &PathBuf, path : bool, auto_clean : bool, mut intro_command: Option<String>, current_list : &PathBuf) {

    if !path {
        let mut file = File::create(file_path.to_str().unwrap()).expect("Could not create the file.");
        let empty_vec : Vec<Todo> = Vec::new();

        if let Err(e) = to_writer(&mut file, &empty_vec) {
            eprintln!("Failed to write to file {:?}: {}", file_path, e);
        }

    }

    let command_check = intro_command.clone();

    if command_check.is_some() {
        if command_check.unwrap() == "" {
            intro_command = None;
        }
    }

    match intro_command {
        Some(command) => execute_commands(command, todo_list, file_path, auto_clean, current_list),
        None => loop {

            println!("Please input what you want to do next? For the list of commands type help: ");
            let mut command = input().unwrap();

            command = command.trim().parse().unwrap();

            if command == "exit" || command == "e" || command == "quit" || command == "q" {
                if auto_clean {
                    clean(file_path);
                }
                break
            }

            execute_commands(command, todo_list, file_path, auto_clean, current_list);

        }
    }

}

fn execute_commands(command: String, todo_list: &mut Vec<Todo>, file_path : &PathBuf, auto_clean : bool, current_list : &PathBuf) {

    match command.to_lowercase().trim() {
        "list" | "l" => {
            list_tasks(todo_list);
        }
        "list -h" => list_hidden(todo_list),
        "list -all" => list_all(file_path),
        "add" | "a" => {
            loop {
                println!("What task would you like to add?");

                println!("Please input the task: ");
                let mut task = String::new();

                io::stdin()
                    .read_line(&mut task)
                    .expect("Please input a task.");

                println!("{}", "How important is the task? (1 lowest level of importance, 4 is the highest.)
                Note: For now inputting a char or string value will make the program panic. Please only
                input a number value.".trim());
                let mut importance : i32 = read!();

                loop {
                    if importance < 1 || importance > 4 {
                        println!("Please input a number between 1 and 4");
                        importance = read!();
                    } else {
                        break
                    }
                }

                add_to_list(create_task(task.to_lowercase().trim(), importance), todo_list);

                println!("Would you like to add a new task or are you done adding tasks? (add/done): ");
                let input = input().expect("Could not unwrap String");

                if input.trim() == "done" || input.trim() == "d" {
                    write_file(todo_list, &current_list).expect("Could not parse the file");

                    if auto_clean {
                        clean(file_path);
                    }
                    break
                }
            }
        }
        "help" | "h" => {

            let list = "list".bright_cyan();
            let list_hidden = "list -h".bright_cyan();
            let add = "add".bright_cyan();
            let remove = "remove".bright_cyan();
            let importance = "importance".bright_cyan();
            let status = "status".bright_cyan();
            let clean = "clean".bright_cyan();
            let autoclean = "auto_clean".bright_cyan();
            let exit = "exit".bright_cyan();

            println!("* {}: Lists the tasks that are currently on your list. Uncompleted and
Completed will show up unless you use a filter, or when you exit the program.
Exiting automatically cleans up completed tasks if auto_clean is set to true.

* {}: Lists hidden tasks. Works the same as other tasks, they are just hidden from your
  current list.

* {}: Adds a task to your list. These can later be marked as completed or
modified to change their importance. You can also filter these tasks later to only
view the ones you need.

* {}: Removes a task from your list. All tasks marked completed are automatically
removed when the program exits. However, you can preemptively remove tasks if you'd
like!

* {}: Allows you to modify the importance of your tasks. You can change their
importance level from an integer between 1 and 4! This will be helpful when you want
to filter tasks, but some tasks are no longer as urgent.

* {}: Status changes the completed status of your task. If you run status, it completes
  the specified task, or alternatively it will mark a task as incomplete if run on the
  same task.

* {}: Cleans up your completed tasks. If auto_clean is set to true, the program
will clean the completed tasks when the program exits. To set auto_clean, just run
todo auto_clean.

* {}: Sets auto_clean to true; run it again, it gets set to false. This
automatically deletes tasks marked as complete once the file exits.

* {}: Exits the program. If auto_clean is enabled, it will automatically delete
completed tasks.", list, list_hidden, add, remove, importance, status ,clean, autoclean, exit);

        }
        "remove" | "r" => {
            println!("Please input the task you would like to remove: ");
            let task_to_remove = input().expect("Couldn't get user input");

            remove_task(todo_list, task_to_remove.to_lowercase().trim());
            write_file(todo_list, current_list).unwrap();

        },
        "importance" | "i" => {
            println!("What task would you like to update?");
            let task = input().unwrap();

            println!("What level of importance would you like to change your task to? (1 - 4)");
            let new_importance = read!();

            change_importance(todo_list, new_importance, task.to_lowercase().trim());
            write_file(todo_list, current_list).unwrap()
        }
        "status" | "s" => {
            println!("What task do you need to change the status(completion) of?");
            let task = input().unwrap();

            mark_completed(todo_list, task.to_lowercase().trim());
            write_file(todo_list, current_list).unwrap()
        }
        "clean" | "c" => {
            clean(current_list);
        }
        "auto_clean" | "ac" => {
            write_flag_values(auto_clean_flag(auto_clean)).expect("Unable to set the flags. \
                Likely a file error");
        }
        "filter -fi" | "fi" => {
            println!("Please input an integer between 1-4");
            let importance : i32 = read!();

            filter_tasks_by_importance(todo_list, importance);
        }
        "hide" => {
            println!("Which task would you like to hide?");
            let task = input().unwrap();

            hide_task(todo_list, task.to_lowercase().trim());
            write_file(todo_list, current_list).unwrap()
        }
        "delete" => {

            println!("Please input the task you'd like to delete (Leave blank to delete the default list): ");
            let list_name = input().unwrap();

            if list_name == "" {
                delete_file(file_path, String::from("todo_list"));
            }

            delete_file(file_path, list_name.trim().to_string());

        }
        "create" => {

            println!("Please name your new list (Leave blank for default list): ");
            let list_name = input().unwrap();

            if list_name == "" {
                create_file(file_path, String::from("todo_list"));
            }

            create_file(file_path, list_name.trim().to_string());


        }
        "change" => {

            println!("Which list would you like to use instead: ");
            let list_name = input().unwrap();

            write_current_list(&list_name.trim().to_string());

        }
        _ => {
                panic!("NO FEATURES HERE!!!! ABORT, ABORT! TO LAZY TO PROPERLY HANDLE!");
        }
    }
}

//Lists the tasks that are on the list
fn list_tasks(todo_list : &mut Vec<Todo>) {

    for task in todo_list {
        if !task.get_hidden() {
            print_tasks(task);
        }
    }

}

fn list_hidden(todo_list : &mut Vec<Todo>) {

    for task in todo_list {
        if task.get_hidden() {
            print_tasks(task);
        }
    }

}

fn list_all(directory : &PathBuf) {
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
fn clean(path_to_file : &PathBuf) {
    let mut list = match read_and_return(path_to_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    list.retain(|task| !task.get_status());

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {:?}. Error {}", path_to_file, e);
    }
}

fn print_tasks(tasks : &mut Todo) {
    println!("Task: {}, Completed: {}, Importance: {}\n",
             tasks.get_task().replace("\n", ""),
             tasks.get_status(),
             tasks.get_importance());
}

