use std::io::ErrorKind;
use std::io::{Error, Read};
use std::fs::File;

pub fn error_handle_demo() {
    let filename = "./test_file_dne.json";

    let file_handle = File::open(filename);

    match file_handle {
         Ok(file) => {
              println!("{:#?}", file);
         }
         Err(error) => {
              // panic!("panic message");
              println!("{:#?}", error);
              match error.kind() {
                   ErrorKind::NotFound => {
                        match File::create(filename) {
                             Ok(file) => {
                                  println!("File Created");
                             }
                             Err(error) => {
                                  println!("{:#?}", error);
                             }
                        }
                   }
                   // match any other error type
                   _ => {
                        println!("{:#?}", error);
                   }
              }

         }
    }
}


pub fn error_propogate_demo() {

    let filename = "./test_file_dne.json";
    let file_data = read_file(filename);
    match file_data {
         Ok(data) => {
              println!("{}", data);
         }
         Err(error) => {
              println!("Error down in stack");
              println!("{}", error);
         }
    }

    fn read_file(filename: &str) -> Result<String, Error> {
         // ? at the end is what does the magic here of passing error up stack
         let mut file_handle = File::open(filename)?;
         let mut file_data = String::new();
         file_handle.read_to_string(&mut file_data)?;
         Ok(file_data)
    }
}