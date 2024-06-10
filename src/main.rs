use std::{env, io};
use text_io::read;
use list::list::Todo;
use std::fs::File;
use std::io::{ Read, Write};
use std::fs::OpenOptions;
use std::path::Path;
use colored::Colorize;
use serde_json::to_writer;

fn main() {
    let mut todo_list : Vec<Todo> = vec![];

    let auto_clean : bool = read_flag_values().unwrap();

    let path = Path::exists("todo_list.json".as_ref());
    let file_path = "todo_list.json";

    let args : Vec<String> = env::args().collect();

    let command = parse_commands(&args);

    handle_command(&mut todo_list, file_path, path, auto_clean, command);

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

    for task in &mut list {
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

fn filter_tasks_by_importance(importance : i32, path_to_file : &str, path : bool) {
    let mut list = match read_and_return(path_to_file, path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    let mut filtered_list= vec![];

    for task in &mut list {
        if task.get_importance() == importance {
            filtered_list.push(task)
        }
    }

    for data in 0..filtered_list.len() {
        println!("Task: {}, Completed: {}, Importance: {} \n",
                 filtered_list[data].get_task().replace("\n", ""),
                 filtered_list[data].get_status(),
                 filtered_list[data].get_importance());
    }

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

    for task in &mut list {
        if task.get_task().trim() == name_of_task {
            task.change_importance(new_importance);
        }
    }

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {}. Error: {}", path_to_file, e);
    }

}

//Parses the commands to later handle the input
fn parse_commands(args : &[String]) -> Option<String> {
    let command = args.get(1).unwrap_or(&String::from("s")).clone();

    Some(command)

}

//Handles the commands that were parsed
fn handle_command(todo_list : &mut Vec<Todo>, file_path : &str, path : bool, auto_clean : bool, intro_command : Option<String>) {

    if !path {
        let file = File::create(file_path).expect("Could not create the file.");
        let empty_vec : Vec<Todo> = Vec::new();

        if let Err(e) = to_writer(file, &empty_vec) {
            eprintln!("Failed to write to file {}: {}", file_path, e);
        }

    }

    match intro_command {
        Some(command) => execute_commands(command, todo_list, file_path, path, auto_clean),
        None => ()
    }

    loop {

        println!("Please input what you want to do next? For the list of commands type help.");
        let mut command = input().unwrap();

        command = command.trim().parse().unwrap();

        if command == "exit" || command == "e" || command == "quit" || command == "q" {
            if auto_clean {
                clean(file_path, path);
            }
            break
        }

        execute_commands(command, todo_list, file_path, path, auto_clean);

    }

}

fn execute_commands(command: String, todo_list: &mut Vec<Todo>, file_path : &str, path : bool, auto_clean : bool) {

    let mut list = todo_list;

    match command.to_lowercase().trim() {
        "list" | "l" => {
            list_tasks(file_path, path).expect("Could not get data from the file.");
        }
        "add" | "a" => {
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

            add_to_list(create_task(task.to_lowercase().trim(), importance), &mut list);

            println!("Would you like to add a new task or are you done adding tasks? (add/done): ");
            let input = input().expect("Could not unwrap String");

            if input.trim() == "done" || input.trim() == "d" {
                write_file(&mut list, file_path).expect("Could not parse the file");

                if auto_clean {
                    clean(file_path, path);
                }

            }

        }
        "help" | "h" => {

            let list = "list".bright_cyan();
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
completed tasks.", list, add, remove, importance, status ,clean, autoclean, exit);

        }
        "remove" | "r" => {
            println!("Please input the task you would like to remove: ");
            let task_to_remove = input().expect("Couldn't get user input");

            remove_task(task_to_remove.trim(), file_path, path);

        },
        "importance" | "i" => {
            println!("What task would you like to update?");
            let task = input().unwrap();

            println!("What level of importance would you like to change your task to? (1 - 4)");
            let new_importance = read!();

            change_importance(new_importance, task.trim(), file_path, path);
        }
        "status" | "s" => {
            println!("What task do you need to change the status(completion) of?");
            let task = input().unwrap();

            mark_completed(task.trim(), file_path, path);
        }
        "clean" | "c" => {
            clean(file_path, path);
        }
        "auto_clean" | "ac" => {
            write_flag_values(auto_clean_flag(auto_clean)).expect("Unable to set the flags. \
                Likely a file error");
        }
        _ => {
                panic!("NO FEATURES HERE!!!! ABORT, ABORT! TO LAZY TO PROPERLY HANDLE!");
        }
    }
}

//Writes the new/updated list to a new or existing file
fn write_file(list : &Vec<Todo>, file_path : &str) -> Result<(), io::Error> {

    let existing_tasks = list;

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

    let file : File;

    file = File::open(&path_to_file)?;

    let tasks: Vec<Todo> = serde_json::from_reader::<_, Vec<Todo>>(file).unwrap_or_else(|e| vec![]);

    for task in 0..tasks.len() {
        println!("Task: {}, Completed: {}, Importance: {} \n",
                 tasks[task].get_task().replace("\n", ""),
                 tasks[task].get_status(),
                 tasks[task].get_importance());
    }

    Ok(())

}

//Cleans up and removes all completed tasks
fn clean(path_to_file : &str, path: bool) {
    let mut list = match read_and_return(path_to_file, path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not read the file due to {}", e);
            return
        }
    };

    list.retain(|task| !task.get_status());

    if let Err(e) = write_file(&mut list, path_to_file) {
        eprintln!("Could not write to file: {}. Error {}", path_to_file, e);
    }
}

fn auto_clean_flag(auto_clean: bool) -> bool {
    let flag = !auto_clean;

    flag

}

//Reads and returns the list from the file
fn read_and_return(path_to_file : &str, path : bool) -> Result<Vec<Todo>, io::Error> {
    let file : File;

    if !path {
        panic!("File does not exist.");
    }

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    Ok(tasks)

}

//Writes values to a flag file, which allows for user flags to be saved
fn write_flag_values(autoclean : bool) -> Result<(), io::Error> {
    let mut file = File::create("flag_values.txt")?;
    file.write_all(autoclean.to_string().as_bytes())?;

    Ok(())

}

fn read_flag_values() -> Result<bool, io::Error> {
    let path = "flag_values.txt";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path);

    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let autoclean = contents.trim().parse().unwrap_or(false);
            Ok(autoclean)
        },
        Err(e) => {
            eprintln!("An error occurred while opening the file: {:?}", e);
            Err(e)
        }
    }

}

//Accepts user input when the read!() macro won't work
fn input() -> Option<String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    Some(input)

}