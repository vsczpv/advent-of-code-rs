/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

	/* The only difference between part one and part two is the target */
	let target = match part {
		Part::One => 2020usize,
		Part::Two => 30000000usize
	};

	/*
	 * A number can never go bigger than the latest one, so the maximum size of our lookup table is roughly that
	 * of our target epoch.
	 */
	let maxsz = target+16;

	let input: Vec<usize> = vec![15,5,1,4,7,0];

	/* Our lookup table. It stores the last epoch a number was seen. */
	let mut numbers_epoch = Vec::<usize>::with_capacity(target);
	let mut start_epoch   = 1usize;

	numbers_epoch.resize(maxsz, 0);

	/* Process input */
	for digit in &input {
		numbers_epoch[*digit] = start_epoch;
		start_epoch += 1;
	}

	let mut current_digit = 0usize;

	/*
	 * If a number has never been seen, the next number is zero.
	 * Otherwise, the next number is dictated by the formula: current_epoch - when.
	 * That is, the current epoch minus when the number was last seen, or how
	 * long has it been when it was last seen.
	 */
	for current_epoch in start_epoch..target {
		let when = numbers_epoch[current_digit];
		numbers_epoch[current_digit] = current_epoch;
		match when {
			0 => current_digit = 0,
			_ => current_digit = current_epoch - when
		}
	}

	println!("{}", current_digit);
}
