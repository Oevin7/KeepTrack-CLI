mod user_handling;
mod file_management;
mod list_maintenance;

use std::{env, fs, io};
use text_io::read;
use list::list::Todo;
use std::fs::{create_dir, File};
use std::io::{ Read, Write};
use std::path::{Path, PathBuf};
use colored::Colorize;
use serde_json::to_writer;
use user_handling::{input, read_flag_values};
use file_management::*;
use crate::file_management::{get_absolute_path, write_current_list, read_current_list};
use crate::list_maintenance::*;

fn main() {
    let auto_clean : bool = read_flag_values().unwrap();

    let home_dir = dir::home_dir().unwrap();

    let mut full_dir = home_dir.join(".keeptrack-cli").join("lists");

    let file_path = &full_dir.clone();
    let path = Path::new(&file_path);

    let current_list_file = PathBuf::from("./current_list.txt");
    let default_list = String::from("todo_list");
    let default_list_path = &file_path.join(&default_list);

    if !default_list_path.exists() {
        create_file(file_path, default_list.clone());
    }

    if !current_list_file.exists() {
        write_current_list(&default_list);
    }

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Directory created successfully!"),
        Err(e) => eprintln!("Problem creating directory: {:?}", e),
    }

    if !path.exists() {
        create_dir(&path).expect("Failed to create directory");
    }

    let current_list = read_current_list(&current_list_file).expect("Could not find the file");
    let current_list_path = get_absolute_path(current_list, file_path);

    println!("current list path: {:?}", current_list_path);

    let mut todo_list : Vec<Todo> = read_and_return(&current_list_path).expect("Could not read file");

    let args : Vec<String> = env::args().collect();

    let command = parse_commands(&args);

    handle_command(todo_list, file_path, path.exists(), auto_clean, command, &current_list_path);


}

//Handles the commands that were parsed
fn handle_command(todo_list : Vec<Todo>, file_path : &PathBuf, path : bool, auto_clean : bool, mut intro_command: Option<String>, current_list : &PathBuf) {

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

            let current_list_name = read_current_list(current_list).expect("Could not read current list");
            let current_list_path = get_absolute_path(current_list_name, file_path);

            println!("Please input what you want to do next? For the list of commands type help: ");
            let mut command = input().unwrap();

            command = command.trim().parse().unwrap();

            if command == "exit" || command == "e" || command == "quit" || command == "q" {
                if auto_clean {
                    clean(file_path);
                }
                break
            }

            execute_commands(command.to_string(), todo_list.clone(), file_path, auto_clean, &current_list_path);

        }
    }

}

fn execute_commands(command: String, mut todo_list: Vec<Todo>, file_path : &PathBuf, auto_clean : bool, current_list : &PathBuf) {

    match command.to_lowercase().trim() {
        "list" | "l" => list_tasks(todo_list),
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

                add_to_list(create_task(task.to_lowercase().trim(), importance), todo_list.clone());

                println!("Would you like to add a new task or are you done adding tasks? (add/done): ");
                let input = input().expect("Could not unwrap String");

                if input.trim() == "done" || input.trim() == "d" {
                    write_file(&todo_list, &current_list).expect("Could not parse the file");

                    if auto_clean {
                        clean(current_list);
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

            remove_task(&mut todo_list, task_to_remove.to_lowercase().trim());
            write_file(&todo_list, current_list).unwrap();

        },
        "importance" | "i" => {
            println!("What task would you like to update?");
            let task = input().unwrap();

            println!("What level of importance would you like to change your task to? (1 - 4)");
            let new_importance = read!();

            change_importance(todo_list.clone(), new_importance, task.to_lowercase().trim());
            write_file(&todo_list, current_list).unwrap()
        }
        "status" | "s" => {
            println!("What task do you need to change the status(completion) of?");
            let task = input().unwrap();

            mark_completed(todo_list.clone(), task.to_lowercase().trim());
            write_file(&todo_list, current_list).unwrap();
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

            hide_task(todo_list.clone(), task.to_lowercase().trim());
            write_file(&todo_list, current_list).unwrap()
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

            write_current_list(&list_name.to_string());

        }
        _ => {
               eprintln!("You made an incorrect input! Please try again :)");
        }
    }
}
