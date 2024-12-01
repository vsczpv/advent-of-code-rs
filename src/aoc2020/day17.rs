/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

const CWN_WIDTH: usize = 20;

pub fn main(part: Part) {

	let mut cwn = Conway3D::new(CWN_WIDTH);

	assert_ne!(part, Part::One, "Part one is irrelevant.");

	/*     0
	 * ..#..##.
     * #.....##
     * ##.#.#.#
     * ..#...#.
     * .###!... 0
     * ######..
     * .###..#.
     * ..#..##.
	 *
	 */
	cwn.set(-2,-4, 0, 0, true);
	cwn.set( 1,-4, 0, 0, true);
	cwn.set( 2,-4, 0, 0, true);
	cwn.set(-4,-3, 0, 0, true);
	cwn.set( 2,-3, 0, 0, true);
	cwn.set( 3,-3, 0, 0, true);
	cwn.set(-4,-2, 0, 0, true);
	cwn.set(-3,-2, 0, 0, true);
	cwn.set(-1,-2, 0, 0, true);
	cwn.set( 1,-2, 0, 0, true);
	cwn.set( 3,-2, 0, 0, true);
	cwn.set(-2,-1, 0, 0, true);
	cwn.set( 2,-1, 0, 0, true);
	cwn.set(-3, 0, 0, 0, true);
	cwn.set(-2, 0, 0, 0, true);
	cwn.set(-1, 0, 0, 0, true);
	cwn.set(-4, 1, 0, 0, true);
	cwn.set(-3, 1, 0, 0, true);
	cwn.set(-2, 1, 0, 0, true);
	cwn.set(-1, 1, 0, 0, true);
	cwn.set( 0, 1, 0, 0, true);
	cwn.set( 1, 1, 0, 0, true);
	cwn.set(-3, 2, 0, 0, true);
	cwn.set(-2, 2, 0, 0, true);
	cwn.set(-1, 2, 0, 0, true);
	cwn.set( 2, 2, 0, 0, true);
	cwn.set(-2, 3, 0, 0, true);
	cwn.set( 1, 3, 0, 0, true);
	cwn.set( 2, 3, 0, 0, true);

	cwn.print_zw(0, 0);

	for i in 0..6 {
		println!("Iteration {i}");
		cwn.iterate();
	}
	println!("final result is {}", cwn.count());
}

struct Conway3D {
//	cells: Vec<Vec<Vec<bool>>>,
	cells: Vec<Vec<Vec<Vec<bool>>>>, /* Holy baloney! */
	width: usize
}

impl Conway3D {
	fn new(width: usize) -> Self {
		Self {
			cells: vec![vec![vec![vec![false; width*2]; width*2]; width*2]; width*2],
			width
		}
	}
	fn set(&mut self, x: i32, y: i32, z: i32, w: i32, cell: bool) {
		let (tx, ty, tz, tw) = (x + self.width as i32, y + self.width as i32, z + self.width as i32, w + self.width as i32);

		assert!(tx >= 0);
		assert!(ty >= 0);
		assert!(tz >= 0);
		assert!(tw >= 0);

		self.cells[ty as usize][tx as usize][tz as usize][tw as usize] = cell;
	}
	fn print_zw(&self, z: i32, w: i32) {
		let (tz, tw) = (z + self.width as i32, w + self.width as i32);
		println!("z={z}, w={w}");
		for xcells in self.cells.iter() {
			for ycells in xcells.iter() {
				print!("{}", if ycells[tz as usize][tw as usize] == true { "#" } else { "." });
			}
			println!("");
		}

	}
	fn iterate(&mut self) {
		let mut new_cells = vec![vec![vec![vec![false; self.width*2]; self.width*2]; self.width*2]; self.width*2];

		let tail_len = self.cells.len()-2;

		for (x, xcells) in self.cells.iter().enumerate().skip(1).take(tail_len) {
			for (y, ycells) in xcells.iter().enumerate().skip(1).take(tail_len) {
				for (z, zcells) in ycells.iter().enumerate().skip(1).take(tail_len) {
					for (w, cell) in zcells.iter().enumerate().skip(1).take(tail_len) {
						let mut count = 0;
						for xo in -1..=1 as i32 {
							for yo in -1..=1 as i32 {
								for zo in -1..=1 as i32 {
									for wo in -1..=1 as i32 {
										if xo == 0 && yo == 0 && zo == 0 && wo == 0 {
											continue;
										}
										count += self.cells[(x as i32+xo) as usize]
														   [(y as i32+yo) as usize]
														   [(z as i32+zo) as usize]
														   [(w as i32+wo) as usize] as i32;
									}
								}
							}
						}
						new_cells[x][y][z][w] = match (cell, count) {
							(true,  2) | (true, 3) => true,
							(false, 3)             => true,
							_                      => false
						};
					}
				}
			}
		}
		self.cells = new_cells;
	}
	fn count(&self) -> i32 {
		let mut count = 0;
		for xcells in &self.cells {
			for ycells in xcells {
				for zcells in ycells {
					for cell in zcells {
						count += *cell as i32;
					}
				}
			}
		}
		return count;
	}
}
