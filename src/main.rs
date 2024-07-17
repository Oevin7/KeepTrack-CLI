mod user_handling;
mod file_management;
mod list_maintenance;
mod list_features;
mod todo_struct;
mod test;

use std::fs::File;
use crate::list_features::*;
use crate::user_handling::input;

fn main() {

    let task_directory = create_task_directory();
    let current_list_path = prepare_current_list(&task_directory);

    loop {
        if !current_list_path.exists() {
            match File::create(&current_list_path) {
                Ok(file) => {
                    println!("The file current_list.txt is successfully created at {:?}", file);
                },
                Err(e) => {
                    eprintln!("Failed to create file: {:?}", e);
                },
            }
        } else {
            break
        }
    }

    run_cli();

}

