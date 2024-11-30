use std::env;
mod common;
mod day15;

use crate::common::Part;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_str() {
        "day15p1" => { day15::main(Part::One); }
        "day15p2" => { day15::main(Part::Two); }
        _         => { println!("Non-existent day."); }
    }
}
