use std::fs::{File, OpenOptions, remove_file};
use std::{fs, io};
use std::path::PathBuf;
use list::list::Todo;
use std::io::{Read, read_to_string, Write};
use serde_json;
use crate::list_maintenance::print_tasks;

//Reads and returns the list from the file
pub fn read_and_return(path_to_file : &PathBuf) -> Result<Vec<Todo>, io::Error> {
    let file : File;

    file = File::open(&path_to_file)?;

    let tasks : Vec<Todo> = serde_json::from_reader(file)?;

    Ok(tasks)

}

//Writes the new/updated list to a new or existing file
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
    let flag = !auto_clean;

    flag

}

pub fn create_file(path_to_file : &PathBuf, file_name : String) {
    let file_extension = String::from(".json");

    let file = file_name + &file_extension;

    let file_path = path_to_file.join(file);

    fs::write(&file_path, "[]").expect("Could not write to file");

}

//Writes values to a flag file, which allows for user flags to be saved
pub fn write_flag_values(autoclean : bool) -> Result<(), io::Error> {
    let mut file = File::create("flag_values.txt")?;
    file.write_all(autoclean.to_string().as_bytes())?;

    Ok(())

}

pub fn get_absolute_path(file_to_find : String, rest_of_path : &PathBuf) -> PathBuf {
    let absolute_path = rest_of_path.join(file_to_find);

    absolute_path
}

pub fn write_current_list(current_list : &String) {
    let mut file = File::create("current_list.txt").expect("Could not create file");

    let trimmed_list = current_list.trim_end();

    let full_list = trimmed_list.to_owned() + ".json";

    file.write(full_list.as_bytes()).expect("Could not write to file.");
}

pub fn read_current_list(file_path : &PathBuf) -> io::Result<String> {

    let content = fs::read_to_string(file_path)?;

    Ok(content.to_string())

}


