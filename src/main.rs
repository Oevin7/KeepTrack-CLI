use std::{env, fs, io};
use text_io::read;
use list::list::Todo;
use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Output;
use serde::Serialize;

fn main() {
    let mut todo_list : Vec<Todo> = vec![];

    let path = Path::exists("todo_list.json".as_ref());
    let file = "todo_list.json";

    let args : Vec<String> = env::args().collect();

    let command = parse_commands(&args);

    handle_command(command, &mut todo_list, path, file);

}

fn add_to_list(task : Todo, list: &mut Vec<Todo>) {
    list.push(task);
}

fn create_task(task : String, importance : i32) -> Todo {
    let new_task = Todo::new(task, false, importance);
    new_task
}

fn parse_commands(args : &[String]) -> &str {
    let command = &args[1];

    command

}

fn handle_command(command : &str, todo_list: &mut Vec<Todo>, path: bool, file : &str) {

    let mut list = todo_list;

    loop {
        match command {
            "list" => {
                let list = read_file().unwrap();

                for task in 0..list.len() {
                    println!("Task: {}, Completed: {}, Importance: {}",
                             list[task].get_task().replace("\n", ""),
                             list[task].get_status(),
                             list[task].get_importance());
                }

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

                add_to_list(create_task(task, importance), &mut list);

                println!("Would you like to add a new task or exit? (add/exit): ");
                let input = input();

                if input.unwrap().trim() == "exit" {

                    if !path {
                        write_file(list).expect("Failed to write to file");
                        break;
                    }

                } else {
                    update_file(file, list).unwrap();
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
            _ => {
                println!("Nothing here yet! These features will be implemented in the future.");
                break;
            },
        }
    }
}

fn write_file(file : &mut Vec<Todo>) -> Result<(), io::Error> {

    let serialized_data = serde_json::to_string(&file)?;

    let mut open_file = File::create("todo_list.json")?;

    if let Err(e) = writeln!(open_file, "{}", serialized_data) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())

}

fn read_file() -> Result<Vec<Todo>, io::Error> {
    let mut file = File::open("todo_list.json")?;

    let mut contents : String = String::new();
    file.read_to_string(&mut contents)?;

    let todo_list : Vec<Todo> = serde_json::from_str(&contents)?;

    Ok(todo_list)

}

fn update_file(filename : &str, new_data : &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let prev_content = fs::read_to_string(filename)?;

    let mut data = if !prev_content.is_empty() {
        serde_json::from_str(&prev_content)?
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };

    if let Some(obj) = data.as_object_mut() {
        for (key, value) in new_data.as_object().expect("New data must be an object") {
            obj.insert(key.clone(), value.clone());
        }
    } else {
        return Err(Box::new(serde_json::Error::InvalidData("File must contain a JSON object")));
    }

    let output = serde_json::to_string_pretty(&data)?;

    fs::write(filename, output)?;

    Ok(())

}

fn input() -> Option<String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    Some(input)

}