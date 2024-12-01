/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i24day01p1.txt").expect("Error: inputs/i24day01p1.txt missing!");
	let file_lines = file.lines();

	let mut llist = Vec::new();
	let mut rlist = Vec::new();

	for line in file_lines {
		let mut line_split = line.split("   ");
		llist.push(line_split.next().expect("number"));
		rlist.push(line_split.next().expect("number"));
		assert_eq!(line_split.count(), 0, "Input must only have two lists.");
	}

	llist.sort();
	rlist.sort();

	if part == Part::One {

		let mut sum = 0i32;
		for pairs in llist.iter().zip(rlist) {
			let res: i32 = pairs.0.parse::<i32>().expect("number") - pairs.1.parse::<i32>().expect("number");
			sum += res.abs();
		}

		println!("result is {sum}");

		return;
	}

	let mut sum = 0;
	for lnum in &llist {
		let ilnum = lnum.parse::<i32>().expect("number");
		let mut count = 0;
		for rnum in &rlist {
			if lnum == rnum {
				count += 1;
			}
		}
		sum += ilnum * count;
	}

	println!("result is {sum}")
}

