/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

const CWN_WIDTH: usize = 20;

pub fn main(_part: Part) {

	let mut cwn = Conway3D::new(CWN_WIDTH);

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
	cwn.set(-2,-4, 0, true);
	cwn.set( 1,-4, 0, true);
	cwn.set( 2,-4, 0, true);
	cwn.set(-4,-3, 0, true);
	cwn.set( 2,-3, 0, true);
	cwn.set( 3,-3, 0, true);
	cwn.set(-4,-2, 0, true);
	cwn.set(-3,-2, 0, true);
	cwn.set(-1,-2, 0, true);
	cwn.set( 1,-2, 0, true);
	cwn.set( 3,-2, 0, true);
	cwn.set(-2,-1, 0, true);
	cwn.set( 2,-1, 0, true);
	cwn.set(-3, 0, 0, true);
	cwn.set(-2, 0, 0, true);
	cwn.set(-1, 0, 0, true);
	cwn.set(-4, 1, 0, true);
	cwn.set(-3, 1, 0, true);
	cwn.set(-2, 1, 0, true);
	cwn.set(-1, 1, 0, true);
	cwn.set( 0, 1, 0, true);
	cwn.set( 1, 1, 0, true);
	cwn.set(-3, 2, 0, true);
	cwn.set(-2, 2, 0, true);
	cwn.set(-1, 2, 0, true);
	cwn.set( 2, 2, 0, true);
	cwn.set(-2, 3, 0, true);
	cwn.set( 1, 3, 0, true);
	cwn.set( 2, 3, 0, true);

	cwn.print_z(0);

//	cwn.set( 0, -1,  0, true);
//	cwn.set( 1,  0,  0, true);
//	cwn.set(-1,  1,  0, true);
//	cwn.set( 0,  1,  0, true);
//	cwn.set( 1,  1,  0, true);

	for _ in 0..6 {
		cwn.iterate();
	}
	println!("final result is {}", cwn.count());
}

struct Conway3D {
	cells: Vec<Vec<Vec<bool>>>,
	width: usize
}

impl Conway3D {
	fn new(width: usize) -> Self {
		Self {
			cells: vec![vec![vec![false; width*2]; width*2]; width*2],
			width
		}
	}
	fn set(&mut self, x: i32, y: i32, z: i32, cell: bool) {
		let (tx, ty, tz) = (x + self.width as i32, y + self.width as i32, z + self.width as i32);

		assert!(tx >= 0);
		assert!(ty >= 0);
		assert!(tz >= 0);

		self.cells[ty as usize][tx as usize][tz as usize] = cell;
	}
	fn print_z(&self, z: i32) {
		let tz = z + self.width as i32;
		println!("z={z}");
		for xcells in self.cells.iter() {
			for ycells in xcells.iter() {
				print!("{}", if ycells[tz as usize] == true { "#" } else { "." });
			}
			println!("");
		}

	}
	fn iterate(&mut self) {
		let mut new_cells = vec![vec![vec![false; self.width*2]; self.width*2]; self.width*2];

		let tail_len = self.cells.len()-2;

		for (x, xcells) in self.cells.iter().enumerate().skip(1).take(tail_len) {
			for (y, ycells) in xcells.iter().enumerate().skip(1).take(tail_len) {
				for (z, cell) in ycells.iter().enumerate().skip(1).take(tail_len) {
					let mut count = 0;
					for xo in -1..=1 as i32 {
						for yo in -1..=1 as i32 {
							for zo in -1..=1 as i32{
								if xo == 0 && yo == 0 && zo == 0 {
									continue;
								}
								count += self.cells[(x as i32+xo) as usize]
								                   [(y as i32+yo) as usize]
												   [(z as i32+zo) as usize] as i32;
							}
						}
					}
					new_cells[x][y][z] = match (cell, count) {
						(true,  2) | (true, 3) => true,
						(false, 3)             => true,
						_                      => false
					};
				}
			}
		}
		self.cells = new_cells;
	}
	fn count(&self) -> i32 {
		self.cells.iter()
			.map(|y| {
				y.iter().map(|z| {
					z.iter().map(|u| {
						*u as i32
					})
				})
			})
			.map(|y| {
				y.map(|z| {
					z.reduce(|zu, zv| {
						zu + zv
					}).unwrap()
				})
			})
			.map(|y| {
				y.reduce(|yu, yv| {
					yu + yv
				}).unwrap()
			})
			.reduce(|xu, xv| {
				xu + xv
			}).unwrap()
	}
}
