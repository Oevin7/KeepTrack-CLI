use std::fs::{File, OpenOptions, remove_file};
use std::{fs, io};
use std::path::{ PathBuf};
use crate::todo_struct::*;
use std::io::{Write};
use serde_json;

//Reads and returns the list from the file
pub fn read_and_return(path_to_file : &PathBuf) -> Result<Vec<Todo>, io::Error> {
    let file : File;

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    Ok(tasks)

}

//Writes the new/updated list to a new or existing file
pub fn write_file(list : &Vec<Todo>, file_path : &PathBuf) -> Result<(), io::Error> {

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

pub fn delete_file(path_to_file : &PathBuf, file_name : String) {
    let file_extension = String::from(".json");
    let file = file_name + &file_extension;

    let file_to_delete = path_to_file.join(file);

    match remove_file(file_to_delete) {
        Ok(()) => println!("File removed successfully"),
        Err(e) => eprintln!("Couldn't remove the file due to {e}"),
    }

}

pub fn auto_clean_flag(auto_clean: bool) -> bool {
    let mut flag = !auto_clean;

    flag

}

pub fn create_file(path_to_file : &PathBuf, file_name : String) {
    let file_extension = String::from(".json");
    let file = file_name + &file_extension;
    let file_path = path_to_file.join(file);

    if !file_path.exists() {
        let file = OpenOptions::new()
            .create(true)
            .open(&file_path);

        let mut file_contents = match file {
            Ok(content) => content,
            Err(_e) => File::create(&file_path).expect("Could not read file"),
        };

        if let Err(e) = writeln!(file_contents, "[]") {
            eprintln!("Couldn't write to file: {}", e);
        }

    }
}

//Writes values to a flag file, which allows for user flags to be saved
pub fn write_flag_values(autoclean : bool) -> Result<(), io::Error> {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("flag_values.txt");

    writeln!(file?, "{}", autoclean)?;

    println!("Autoclean was set to {}", autoclean);

    Ok(())

}

pub fn write_current_list(current_list : &PathBuf) {
    let mut file = File::create("current_list.txt").expect("Could not create file");

    let trimmed_list = current_list.to_owned();
    let trimmed_list_str = trimmed_list.to_str().unwrap().to_string();

    let full_list = trimmed_list_str;

    file.write(full_list.as_bytes()).expect("Could not write to file.");
}

pub fn read_current_list(file_path : &PathBuf) -> io::Result<PathBuf> {

    let content = fs::read_to_string(file_path)?;

    Ok(content.parse().unwrap())

}

pub fn create_default_file(default_file_path : &PathBuf) {

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(default_file_path)
        .expect("Could not open file");


    if file.metadata().expect("Unable to retrieve metadata").len() == 0 {
        let mut file = OpenOptions::new()
            .write(true)
            .open(default_file_path)
            .expect("Could not open file to write");

        file.write_all(b"[]").expect("Could not write to file");

    }
}
