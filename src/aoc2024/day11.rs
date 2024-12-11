/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

use std::collections::HashMap;

pub fn main(part: Part) {

	let blinks = match part {
		Part::One => 25,
		Part::Two => 75
	};

	let file = std::fs::read_to_string("inputs/i24day11p1.txt").unwrap();
	let mut root = Vec::new();

	for num in file.split(" ") {
		root.push(num.parse::<u128>().unwrap());
	}

	let mut total = 0;
	let mut cache: HashMap<NodeBlinkPair, u128> = HashMap::new();
	for node in root {
		total += travel(node, blinks, &mut cache);
	}

	println!("final result is {total}");
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct NodeBlinkPair(u128, u32);

fn travel(node: u128, blinks: u32, cache: &mut HashMap<NodeBlinkPair, u128>) -> u128 {

	if blinks == 0 { return 1; }

	if let Some(item) = cache.get(&NodeBlinkPair(node, blinks)).copied() {
		return item;
	}

	let blinks = blinks - 1;

	let digicount = |v: u128| v.ilog10() + 1;

	if node == 0 {
		let res = travel(1, blinks, cache);
		cache.insert(NodeBlinkPair(1, blinks), res);
		res
	} else {
		let ct = digicount(node);
		if ct % 2 == 0 {
			let nodestr       = node.to_string();
			let (left, right) = nodestr.split_at(nodestr.len()/2);
			let (left, right) = (left.parse().unwrap(), right.parse().unwrap());

			let left_res  = travel(left,  blinks, cache);
			let right_res = travel(right, blinks, cache);

			cache.insert(NodeBlinkPair(left,  blinks), left_res);
			cache.insert(NodeBlinkPair(right, blinks), right_res);

			left_res + right_res
		} else {
			let res = travel(node*2024, blinks, cache);
			cache.insert(NodeBlinkPair(node*2024, blinks), res);
			res
		}
	}
}
