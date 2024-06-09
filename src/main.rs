use std::{env, io};
use text_io::read;
use list::list::Todo;
use std::fs::File;
use std::io::{ Read, Write};
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    let todo_list : Vec<Todo> = vec![];

    let path = Path::exists("todo_list.json".as_ref());
    let file_path = "todo_list.json";

    let args : Vec<String> = env::args().collect();

    let command = parse_commands(&args);

    handle_command(command, todo_list, file_path, path);

}

//Adds tasks to the list
fn add_to_list(task : Todo, list: &mut Vec<Todo>) {
    list.push(task);
}

//Removes tasks from the list
fn remove_task(task_to_remove : &str, path_to_file : &str, path : bool) {

    let mut list = match read_and_return(path_to_file, path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    if let Some(data) = list.iter().position(|task| task.get_task().trim() == task_to_remove) {
        list.remove(data);
    } else {
        eprintln!("Task was not found in the list.")
    }

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {}. Error: {}", path_to_file, e);
    }

}

//Marks a task as completed
fn mark_completed(task_to_complete : &str, path_to_file : &str, path : bool) {
    let mut list = match read_and_return(path_to_file, path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    for mut task in &mut list {
        if task.get_task().trim() == task_to_complete {
            task.change_status();
        }
    }

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {}. Error: {}", path_to_file, e);
    }
}

//Creates a new task when one is added
fn create_task(task : &str, importance : i32) -> Todo {
    let new_task = Todo::new(task.parse().unwrap(), false, importance);
    new_task
}

//Finds a task in the list. Used to aid the other functions in editing the list
fn find_task(name_of_task : &str, path_to_file : &str, path : bool) -> Option<Todo> {

    let mut task_to_return = None;

    let tasks = match read_and_return(path_to_file, path) {
        Ok(data) => data,
        Err(e) => panic!("Could not read the file due to {}", e),
    };

    for task in tasks {
        if name_of_task == task.get_task().trim() {
            task_to_return = Some(task);
            break;
        }
    }

    task_to_return

}

//Changes the importance of a task
fn change_importance(new_importance : i32, name_of_task : &str, path_to_file : &str, path: bool) {

    let mut list = match read_and_return(path_to_file, path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    for mut task in &mut list {
        if task.get_task().trim() == name_of_task {
            task.change_importance(new_importance);
        }
    }

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {}. Error: {}", path_to_file, e);
    }

}

//Parses the commands to later handle the input
fn parse_commands(args : &[String]) -> &str {
    let command = &args[1];

    command

}

//Handles the commands that were parsed
fn handle_command(command : &str, todo_list: Vec<Todo>, file_path : &str, path : bool) {

    let mut list = todo_list;

    loop {
        match command {
            "list" => {
                list_tasks(file_path, path).expect("Could not get data from the file.");
                break
            }
            "add" => {
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

                add_to_list(create_task(task.trim(), importance), &mut list);

                println!("Would you like to add a new task or exit? (add/exit): ");
                let input = input().expect("Could not unwrap String");

                if input.trim() == "exit" || input.trim() == "e" {
                    write_file(&mut list, file_path).expect("Could not parse the file");
                    break
                }

            }
            "help" => {
                println!("\n\
                \t\tlist: Lists the tasks that are currently on your list. Uncompleted and
                Completed will show up unless you use a filter, or when you exit the program.
                Exiting automatically cleans up completed tasks.

                add: Adds a task to your list. These can later be marked as completed or
                modified to change their importance. You can also filter these tasks later to only
                view the ones you need.

                remove: Removes a task from your list. All tasks marked completed are automatically
                removed when the program exits. However, you can preemptively remove tasks if you'd
                like!

                importance: Allows you to modify the importance of your tasks. You can change their
                importance level from an integer between 1 and 4! This will be helpful when you want
                to filter tasks, but some tasks are no longer as urgent.

                ");

                break

            }
            "remove" => {
                println!("Please input the task you would like to remove: ");
                let task_to_remove = input().expect("Couldn't get user input");

                remove_task(task_to_remove.trim(), file_path, path);
                break

            },
            "importance" => {
                println!("What task would you like to update?");
                let task = input().unwrap();

                println!("What level of importance would you like to change your task to? (1 - 4)");
                let new_importance = read!();

                change_importance(new_importance, task.trim(), file_path, path);
                break
            }
            "status" => {
                println!("What task do you need to change the status(importance) of?");
                let task = input().unwrap();

                mark_completed(task.trim(), file_path, path);
                break
            }
            _ => {
                panic!("NO FEATURES HERE!!!! ABORT, ABORT! TO LAZY TO PROPERLY HANDLE!");
            }
        }
    }
}

//Writes the new/updated list to a new or existing file
fn write_file(list : &Vec<Todo>, file_path : &str) -> Result<(), io::Error> {

    let existing_tasks = list.clone();

    let serialized_data = serde_json::to_string_pretty(&existing_tasks)?;

    let mut open_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    if let Err(e) = writeln!(open_file, "{}", serialized_data) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())

}

//Lists the tasks that are on the list
fn list_tasks(path_to_file : &str, path : bool) -> Result<(), io::Error> {

    let mut file : File;

    if !path {
        panic!("File does not exist.");
    }

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    for task in 0..tasks.len() {
        println!("Task: {}, Completed: {}, Importance: {} \n",
                 tasks[task].get_task().replace("\n", ""),
                 tasks[task].get_status(),
                 tasks[task].get_importance());
    }

    Ok(())

}

//Reads and returns the list from the file
fn read_and_return(path_to_file : &str, path : bool) -> Result<Vec<Todo>, io::Error> {
    let mut file : File;

    if !path {
        panic!("File does not exist.");
    }

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    Ok(tasks)

}

//Accepts user input when the read!() macro won't work
fn input() -> Option<String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    Some(input)

}