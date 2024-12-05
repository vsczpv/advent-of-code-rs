/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;
use crate::common::section_file;

use std::collections::HashMap;
use std::collections::LinkedList;

pub fn main(_part: Part) {

	let file               = std::fs::read_to_string("inputs/i24day05p1.txt").unwrap();
	let (first_s, secnd_s) = parse_input(file);

	let mut rules = HashMap::<i32, LinkedList<i32>>::new();

	let first_pl: Vec<_> = first_s.iter().map(|x| { x.split("|") }).collect();

	for mut rule in first_pl {
		let left  = rule.next().unwrap().parse::<i32>().unwrap();
		let right = rule.next().unwrap().parse::<i32>().unwrap();
		if !rules.contains_key(&left) {
			rules.insert(left, LinkedList::new());
		}
		rules.get_mut(&left).unwrap().push_back(right);
	}

	let mut sum = 0;
	'outer: for listing in secnd_s {
		for (i, num) in listing.split(",").enumerate() {
			let num = num.parse::<i32>().unwrap();
			if rules.contains_key(&num) {
				let bans = rules.get(&num).unwrap();
				for j in (0..i).rev() {
					let off: i32 = listing.split(",").nth(j).unwrap().parse().unwrap();
					if bans.contains(&off) {
						continue 'outer;
					}
				}
			}
		}
		let nums: Vec<_> = listing.split(",").map(|x| { x.parse::<i32>().unwrap() }).collect();
		let whr          = nums.len()/2;
		sum += nums[whr];
	}

	println!("final result is {sum}");
}

fn parse_input(file: String) -> (Vec<String>, Vec<String>) {

	let sects = section_file(file);

	return (sects[0].clone(), sects[1].clone());
}
