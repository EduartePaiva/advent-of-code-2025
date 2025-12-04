pub mod questions;

use std::{fs::File, io::Read};

pub fn read_question_file(question_number: u16) -> String {
    let mut input: String = String::new();

    let path = "./src/inputs/q".to_string() + question_number.to_string().as_str() + ".txt";

    File::open(path)
        .expect("error opening the file")
        .read_to_string(&mut input)
        .expect("Some error converting to string");
    input
}
