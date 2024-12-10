/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

use std::collections::HashSet;

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i24day10p1.txt").unwrap();
	let grid = parse_input(file);

	let mut sum = 0;
	for (y, row) in grid.iter().enumerate() {
		for (x, v) in row.iter().enumerate() {
			let mut matches = HashSet::<Pos>::new();
			if *v == 0 {
				matches.clear();
				sum += search(y, x, &grid, &mut matches, part);
			}
		}
	}

	println!("final result is {sum}");

}

#[derive(Hash, PartialEq, Eq)]
struct Pos {
	y: usize,
	x: usize
}

fn search(y: usize, x: usize, grid: &Vec<Vec<u32>>, matches: &mut HashSet<Pos>, part: Part) -> u32 {

	let this = grid[y][x];

	if this == 9 {
		if part == Part::One {
			matches.insert(Pos{y,x});
		}
		return 1;
	}

	let up = (grid.get(y.saturating_sub(1)).and_then(|v| v.get(x+0)),                 y.saturating_sub(1), x+0);
	let dw = (grid.get(y+1)                .and_then(|v| v.get(x+0)),                 y+1, x+0);
	let le = (grid.get(y+0)                .and_then(|v| v.get(x.saturating_sub(1))), y+0, x.saturating_sub(1));
	let ri = (grid.get(y+0)                .and_then(|v| v.get(x+1)),                 y+0, x+1);

	let mut sum = 0;
	for opt in [up, dw, le, ri] {
		sum += match opt {
			(Some(way), y, x) => {
				if *way == this+1 {
					if part == Part::One {
						if matches.contains(&Pos{y, x}) { continue; }
					}
					search(y, x, grid, matches, part)
				} else {
					0
				}
			},
			(None, _, _) => 0
		}
	}

	return sum;
}

fn parse_input(file: String) -> Vec<Vec<u32>>{
	let mut grid: Vec<Vec<u32>> = Vec::new();
	for (y, line) in file.lines().enumerate() {
		grid.push(Vec::new());
		for c in line.chars() {
			grid[y].push(c.to_digit(10).unwrap());
		}
	}
	grid
}
