/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;
use phf::phf_map;

pub fn main(_part: Part) {

	let file = std::fs::read_to_string("inputs/i24day06p1.txt").unwrap();

	let mut grid: Vec<Vec<char>> = Vec::new();

	let (mut where_x, mut where_y) = (0, 0);
	for (y, line) in file.lines().enumerate() {
		grid.push(Vec::new());
		for (x, c) in line.chars().enumerate() {
			grid[y].push(c);
			if c == '^' {
				where_x = x;
				where_y = y;
			}
		}
	}

	static DELTA_M: phf::Map<char, (isize, isize)> = phf_map! {
		'^' => ( 0, -1),
		'>' => ( 1,  0),
		'v' => ( 0,  1),
		'<' => (-1,  0)
	};

	loop {
		let guard = grid[where_y][where_x];
		grid[where_y][where_x] = 'X';

		let dm = DELTA_M.get(&guard).unwrap();

		let next = (where_x as isize + dm.0, where_y as isize + dm.1);
		let (next_x, next_y) = (next.0 as usize, next.1 as usize);

		if !boundscheck(next_x, next_y, &grid) { break; }
		let next = grid[next_y][next_x];

		if next == '#' {
			grid[where_y][where_x] = turn(guard);
		} else {
			where_x = next_x;
			where_y = next_y;
			grid[where_y][where_x] = guard;
		}
	}
	printgrid(&grid);

	let count: i32 =
		grid
			.iter()
			.map(|x| { x.iter().map(|y| { (*y == 'X') as i32 }).sum::<i32>() })
			.sum();

	println!("final result is {count}");
}

fn printgrid(grid: &Vec<Vec<char>>) {
	for line in grid {
		for col in line {
			print!("{col}");
		}
		println!("");
	}
}

fn boundscheck(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
	if y < grid.len() {
		if x < grid[0].len() {
			return true;
		}
	}
	return false;
}

fn turn(guard: char) -> char {
	match guard {
		'^' => '>',
		'>' => 'v',
		'v' => '<',
		'<' => '^',
		 _  => panic!()
	}
}
