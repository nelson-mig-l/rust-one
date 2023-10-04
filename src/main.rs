extern crate atty;
extern crate wkt;

mod load;
mod filters;

use std::any::Any;
use std::str::FromStr;
use clap::Parser;
use geo::{Contains, Geometry};
use geo::{line_string, point};
use crate::filters::get_filter;


#[derive(Parser)]
struct Args {
    #[arg(index = 1, default_value = "*")]
    pattern: String,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    let pattern = &args.pattern;
    println!("{}", pattern);
    let filter = get_filter(pattern);

    // //g.contains(&g);
    // let line_string = line_string![
    //     (x: 0., y: 0.),
    //     (x: 2., y: 0.),
    //     (x: 2., y: 2.),
    //     (x: 0., y: 2.),
    //     (x: 0., y: 0.),
    // ];
    // let polygon = Polygon::new(line_string.clone(), vec![]);

    let data = load::get_data();
    for ele in data {
        let g = wkt::Wkt::<f64>::from_str(&ele).unwrap();
        let result = Geometry::try_from(g.clone());
        if filter(result.unwrap()) {
            println!("{}", g);
        }
    }
    // //let other = &result.unwrap();
    // //println!("{}", Contains::contains(other, other));
    // println!("{}", Contains::contains(&polygon, &polygon));
    println!();
    println!("Goodbye, world!");
}
