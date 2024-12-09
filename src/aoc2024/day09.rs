/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

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
	let backward = |x: &mut usize, disk: &Vec<Option<u32>>| while disk[*x] == None { if *x == 0 { return; } else { *x -= 1; } };

	fn backward_until(x: &mut usize, disk: &Vec<Option<u32>>) {
		let dig = disk[*x];
		if dig == None {
			*x -= 1;
			backward_until(x, disk);
			return;
		}
		while disk[*x] == dig {
			if *x == 0 { return; } else { *x -= 1; }
		}
		*x += 1;
	}

	match part {
		Part::One => {
			forward (&mut i, &disk);
			backward(&mut j, &disk);

			loop {

				if j <= i { break; }

				let aux = disk[i];
				disk[i] = disk[j];
				disk[j] = aux;

				forward (&mut i, &disk);
				backward(&mut j, &disk);

			}
		},
		Part::Two => {

			let count  = |mut x: usize, disk: &Vec<Option<u32>>| {
				let mut count = 0;
				let dig       = disk[x];
				while disk[x] == dig {
					x += 1;
					count += 1;
					if x >= disk.len() { break; }
				} count
			};

			forward (&mut i, &disk);
			let mut a_len = count(i, &disk);
			backward_until(&mut j, &disk);
			let mut b_len = count(j, &disk);

			loop {

				if j > i {
					if a_len < b_len {
						i += 1;
						forward(&mut i, &disk);
						a_len = count(i, &disk);
						continue;
					}

					for k in 0..b_len {
						disk[i+k] = disk[j+k];
						disk[j+k] = None;
					}
				}

				i  = 0;
				forward (&mut i, &disk);
				a_len = count(i, &disk);
				j -= 1;
				backward_until(&mut j, &disk);
				if j == 0 { break; }
				b_len = count(j, &disk);
			}
		}
	}

	let mut sum: u64 = 0;
	for (i, d) in disk.iter().enumerate() {
		if *d == None { continue; }
		sum += i as u64 * (d.unwrap() as u64);
	}
	println!("final result is {sum}");
}
