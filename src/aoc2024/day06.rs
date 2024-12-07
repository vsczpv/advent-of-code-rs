/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;
use phf::phf_map;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

const WALL_ID     : u8 = 0b11111111u8;
const U_MASK      : u8 = 0b1000u8;
const R_MASK      : u8 = 0b0100u8;
const D_MASK      : u8 = 0b0010u8;
const L_MASK      : u8 = 0b0001u8;
const GUARD_SHIFT : u8 = 4u8;
const VISIT_MASK  : u8 = 0b1111u8;

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

	let candidates = Arc::new(candidates);

	let starting_grid: Vec<Vec<u8>> =
		starting_grid
			.iter()
			.map(|x| { x.iter().map(|y| {
				match y {
					'#' => WALL_ID,
					'^' => U_MASK << GUARD_SHIFT,
					'.' => 0u8,
					 _  => panic!("impossible scenario, {y}")
				}
			}).collect()})
			.collect();

	let thread_count = 4;

	let (tx, rx) = mpsc::channel();

	let candidates: Vec<Vec<_>> = candidates.chunks(candidates.len()/thread_count).map(|s| s.into()).collect();

	for thread_id in 0..thread_count {
		let new_grid = starting_grid.clone();
		make_thread(candidates[thread_id].clone(), start_x, start_y, new_grid, tx.clone());
	}

	drop(tx);

	let mut count = 0;
	for msg in rx {
		count += msg;
	}

	println!("(part2) final result is {count}");
}

fn delta_m_u8(i: u8) -> (isize, isize) {
	match i {
		U_MASK => ( 0, -1),
		R_MASK => ( 1,  0),
		D_MASK => ( 0,  1),
		L_MASK => (-1,  0),
		_      => panic!("")
	}
}

fn make_thread(candidates: /*Arc<Vec<(usize, usize)>>*/ Vec<(usize, usize)>, start_x: usize, start_y: usize,
			   mut new_grid: Vec<Vec<u8>>, tx: std::sync::mpsc::Sender<i32>) {
	thread::spawn(move || {

		let mut count = 0;

		for can in candidates {

			if can.1 == start_y && can.0 == start_x { continue; }

			new_grid[can.1][can.0] = WALL_ID;
			let mut where_x = start_x;
			let mut where_y = start_y;
			new_grid[where_y][where_x] = U_MASK << GUARD_SHIFT;

			let cycles = loop {

				let mut guard = new_grid[where_y][where_x] >> GUARD_SHIFT;

				if (guard & (new_grid[where_y][where_x] & VISIT_MASK)) != 0 {
					break true;
				}

				let dm = delta_m_u8(guard);

				let (next_x, next_y) = (where_x as isize + dm.0, where_y as isize + dm.1);
				let (next_x, next_y) = (next_x  as usize,        next_y  as usize);

				if !boundscheck_u8(next_x, next_y, &new_grid) { break false; }

				let next = new_grid[next_y][next_x];

				let mut iterate = |shall_move: bool| {
					let old = new_grid[where_y][where_x];
					let old = (old & VISIT_MASK) | (old >> GUARD_SHIFT);
					new_grid[where_y][where_x] = old;
					if shall_move {
						where_x = next_x;
						where_y = next_y;
					} else {
						guard = turn_u8(guard);
					}
					new_grid[where_y][where_x] |= guard << GUARD_SHIFT;
				};

				iterate(next != WALL_ID);

			};

			if cycles {
				count += 1;
			}

			/* Clear Grid */
			new_grid[where_y][where_x] = 0u8;
			new_grid
				.iter_mut()
				.for_each(|x| {
					x.iter_mut().for_each(|y| { *y = (*y >> 7) * WALL_ID /* *y = if *y == WALL_ID { *y } else { 0u8 }*/ });
				});
			new_grid[can.1][can.0] = 0u8;
		}

		tx.send(count)
			.expect("Unable to send over channel.");
	});
}

fn boundscheck_u8(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> bool {
	if y < grid.len() {
		if x < grid[0].len() {
			return true;
		}
	}
	return false;
}

fn boundscheck(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
	if y < grid.len() {
		if x < grid[0].len() {
			return true;
		}
	}
	return false;
}

fn turn_u8(guard: u8) -> u8 {
	match guard {
		U_MASK => R_MASK,
		R_MASK => D_MASK,
		D_MASK => L_MASK,
		L_MASK => U_MASK,
		 _  => panic!()
	}
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
