/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

use std::collections::HashMap;

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i24day08p1.txt").unwrap();

	let height = file.lines().count();
	let width  = file.lines().next().unwrap().len();

	let mut grid: Vec<Vec<Option<char>>> = vec![vec![None; width]; height];

	let mut freqs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

	for (y, line) in file.lines().enumerate() {
		for (x, col) in line.chars().enumerate() {
			grid[y][x] = match col {
				'.' => None,
				 _  => {
					if !freqs.contains_key(&col) {
						freqs.insert(col, vec![(y, x)]);
					} else {
						freqs.get_mut(&col).unwrap().push((y, x));
					}
					Some(col)
				}
			};
		}
	}

	let printgrid = |grid: &Vec<Vec<Option<char>>>| {
		for line in grid {
			for col in line {
					print!("{}", if let Some(c) = col { *c } else { '.' });
			}
			println!("");
		}
	};

	let delta = |a: &(usize, usize), b: &(usize, usize)| (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

	printgrid(&grid);

	let part1 = |a: &(usize, usize), b: &(usize, usize), gr: &mut Vec<Vec<Option<char>>>| {
		let d        = delta(a, b);
		let (cy, cx) = (a.0 as isize - d.0, a.1 as isize - d.1);
		let (cy, cx) = (cy  as usize,       cx  as usize);
		if cy >= height || cx >= width { return; }
		gr[cy][cx] = Some('#');
	};

	let part2 = |a: &(usize, usize), b: &(usize, usize), gr: &mut Vec<Vec<Option<char>>>| {
		let d        = delta(a, b);
		let mut multiplier = 1;
		gr[a.0][a.1] = Some('#');
		loop {
			let (cy, cx) = (a.0 as isize - d.0 * multiplier, a.1 as isize - d.1 * multiplier);
			let (cy, cx) = (cy  as usize, cx  as usize);
			if cy >= height || cx >= width { return; }
			gr[cy][cx] = Some('#');
			multiplier += 1;
		}
	};

	for l in freqs.into_values() {
		for (i, a) in l.iter().enumerate() {
			for (j, b) in l.iter().enumerate() {
				if i == j { continue; }
				match part {
					Part::One => part1(a, b, &mut grid),
					Part::Two => part2(a, b, &mut grid)
				}
			}
		}
	}

	printgrid(&grid);

	let count: i32 =
		grid
			.iter()
			.map (|y| y.iter().map(|x| if let Some(c) = *x { (c == '#') as i32 } else { 0 }).sum::<i32>())
			.sum ();

	println!("{count}");
}
