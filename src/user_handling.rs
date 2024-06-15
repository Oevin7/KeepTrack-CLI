use std::fs::OpenOptions;
use std::io;
use std::io::Read;

//Accepts user input when the read!() macro won't work
pub fn input() -> Option<String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    Some(input)

}

pub fn read_flag_values() -> Result<bool, io::Error> {
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