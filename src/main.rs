extern crate atty;
extern crate wkt;

use std::io::{BufRead, stdin};
use std::str::FromStr;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "*")]
    pattern: String,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    println!("{}", &args.pattern);
    if atty::isnt(atty::Stream::Stdin) {
        stdin()
            .lock()
            .lines()
            .for_each(|line| println!("{}", line.unwrap()));
    }
    let s = "POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))";
    let g = wkt::Wkt::<f64>::from_str(s).unwrap();

    println!("{}", g);

    println!("Goodbye, world!"); 
}
