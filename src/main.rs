extern crate atty;
extern crate wkt;

use std::any::{Any, TypeId};
use std::io::{BufRead, Lines, stdin, StdinLock};
use std::str::FromStr;
use clap::Parser;
use geo::{Contains, Geometry};
use geo::{line_string, point, Polygon};
use std::fs::read_to_string;
use std::iter::Map;
use std::vec::IntoIter;
use wkt::Wkt;
use core::result::Iter;


#[derive(Parser)]
struct Args {
    #[arg(index = 1, default_value = "*")]
    pattern: String,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    println!("{}", &args.pattern);
    // let s = "POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))";
    // let g = wkt::Wkt::<f64>::from_str(s).unwrap();
    // let result = Geometry::try_from(g.clone());
    // //g.contains(&g);
    // let line_string = line_string![
    //     (x: 0., y: 0.),
    //     (x: 2., y: 0.),
    //     (x: 2., y: 2.),
    //     (x: 0., y: 2.),
    //     (x: 0., y: 0.),
    // ];
    // let polygon = Polygon::new(line_string.clone(), vec![]);

    let data = get_data();
    for ele in data {
        let g = wkt::Wkt::<f64>::from_str(&ele).unwrap();
        let result = Geometry::try_from(g.clone());
        print!("{}", g);
    }
    // println!("{}", g);
    // println!("{}", TypeId::of::<Polygon>() == line_string.type_id());
    // //let other = &result.unwrap();
    // //println!("{}", Contains::contains(other, other));
    // println!("{}", Contains::contains(&polygon, &polygon));
    println!();
    println!("Goodbye, world!");
}

fn get_data() -> Vec<String> {
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
