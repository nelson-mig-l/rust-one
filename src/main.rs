use std::io::{BufRead, stdin};

fn main() {
    println!("Hello, world!");
    stdin()
        .lock()
        .lines()
        .for_each(|line| println!("{}", line.unwrap()));
}
