use std::{fs::File, io::Read};

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

pub fn main() {
    // Unrecoverable errors -> Stop program right away
    // panic!("exit");

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

    // Unwrap and expect
    // fn get_user_name(username: &str) -> Option<&str> {
    //     if username.is_empty() {
    //         return None;
    //     }

    //     Some(username)
    // }

    // let unwrap_case = get_user_name("user 1").unwrap(); // stop program right away if error
    // let expect_case = get_user_name("user 2").expect("Not be empty"); // stop program right away if error

    // println!("unwrap_case: {}", unwrap_case);
    // println!("expect_case: {}", expect_case);

    // #[derive(Debug)]
    // enum CustomFileError {
    //     FileOpenError,
    //     FileReadError,
    // }
    // fn read_file_contents(path: &str) -> Result<String, CustomFileError> {
    //     let mut file = File::open(path).map_err(|_| CustomFileError::FileOpenError)?;
    //     let mut content = String::new();

    //     file.read_to_string(&mut content)
    //         .map_err(|_| CustomFileError::FileReadError)?;

    //     Ok(content)
    // }
    // let file_content = read_file_contents("data.csv");
    // println!("file_content: {:?}", file_content.unwrap());
}
