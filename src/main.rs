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
    xyz();
    let s = "POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))";
    let g = wkt::Wkt::<f64>::from_str(s).unwrap();
    let result = Geometry::try_from(g.clone());
    //g.contains(&g);
    let line_string = line_string![
        (x: 0., y: 0.),
        (x: 2., y: 0.),
        (x: 2., y: 2.),
        (x: 0., y: 2.),
        (x: 0., y: 0.),
    ];
    let polygon = Polygon::new(line_string.clone(), vec![]);


    println!("{}", g);
    println!("{}", TypeId::of::<Polygon>() == line_string.type_id());
    //let other = &result.unwrap();
    //println!("{}", Contains::contains(other, other));
    println!("{}", Contains::contains(&polygon, &polygon));

    println!("Goodbye, world!");
}

fn fromPipe() -> Vec<std::io::Result<String>> {
    return stdin()
        .lock()
        .lines()
        .collect::<Vec<_>>();

}

fn fromFile(filename: &str) {
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
}

fn xyz() {
    if atty::isnt(atty::Stream::Stdin) {
        stdin()
            .lock()
            .lines()
            .for_each(|line| println!("{}", line.unwrap()));
    }
}
