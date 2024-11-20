use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_input_lines(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    lines
}

pub fn split_once<'a>(in_string: &'a str, separator: &'a str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, separator);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();

    (first, second)
}
