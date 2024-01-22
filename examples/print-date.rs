extern crate langtime;

use std::env;
use langtime::parse;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if let None = args.first() {
        println!("You need to pass the input as an argument.");
        return;
    }

    match parse(args.first().unwrap()) {
        Ok(dt) => println!("{:?}", dt),
        Err(_) => println!("Cannot parse input as a date")
    }
}
