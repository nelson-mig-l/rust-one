extern crate atty;

use std::io::{BufRead, stdin};
use std::fs::read_to_string;

pub fn get_data() -> Vec<String> {
    if atty::is(atty::Stream::Stdin) {
        println!("File");
        return from_file("example.txt");
    } else {
        println!("Pipe");
        return from_pipe();
    }
}

fn from_pipe() -> Vec<String> {
    let data = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    return data;

}

fn from_file(filename: &str) -> Vec<String> {
    let data = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();
    return data;
}