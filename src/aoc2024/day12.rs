/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

#[derive(Clone, Copy, Hash)]
enum CharState {
	Clean(char),
	Used (char)
}

impl CharState {
	fn is_clean(&self) -> bool {
		match self {
			CharState::Clean(_) => true,
			_                   => false
		}
	}
	fn is_used(&self) -> bool {
		match self {
			CharState::Used(_) => true,
			_                  => false
		}
	}
	fn get_char(&self) -> char {
		match self {
			CharState::Clean(c) => *c,
			CharState::Used (c) => *c
		}
	}
	fn same_as(&self, rhs: &CharState) -> bool {
		self.get_char() == rhs.get_char()
	}
	fn expend(&mut self) {
		*self = CharState::Used(self.get_char());
	}
}

pub fn main(_part: Part) {

	let file = std::fs::read_to_string("inputs/i24day12p1.txt").unwrap();

	let mut grid: Vec<Vec<CharState>> = Vec::new();

	for line in file.lines() {
		let mut v = Vec::new();
		for c in line.chars() {
			v.push(CharState::Clean(c));
		}
		grid.push(v);
	}

	let mut cost = 0;
	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if grid[y][x].is_clean() {
				let (perimeter, volume) = walk(&mut grid, y, x);
				cost += perimeter * volume;
			}
		}
	}

	println!("final result is {cost}");
}

fn walk(grid: &mut Vec<Vec<CharState>>, y: usize, x: usize) -> (usize, usize) {

	if grid[y][x].is_used() {
		return (0, 0);
	}

	grid[y][x].expend();

	let coords: [(isize, isize); 4] = [
		(y as isize,     x as isize - 1),
		(y as isize,     x as isize + 1),
		(y as isize - 1, x as isize),
		(y as isize + 1, x as isize)
	];

	macro_rules! get {
		($i:expr) => { grid.get(coords[$i].0 as usize).and_then(|row| row.get(coords[$i].1 as usize)).cloned() }
	}

	let left  = get!(0);
	let right = get!(1);
	let up    = get!(2);
	let down  = get!(3);

	let perimeter =
		left .map_or(1, |v| !v.same_as(&grid[y][x]) as usize) +
		right.map_or(1, |v| !v.same_as(&grid[y][x]) as usize) +
		up   .map_or(1, |v| !v.same_as(&grid[y][x]) as usize) +
		down .map_or(1, |v| !v.same_as(&grid[y][x]) as usize);

	let left  = if left .is_some_and(|v| v.same_as(&grid[y][x])) { walk(grid, coords[0].0 as usize, coords[0].1 as usize) } else { (0, 0) };
	let right = if right.is_some_and(|v| v.same_as(&grid[y][x])) { walk(grid, coords[1].0 as usize, coords[1].1 as usize) } else { (0, 0) };
	let up    = if up   .is_some_and(|v| v.same_as(&grid[y][x])) { walk(grid, coords[2].0 as usize, coords[2].1 as usize) } else { (0, 0) };
	let down  = if down .is_some_and(|v| v.same_as(&grid[y][x])) { walk(grid, coords[3].0 as usize, coords[3].1 as usize) } else { (0, 0) };

	let perimeter = perimeter + left.0 + right.0 + up.0 + down.0;
	let volume    = 1         + left.1 + right.1 + up.1 + down.1;

	return (perimeter, volume);
}
