/* SPDX-License-Identifer: 0BSD */

use crate::common::Part;

use regex::Regex;

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i24day03p1.txt").unwrap();

	let mulrgx = match part {
		Part::One => Regex::new(r"mul\(\d\d?\d?,\d\d?\d?\)").unwrap(),
		Part::Two => Regex::new(r"mul\(\d\d?\d?,\d\d?\d?\)|do\(\)|don\'t\(\)").unwrap()
	};

	let mut sum     = 0;
	let mut enabled = true;
	for mtch in mulrgx.find_iter(file.as_str()) {

		match mtch.as_str() {
			"do()"    => { enabled = true;  continue; }
			"don't()" => { enabled = false; continue; }
			_         => {}
		}

		if !enabled { continue; }

		let txt =
			mtch
				.as_str()
				.to_string()
				.chars()
				.skip(4)
				.filter(|x| { *x != ')' })
				.collect::<String>();

		let mut split = txt.split(",");

		let x: i32 = split.next().unwrap().parse().unwrap();
		let y: i32 = split.next().unwrap().parse().unwrap();

		sum += x * y;
	}

	println!("final result is {sum}");
}
