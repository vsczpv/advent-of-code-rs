/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

	if part == Part::One {
		panic!("Part one is irrelevant.");
	}

	let file = std::fs::read_to_string("inputs/i24day07p1.txt").unwrap();

	let mut forms: Vec<(u64, Vec<u64>)> = Vec::new();
	for line in file.lines() {
		let mut ls           = line.split(":");
		let left             = ls.next().unwrap().parse::<u64>().unwrap();
		let right: Vec<u64>  = ls.next().unwrap().chars().skip(1).collect::<String>().split(" ")
								 .map(|x| { x.parse::<u64>().unwrap() }).collect();
		forms.push((left,right));
	}

	let mut calsum = 0;
	'outer: for form in forms {
		let amnt = form.1.len();
		assert!(amnt <= 15);
		print!("{form:?} ? ");
		'inner: for i in 0..(2_usize.pow((amnt*2) as u32 - 1)) {
			let mut res = form.1[0];
			for j in 1..amnt {
				match (i >> (j - 1)*2) & 0b11 {
					0b00 => res *= form.1[j],
					0b01 => res += form.1[j],
					0b10 => {
						let mut sres = res.to_string();
						let sfrm     = form.1[j].to_string();
//						print!("{sres} || {sfrm} = ");
						sres.push_str(sfrm.as_str());
//						println!("{sres}");
						res          = sres.parse().unwrap();
					},
					0b11 => continue 'inner,
					_    => panic!()
				}
			}
			if res == form.0 {
				calsum += form.0;
				println!("true");
				continue 'outer;
			}
		}
		println!("false");
	}

	println!("final result is {calsum}");
}
