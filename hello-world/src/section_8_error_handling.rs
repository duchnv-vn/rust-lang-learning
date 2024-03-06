use std::{fs::File, io::Read};

pub fn main() {
    // Unrecoverable errors -> Stop program right away
    // panic!("exit");

    // Result enum
    // enum Result<T, E> {
    //     Ok(T),  // Success process return value
    //     Err(E), // Failed process return error
    // }

    // Option enum
    // enum Option<T> {
    //     None,    // No value | same null
    //     Some(T), // Contain value
    // }

    // Recoverable errors -> No stop program
    // let file_name = "data.txt";
    // let data = File::open(file_name);
    // let mut data_file = match data {
    //     Ok(file) => file,
    //     Err(_) => match File::create(file_name) {
    //         Ok(new_file) => new_file,
    //         Err(error) => panic!("Problem to create file: {:?}", error),
    //     },
    // };

    // let mut buffer = String::new();
    // println!("data_file: {:?}", data_file.read_to_string(&mut buffer));
}
