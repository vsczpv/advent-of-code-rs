/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;
use phf::phf_map;

use std::collections::HashSet;

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

	let starting_grid      = grid.clone();
	let (start_x, start_y) = (where_x, where_y);

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

	println!("(part1) final result is {count}");

	let mut candidates: Vec<(usize, usize)> = Vec::new();

	for (y, row) in grid.iter().enumerate() {
		for (x, col) in row.iter().enumerate() {
			if *col == 'X' {
				candidates.push((x, y));
			}
		}
	}

	let mut moves: HashSet<Move> = HashSet::new();
	let mut count = 0;

	for can in candidates {
		let mut new_grid = starting_grid.clone();
		if can.1 == start_y && can.0 == start_x { continue; }
		new_grid[can.1][can.0] = '#';
		where_x = start_x;
		where_y = start_y;
		let cycles = loop {
			let guard = new_grid[where_y][where_x];

			if !moves.insert(Move::new(guard, where_x, where_y)) {
				break true;
			}

			let dm = DELTA_M.get(&guard).unwrap();

			let next = (where_x as isize + dm.0, where_y as isize + dm.1);
			let (next_x, next_y) = (next.0 as usize, next.1 as usize);

			if !boundscheck(next_x, next_y, &new_grid) { break false; }
			let next = new_grid[next_y][next_x];

			if next == '#' {
				new_grid[where_y][where_x] = turn(guard);
			} else {
				where_x = next_x;
				where_y = next_y;
				new_grid[where_y][where_x] = guard;
			}
		};
		if cycles {
			println!("loops at {} {}", can.0, can.1);
			count += 1;
		}
		moves.clear();
	}

	println!("(part2) final result is {count}");
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Move {
	guard: char,
	x:     usize,
	y:     usize
}

impl Move {
	fn new(guard: char, x: usize, y: usize) -> Self { Self { guard, x, y } }
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
