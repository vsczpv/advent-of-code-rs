/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(_part: Part) {

	let file = std::fs::read_to_string("inputs/i24day09p1.txt").unwrap();

	let mut disk = Vec::<Option<u32>>::new();

	let mut id = 0;
	for (i, c) in file.chars().enumerate() {
		let d = c.to_digit(10).unwrap();
		if i % 2 == 0 {
			for _ in 0..d { disk.push(Some(id)); }
			id += 1;
		} else {
			for _ in 0..d { disk.push(None) }
		}
	}

	let mut i = 0;
	let mut j = disk.len()-1;

	let forward  = |x: &mut usize, disk: &Vec<Option<u32>>| while disk[*x] != None { *x += 1; };
	let backward = |x: &mut usize, disk: &Vec<Option<u32>>| while disk[*x] == None { *x -= 1; };

	forward (&mut i, &disk);
	backward(&mut j, &disk);

	loop {

		if j <= i { break; }

		let aux = disk[i];
		disk[i] = disk[j];
		disk[j] = aux;

		forward (&mut i, &disk);
		backward(&mut j, &disk);

		/*
		for d in &disk {
			if *d == None {
				print!(".");
			} else {
				print!("{}", d.unwrap());
			}
		}
		println!("");
		*/

	}

	/*
	for d in &disk {
		if *d == None { break; }
		print!("{}", d.unwrap());
	}
	println!("");
	*/

	let mut sum: u64 = 0;
	for (i, d) in disk.iter().enumerate() {
		if *d == None { break; }
		sum += i as u64 * (d.unwrap() as u64);
	}
	println!("final result is {sum}");
}
