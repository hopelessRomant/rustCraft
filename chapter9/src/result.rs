use std::fs::File;

pub fn intro() {
    let file_result = File::open("file.txt");
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("shit happened: {:#?}",error),
    };
}