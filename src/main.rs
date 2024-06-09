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

fn add_to_list(task : Todo, list: &mut Vec<Todo>) {
    list.push(task);
}

fn create_task(task : &str, importance : i32) -> Todo {
    let new_task = Todo::new(task.parse().unwrap(), false, importance);
    new_task
}

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

fn parse_commands(args : &[String]) -> &str {
    let command = &args[1];

    command

}

fn handle_command(command : &str, todo_list: Vec<Todo>, file_path : &str, path : bool) {

    let mut list = todo_list;

    loop {
        match command {
            "list" => {
                let list_ref = &list;
                list_tasks(file_path, path).expect("Could not get data from the file.");
                break;
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
                    write_file(&mut list, file_path, path).expect("Could not parse the file");
                    break;
                }

            }
            "help" => {
                println!("\
                \t\tlist: Lists the tasks that are currently on your list. Uncompleted and
                Completed will show up unless you use a filter, or when you exit the program.
                Exiting automatically cleans up completed tasks.

                add: Adds a task to your list. These can later be marked as completed or
                modified to change their importance. You can also filter these tasks later to only
                view the ones you need.

                ");

                break;

            }
            "find" => {
                loop {
                    println!("What task would you like to find?");
                    let task_to_find = input().unwrap();

                    let task = find_task(task_to_find.trim(), file_path, path);

                    if task.is_none() {
                        println!("That task was not found in the list. Please try again!");
                    } else {
                        let data_in_task = task.unwrap();
                        println!("{:?}", data_in_task.get_task().trim());
                        break
                    }
                    break
                }
            },
            _ => {
                panic!("NO FEATURES HERE!!!! ABORT, ABORT! TO LAZY TO PROPERLY HANDLE!");
            }
        }
    }
}

fn write_file(list : &mut Vec<Todo>, file_path : &str, path : bool) -> Result<(), io::Error> {

    let mut data = String::new();
    let mut existing_tasks : Vec<Todo> = Vec::new();
    
    if path {
        let mut open_file = File::open(file_path)?;
        open_file.read_to_string(&mut data)?;

        match serde_json::from_str(&data) {
            Ok(tasks) => existing_tasks = tasks,
            Err(e) => {
                if e.classify() != serde_json::error::Category::Eof {
                    return Err(e.into());
                }
            }
        }
    }

    existing_tasks.append(list);

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

fn read_and_return(path_to_file : &str, path : bool) -> Result<Vec<Todo>, io::Error> {
    let mut file : File;

    if !path {
        panic!("File does not exist.");
    }

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    Ok(tasks)

}

fn input() -> Option<String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    Some(input)

}