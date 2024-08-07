use std::{env, fs};
use std::fs::{create_dir_all, File};
use std::io::{Write};
use std::path::{Path, PathBuf};
use colored::Colorize;
use crate::todo_struct::*;
use serde_json::to_writer;
use text_io::read;
use crate::file_management::*;
use crate::list_maintenance::*;
use crate::user_handling::{input, read_flag_values, split_input};

pub fn add_task_command(todo_list: &mut Vec<Todo>, current_list : &PathBuf, auto_clean : bool) {
    loop {
        println!("What task would you like to add?");

        println!("Please input the task: ");
        let task = input().unwrap().trim_end().to_string();

        let mut importance = 0;

        loop {

            println!("{}", "How important is the task? (1 lowest level of importance, 4 is the highest.)".trim());

            let user_in = input().unwrap().to_lowercase();

            match user_in.trim().parse::<i32>() {
                Ok(n) if (1..=4).contains(&n) => {
                    importance = n;
                    break
                }
                _ => {
                    eprintln!("Please input a number between 1 and 4.");
                    continue
                }
            }
        }

        let return_list = add_to_list(create_task(task.to_lowercase().trim(), importance), todo_list);

        println!("Would you like to add a new task or are you done adding tasks? (add/done): ");
        let input = input().expect("Could not unwrap String");

        if input.trim() == "done" || input.trim() == "d" {
            match write_file(&return_list, current_list) {
                Ok(_) => println!("File updated successfully!"),
                Err(e) => eprintln!("Error writing to file: {:?}", e),
            }

            if auto_clean {
                clean(current_list);
            }
            break

        }
    }
}

pub fn help_command() {
    let list = "list | l".bright_cyan();
    let list_hidden = "list -h | lh".bright_cyan();
    let add = "add | a".bright_cyan();
    let remove = "remove | r".bright_cyan();
    let importance = "importance | i".bright_cyan();
    let status = "status | s".bright_cyan();
    let filter = "filter".bright_cyan();
    let clean = "clean | c".bright_cyan();
    let create = "create | cr".bright_cyan();
    let delete = "delete | d".bright_cyan();
    let change = "change | ch".bright_cyan();
    let autoclean = "auto_clean | ac".bright_cyan();
    let add_tags = "tags -a | at".bright_cyan();
    let remove_tags = "tags -r | rt".bright_cyan();
    let exit = "exit | e".bright_cyan();

    println!("* {}: Lists the tasks that are currently on your list. Uncompleted and
Completed will show up unless you use a filter, or when you exit the program.
Exiting automatically cleans up completed tasks if auto_clean is set to true.

Running list -all shows all your lists.

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

* {}: a command that allows you to filter your tasks.
Running filter -fi allows you to filter by importance.
While running filter -s allows you to filter by completion status.
Running filter -t allows you to filter by tags. This shows all tasks that have those tags.

* {}: Cleans up your completed tasks. If auto_clean is set to true, the program
will clean the completed tasks when the program exits. To set auto_clean, just run
todo auto_clean.

* {}: Sets auto_clean to true; run it again, it gets set to false. This
automatically deletes tasks marked as complete once the file exits.

* {}: Creates a new list. Just run create and then input a new list name.

* {}: Deletes a list. Does the opposite of create. Same rules apply. Just enter a name and
the program handles the rest!

* {}: Changes your current list. Running changes allows you to switch your current list so you can
use a different one.

* {}: Allows you to add tags to the task you select.

* {}: Allows you to remove tags from the task you select.

* {}: Exits the program. If auto_clean is enabled, it will automatically delete
completed tasks.", list, list_hidden, add, remove, importance, status, filter ,clean, autoclean,
            add_tags, remove_tags ,create, delete, change, exit);

}

