extern crate atty;
extern crate wkt;

mod load;

use std::any::{Any, TypeId};
use std::str::FromStr;
use clap::Parser;
use geo::{Contains, Geometry};
use geo::{line_string, point, Polygon};
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
