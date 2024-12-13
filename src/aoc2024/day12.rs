/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

#[derive(Clone, Copy, Hash)]
enum CharState {
	Clean(char),
	Used (char),
	Padding
}

impl CharState {
	fn is_clean  (&self) -> bool { matches!(self, CharState::Clean(_)) }
	fn is_used   (&self) -> bool { matches!(self, CharState::Used (_)) }
	fn is_padding(&self) -> bool { matches!(self, CharState::Padding)  }
	fn get_char(&self) -> char {
		match self {
			CharState::Clean(c) => *c,
			CharState::Used (c) => *c,
			CharState::Padding         => panic!()
		}
	}
	fn same_as(&self, rhs: &CharState) -> bool {
		if !self.is_padding() && !self.is_padding() {
			self.get_char() == rhs.get_char()
		} else {
			false
		}
	}
	fn expend(&mut self) {
		*self = CharState::Used(self.get_char());
	}
}

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i24day12p1.txt").unwrap();

	let mut grid: Vec<Vec<CharState>> = Vec::new();

	for line in file.lines() {
		let mut v = Vec::new();
		for c in line.chars() {
			v.push(CharState::Clean(c));
		}
		grid.push(v);
	}

	match part {
		Part::One => part1(grid),
		Part::Two => part2(grid)
	}

}

fn part2(grid: Vec<Vec<CharState>>) {

	let mut padded_grid: Vec<Vec<CharState>> = vec![vec![CharState::Padding; grid[0].len()+2]; grid.len()+2];

	for (y, row) in grid.into_iter().enumerate() {
		for (x, col) in row.into_iter().enumerate() {
			padded_grid[y+1][x+1] = col;
		}
	}

	let _print_fence = |fence: Vec<Vec<Vec<bool>>>| {
		for row in fence {
			for col in row {
					print!("{}", match (col[0], col[1], col[2], col[3]) {
						(false,false,false,false) => '.',
						(true, false,false,false) => '▌',
						(false,true, false,false) => '▐',
						(false,false,true ,false) => '▀',
						(false,false,false,true ) => '▄',
						(true, true, false,false) => '◫',
						(false,false,true, true)  => '=',
						(true, true, true, false) => '△',
						(true, true, false,true ) => '▽',
						(true, false,true, true ) => '◁',
						(false,true, true, true ) => '▷',
						(true, false,false,true ) => '▙',
						(true, false,true, false) => '▛',
						(false,true, true, false) => '▜',
						(false,true, false, true) => '▟',
						(true, true, true, true ) => '▓',
					});
			}
			println!();
		}
	};

	let mut cost = 0;
	for y in 0..padded_grid.len() {
		for x in 0..padded_grid[0].len() {
			if padded_grid[y][x].is_clean() {
				let mut fences: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; padded_grid[0].len()]; padded_grid.len()];
				let volume = walk2(&mut padded_grid, y, x, &mut fences);
				let mut fence_count = 0;
			
				for z in [2, 3] {
					for row in &fences {
						let mut cur_fence_count = 0;
						for col in row {
							if (cur_fence_count % 2 == 0 &&  col[z]) ||
							   (cur_fence_count % 2 == 1 && !col[z]) {
								cur_fence_count += 1;
							}
						}
						fence_count += cur_fence_count / 2;
					}
				}


				for z in [0, 1] {
					for x in 0..fences[0].len() {
						let mut cur_fence_count = 0;
						for row in &fences {
							if (cur_fence_count % 2 == 0 &&  row[x][z]) ||
							   (cur_fence_count % 2 == 1 && !row[x][z]) {
								cur_fence_count += 1;
							}
						}
						fence_count += cur_fence_count / 2;
					}
				}

//				println!("v{volume}\tf{fence_count}");
//				print_fence(fences);

				cost += volume * fence_count;
			}
		}
	}

	println!("final result is {cost}");
}

fn walk2(grid: &mut Vec<Vec<CharState>>, y: usize, x: usize, fences: &mut Vec<Vec<Vec<bool>>>) -> usize { 

	if grid[y][x].is_used() {
		return 0;
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

	macro_rules! mark {
		($i:expr) => { fences[y][x][$i] = true }
	}

	let left  = get!(0).unwrap();
	let right = get!(1).unwrap();
	let up    = get!(2).unwrap();
	let down  = get!(3).unwrap();

	if !left .same_as(&grid[y][x]) { mark!(0); }
	if !right.same_as(&grid[y][x]) { mark!(1); }
	if !up   .same_as(&grid[y][x]) { mark!(2); }
	if !down .same_as(&grid[y][x]) { mark!(3); }

	let left  = if left .same_as(&grid[y][x]) { walk2(grid, coords[0].0 as usize, coords[0].1 as usize, fences) } else { 0 };
	let right = if right.same_as(&grid[y][x]) { walk2(grid, coords[1].0 as usize, coords[1].1 as usize, fences) } else { 0 };
	let up    = if up   .same_as(&grid[y][x]) { walk2(grid, coords[2].0 as usize, coords[2].1 as usize, fences) } else { 0 };
	let down  = if down .same_as(&grid[y][x]) { walk2(grid, coords[3].0 as usize, coords[3].1 as usize, fences) } else { 0 };

	1 + left + right + up + down
}

fn part1(mut grid: Vec<Vec<CharState>>) {

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

	(perimeter, volume)
}