pub fn remove_task_command(mut todo_list: Vec<Todo>, current_list : &PathBuf) {

    println!("Please input the task you would like to remove: ");
    let task_to_remove = input().expect("Couldn't get user input").to_lowercase();

    let index = match_task_or_tag(&todo_list, task_to_remove.trim());

    match index {
        Ok(index) => remove_task(&mut todo_list, index),
        Err(_) => {
            eprintln!("Task not found. Please try again.");
            return
        }
    }

    match write_file(&todo_list, current_list) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Could not write to file: {e:?}");
            return;
        }
    }
}

pub fn change_importance_command(mut todo_list: Vec<Todo>, current_list : &PathBuf) {
    println!("What task would you like to update?");
    let task = input().unwrap().to_lowercase();

    println!("What level of importance would you like to change your task to? (1 - 4)");
    let new_importance = read!();

    let index = match_task_or_tag(&todo_list, task.trim());

    match index {
        Ok(index) => change_importance(&mut todo_list, new_importance, index),
        Err(_) => {
            eprintln!("Task not found. Please try again.");
            return
        }
    }

    match write_file(&todo_list, current_list) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Could not write to file: {e:?}");
            return;
        }
    }
}

pub fn change_status_command(mut todo_list: &mut Vec<Todo>, current_list : &PathBuf) {
    println!("What task do you need to change the status(completion) of?");
    let task = input().unwrap().to_lowercase();

    let index = match_task_or_tag(&todo_list, task.trim());

    match index {
        Ok(index) => mark_completed(&mut todo_list, index),
        Err(_) => {
            eprintln!("Task not found. Please try again.");
            return
        }
    }

    match write_file(&todo_list, current_list) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Could not write to file: {e:?}");
            return;
        }
    }
}
pub fn filter_importance_command(todo_list : Vec<Todo>) {
    println!("Please input an integer between 1-4");
    let importance : i32 = read!();

    filter_tasks_by_importance(todo_list, importance);
}

pub fn filter_by_tags(todo_list : Vec<Todo>) {
    println!("Enter tags for the task you'd like to find (space seperated):");

    let tags = match input() {
        Some(tag) => tag.to_lowercase(),
        None => {
            eprintln!("Could not get input, please try again.");
            return;
        }
    };

    let tags_list = split_input(&tags);
    let mut matching_tasks = Vec::new();

    for tags in 0..todo_list.len() {
        match todo_list[tags].get_tag_list().try_borrow() {
            Ok(task_tags) => {
                tags_list.iter().map(|tag| {
                    if task_tags.contains(&tag.to_string()) {
                        matching_tasks.push(&todo_list[tags]);
                    }
                }).collect()
            }
            Err(e) => {
                eprintln!("Could not get the tags list: {e:?}");
                return;
            }
        };
    }

    println!("Matching tasks:");
    for task in matching_tasks {
        println!("{}", task);
    }

}

fn hide_task_command(mut todo_list: Vec<Todo>, current_list : &PathBuf) {
    println!("Which task would you like to hide?");
    let task = input().unwrap();

    if let Some(task) = find_task_by_partial_name(&todo_list, task.trim()) {
        hide_task(&mut todo_list, task);
    } else {
        println!("Task not found. Please try again.");
    }

    match write_file(&todo_list, current_list) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Could not write to file: {e:?}");
            return;
        }
    }
}

fn add_tags_to_task(todo_list : Vec<Todo>, current_list : &PathBuf) {
    println!("Which task would you like to add tags to?");
    let task = match input() {
        Some(task) => task.trim().to_lowercase(),
        None => {
            eprintln!("Error receiving user input. Please try again");
            return;
        }
    };

    let matched_task = match_task_or_tag(&todo_list, &task);

    match matched_task {
        Ok(task) => {
            println!("Enter tags (space seperated):");

            let tags = match input() {
                Some(tag) => tag.to_lowercase(),
                None => {
                    eprintln!("Could not get input, please try again.");
                    return;
                }
            };

            let tags_list = split_input(&tags);

            for tag in tags_list {
                add_tag(&todo_list, task, tag);
            }

            match write_file(&todo_list, current_list) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Could not write to file: {e:?}");
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Could not find that task in the list: {e:?}");
            return;
        }
    }

}
fn add_tag(todo_list : &Vec<Todo>, task : usize, tag : &str) {
    let tags = &mut todo_list[task].get_tag_list();

    tags.borrow_mut().push(tag.to_string());
}

