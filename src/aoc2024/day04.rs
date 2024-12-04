/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

	let file  = std::fs::read_to_string("inputs/i24day04p1.txt").unwrap();

	match part {
		Part::One => part_1(file),
		Part::Two => part_2(file)
	}
}

fn part_2(file: String) {

	let (input, width, height) = parse_input(file);

	let mut count = 0;

	/*
	 *   M . S
	 *   . A .
	 *   M . S
	 */
	let left  = ['M', 'M', 'S', 'S'];

	/*
	 *   S . M
	 *   . A .
	 *   S . M
	 */
	let right = ['S', 'S', 'M', 'M'];

	/*
	 *   S . S
	 *   . A .
	 *   M . M
	 */
	let up    = ['S', 'M', 'M', 'S'];

	/*
	 *   M . M
	 *   . A .
	 *   S . S
	 */
	let down  = ['M', 'S', 'S', 'M'];

	let find = |input: &Vec<Vec<char>>, x: usize, y: usize, mtch: [char; 4]| -> bool {

		let mut mtch_iter = mtch.into_iter();

		if input[x][y] != 'A' { return false; }

		let Some(u) = x.checked_sub(1) else { return false; };
		let Some(v) = y.checked_sub(1) else { return false; };

		if u >= width || v >= height { return false; }
		if input[u][v] != mtch_iter.next().unwrap() { return false; }

		let Some(u) = x.checked_sub(1) else { return false; };
		let Some(v) = y.checked_add(1) else { return false; };

		if u >= width || v >= height { return false; }
		if input[u][v] != mtch_iter.next().unwrap() { return false; }

		let Some(u) = x.checked_add(1) else { return false; };
		let Some(v) = y.checked_add(1) else { return false; };

		if u >= width || v >= height { return false; }
		if input[u][v] != mtch_iter.next().unwrap() { return false; }

		let Some(u) = x.checked_add(1) else { return false; };
		let Some(v) = y.checked_sub(1) else { return false; };

		if u >= width || v >= height { return false; }
		if input[u][v] != mtch_iter.next().unwrap() { return false; }

		return true;
	};

	for x in 0..width {
		for y in 0..height {
			count += find(&input, x, y, left)  as i32;
			count += find(&input, x, y, right) as i32;
			count += find(&input, x, y, up)    as i32;
			count += find(&input, x, y, down)  as i32;
		}
	}

	println!("result is {count}");
}

fn part_1(file: String) {

	let (input, width, height) = parse_input(file);

	let mut count = 0;

	let find_vertical = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x;
			let v = y.saturating_add(i);
			if u >= width || v >= height { return false; }
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_vertical_i = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x;
			let v = y.saturating_sub(i);
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_horizont = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.saturating_add(i);
			let v = y;
			if u >= width || v >= height { return false; }
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_horizont_i = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.saturating_sub(i);
			let v = y;
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_diagonal = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.checked_add(i);
			let v = y.checked_add(i);
			let Some(u) = u else { return false; };
			let Some(v) = v else { return false; };
			if u >= width || v >= height { return false; }
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_diagonal_i = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.checked_sub(i);
			let v = y.checked_sub(i);
			let Some(u) = u else { return false; };
			let Some(v) = v else { return false; };
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_diagonal_b = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.checked_add(i);
			let v = y.checked_sub(i);
			let Some(u) = u else { return false; };
			let Some(v) = v else { return false; };
			if u >= width || v >= height { return false; }
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	let find_diagonal_i_b = |input: &Vec<Vec<char>>, x: usize, y: usize| -> bool {
		for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
			let u = x.checked_sub(i);
			let v = y.checked_add(i);
			let Some(u) = u else { return false; };
			let Some(v) = v else { return false; };
			if u >= width || v >= height { return false; }
			let chr = input[u][v];
			if chr != *c { return false; }
		}
		return true;
	};

	for x in 0..width {
		for y in 0..height {
			count += find_vertical    (&input, x, y) as i32;
			count += find_vertical_i  (&input, x, y) as i32;
			count += find_diagonal    (&input, x, y) as i32;
			count += find_diagonal_i  (&input, x, y) as i32;
			count += find_diagonal_b  (&input, x, y) as i32;
			count += find_diagonal_i_b(&input, x, y) as i32;
			count += find_horizont    (&input, x, y) as i32;
			count += find_horizont_i  (&input, x, y) as i32;
		}
	}

	println!("result is {count}");
}

fn parse_input(file: String) -> (Vec<Vec<char>>, usize, usize) {

	let height = file.lines().count();
	let width  = file.lines().next().unwrap().len();

	let mut res = vec![vec![char::default(); height]; width];

	for (i, line) in file.lines().enumerate() {
		res[i] = line.chars().collect();
	}

	return (res, width, height);
}
