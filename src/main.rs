use std::env;
mod common;
mod aoc2020;

use crate::common::Part;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_str() {
        "20day15p1" => aoc2020::day15::main(Part::One),
        "20day15p2" => aoc2020::day15::main(Part::Two),
        "20day16p1" => aoc2020::day16::main(Part::One),
        "20day16p2" => aoc2020::day16::main(Part::Two),
        "20day17p1" => aoc2020::day17::main(Part::One),
        "20day17p2" => aoc2020::day17::main(Part::Two),
        _         => println!("Non-existent day.")
    }
}