fn remove_tag_from_task(todo_list : Vec<Todo>, current_list : &PathBuf) {
    println!("Which task would you like to remove tags from?");
    let task = match input() {
        Some(task) => task.trim().to_lowercase(),
        None => {
            eprintln!("Error receiving user input. Please try again");
            return;
        }
    };

    let matched_task = match_task_or_tag(&todo_list, &task);

    match matched_task {
        Ok(task) => {
            println!("Enter tags to remove (space seperated):");

            let tags = match input() {
                Some(tag) => tag,
                None => {
                    eprintln!("Could not get input, please try again.");
                    return;
                }
            };

            let tags_list = split_input(&tags);

            let mut task_tags = todo_list[task].get_tag_list().try_borrow_mut();

            match task_tags {
                Ok(mut task_tags) => {
                    for tags in 0..tags_list.len() {
                        if task_tags.contains(&tags_list[tags].to_string()) {
                            task_tags.remove(tags);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Could not mutably borrow task_tags: {e:?}");
                    return;
                }
            }

            match write_file(&todo_list, current_list) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Could not write to file: {e:?}");
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Could not find a matching task: {e:?}");
            return;
        }
    }
}

pub fn delete_file_command(file_path : &PathBuf, current_list : &PathBuf) {

    println!("Please input the list you'd like to delete (Leave blank to delete the default list): ");
    let list_name = input().unwrap().to_lowercase();

    delete_file(file_path, list_name.trim().to_string(), current_list);
}

pub fn create_file_command(file_path : &PathBuf) {

    let mut list_name= String::from("");

    loop {
        println!("Please name your new list (Leave blank for default list): ");
        list_name = input().expect("Could not get input").trim_end().to_lowercase().parse().unwrap();

        if list_name == "" {
            eprintln!("Please enter a proper name for your list.")
        } else {
            break
        }
    }

    create_file(file_path, list_name.trim().to_string());
}

pub fn change_file_command(file_path : &PathBuf) {

    println!("Which list would you like to use instead: ");
    let list_name = input().unwrap().to_lowercase();

    let change_list = match get_list_from_name(&list_name, file_path) {
        Some(file) => file,
        None => {
            eprintln!("Could not find the list. Please try again.");
            return
        }
    };

    write_current_list(&change_list);

}

pub fn create_task_directory() -> PathBuf {
    let home_dir = dir::home_dir().unwrap();

    let task_directory = home_dir.join(".keeptrack-cli").join("lists");

    create_task_directory_if_needed(&task_directory);

    task_directory

}

fn create_task_directory_if_needed(task_directory: &Path) {
    if task_directory.exists() {
        return;
    }

    match create_dir_all(task_directory) {
        Ok(_) => println!("Directory created successfully!"),
        Err(e) => eprintln!("Problem creating directory: {:?}", e),
    }
}

pub fn prepare_current_list(task_directory : &PathBuf) -> PathBuf {
    let current_list_file = PathBuf::from("./current_list.txt");

    initialize_default_list_if_needed(task_directory, &current_list_file);

    read_current_list(&current_list_file).expect("Could not find the file")

}

fn initialize_default_list_if_needed(task_directory: &Path, current_list_file: &Path) {
    let default_list = task_directory.join("todo_list.json");
    create_default_file(&default_list);

    if current_list_file.exists() {
        return;
    }

    write_current_list(&default_list);
}


pub fn run_cli() {
    let home_dir = dir::home_dir().unwrap();
    let full_dir = home_dir.join(".keeptrack-cli").join("lists");

    let file_path = &full_dir.clone();

    let args : Vec<String> = env::args().collect();
    let command = parse_commands(&args);

    handle_command(file_path, command);

}

//Handles the commands that were parsed
fn handle_command(file_path : &PathBuf, mut intro_command: Option<String>) {

    print_logo();

    let current_list_file = PathBuf::from("./current_list.txt");
    let current_list = read_current_list(&current_list_file).expect("Couldn't read file");

    let path = current_list.exists();

    let todo_list : Vec<Todo> = read_and_return(&current_list).unwrap();
    let auto_clean : bool = read_flag_values().unwrap();

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
        Some(command) => execute_commands(command, todo_list, file_path, auto_clean, &current_list),
        None => loop {

            let current_list = read_current_list(&current_list_file).expect("Couldn't read file");
            let todo_list : Vec<Todo> = read_and_return(&current_list).unwrap();
            let auto_clean : bool = read_flag_values().unwrap();

            println!("Please input what you want to do next? For the list of commands type help: ");
            let mut command = input().unwrap();

            command = command.trim().parse().unwrap();

            if command == "exit" || command == "ex" || command == "quit" || command == "q" {
                if auto_clean {
                    clean(&current_list);
                }
                break
            }

            execute_commands(command.to_string(), todo_list.clone(), file_path, auto_clean, &current_list);

        }
    }
}

fn execute_commands(command: String, mut todo_list: Vec<Todo>, file_path : &PathBuf, auto_clean : bool, current_list : &PathBuf) {

    match command.to_lowercase().trim() {
        "list" | "l" => list_tasks(todo_list),
        "list -h" => list_hidden(todo_list),
        "list -all" => list_all(file_path),
        "list -t" | "lt" => list_tags(todo_list),
        "add" | "a" => add_task_command(&mut todo_list, current_list, auto_clean),
        "help" | "h" => help_command(),
        "remove" | "r" => remove_task_command(todo_list, current_list),
        "importance" | "i" => change_importance_command(todo_list, current_list),
        "status" | "s" => change_status_command(&mut todo_list, current_list),
        "clean" | "c" => clean(current_list),
        "auto_clean" | "ac" => write_flag_values(auto_clean_flag(auto_clean)).expect
        ("Unable to set the flags. \
                Likely a file error"),
        "filter -fi" | "fi" => filter_importance_command(todo_list),
        "filter -s" | "fs" => filter_tasks_by_status(todo_list),
        "filter -t" | "ft" => filter_by_tags(todo_list),
        "hide" | "hd" => hide_task_command(todo_list, current_list),
        "delete" | "d" => delete_file_command(file_path, current_list),
        "create" | "cr" => create_file_command(file_path),
        "change" | "ch" => change_file_command(file_path),
        "tags -a" | "at" => add_tags_to_task(todo_list, current_list),
        "tags -r" | "rt" => remove_tag_from_task(todo_list, current_list),
        _ => {
            eprintln!("You made an incorrect input! Please try again :)");
        }
    }
}

pub fn get_list_from_name(list_name : &String, directory : &PathBuf) -> Option<PathBuf> {
    let entries = fs::read_dir(directory).expect("Could not read directory.");
    let file_name = list_name.to_lowercase().trim_end().to_owned() + ".json";

    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();

        match path.file_name() {
            Some(name) if name.to_str().map(|s| s.to_lowercase()) == Some(file_name.clone())
                => {
                return Some(path)
            },
            _ => continue,
        }
    }

    None

}

fn print_logo() {
    println!("

.--------------------------------------------------------------------------.
|                                                                          |
| _  __                 _____               _               ____ _     ___ |
|| |/ /___  ___ _ __   |_   _| __ __ _  ___| | __          / ___| |   |_ _||
|| ' // _ \\/ _ \\ '_ \\    | || '__/ _` |/ __| |/ /  _____  | |   | |    | | |
|| . \\  __/  __/ |_) |   | || | | (_| | (__|   <  |_____| | |___| |___ | | |
||_|\\_\\___|\\___| .__/    |_||_|  \\__,_|\\___|_|\\_\\          \\____|_____|___||
|              |_|                                                         |
|                                                                          |
'--------------------------------------------------------------------------'

");
}