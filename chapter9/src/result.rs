use std::fs::File;
use std::io::ErrorKind;

pub fn intro() {
    let file_result = File::open("file.txt");
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("shit happened: {:#?}",error),
    };
}

pub fn create() {
    let file = File::open("file.txt");
    let _file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(cf) => cf,
                Err(er) => panic!("no hope: {:#?}",er),
            }
        _ => panic!("shit happened: {:#?}", error),
        }
    };
}