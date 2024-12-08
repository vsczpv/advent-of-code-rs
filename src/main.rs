/* SPDX-License-Identifier: 0BSD */

use std::env;
mod common;
mod aoc2020;
mod aoc2024;

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
        "20day18p1" => aoc2020::day18::main(Part::One),
        "20day18p2" => aoc2020::day18::main(Part::Two),
        "24day01p1" => aoc2024::day01::main(Part::One),
        "24day01p2" => aoc2024::day01::main(Part::Two),
        "24day02p1" => aoc2024::day02::main(Part::One),
        "24day02p2" => aoc2024::day02::main(Part::Two),
        "24day03p1" => aoc2024::day03::main(Part::One),
        "24day03p2" => aoc2024::day03::main(Part::Two),
        "24day04p1" => aoc2024::day04::main(Part::One),
        "24day04p2" => aoc2024::day04::main(Part::Two),
        "24day05p1" => aoc2024::day05::main(Part::One),
        "24day05p2" => aoc2024::day05::main(Part::Two),
        "24day06p1" => aoc2024::day06::main(Part::One),
        "24day06p2" => aoc2024::day06::main(Part::Two),
        "24day07p1" => aoc2024::day07::main(Part::One),
        "24day07p2" => aoc2024::day07::main(Part::Two),
        "24day08p1" => aoc2024::day08::main(Part::One),
        "24day08p2" => aoc2024::day08::main(Part::Two),
        _         => println!("Non-existent day.")
    }
}
