use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::path::PathBuf;
use list::list::Todo;
use std::io::Write;

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

pub fn delete_file(path_to_file : &PathBuf) -> Result<(), io::Error> {
    remove_file(path_to_file)?;

    Ok(())

}