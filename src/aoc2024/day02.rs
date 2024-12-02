/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

pub fn main(part: Part) {

	let reports = parse_data();

	match part {
		Part::One => part_1(reports),
		Part::Two => part_2(reports)
	}
}

fn count_outliers(rpr: &Vec<i32>) -> i32 {

	/* Take the vector's derivative */
	let outliers: Vec<_> =
		rpr
			.as_slice()
			.windows(2)
			.map(|w| { w[1] - w[0] })
			.collect();

	/*
	 * The 'wrong' values are always represented using negatives, so we must
	 * invert the vector if the first value is negative.
	 */
	let direction = outliers[0].signum();

	/*
	 * Count how many steps are too steep/backwards.
	 */
	let outliers: i32 =
		outliers
			.iter()
			.map(|x| { x * direction })
			.map(|x| { if x >= 1 && x <= 3 { 0 } else { 1 }})
			.sum();

	return outliers;
}

fn part_2(reports: Vec<Vec<i32>>) {

	let mut safe = 0;
	for rpr in reports {

		let outliers = count_outliers(&rpr);

		if outliers == 0 {
			safe += 1;
		}

		/*
		 * In case there is an outlier, we remove one level from the report
		 * and retry. If we find one configuration that has no outliers then
		 * it is safe.
		 */
		if outliers != 0 {
			/* Unga Bunga O(n^2) bruteforce */
			for i in 0..rpr.len() {
				let mut rprcpy = rpr.clone();
				rprcpy.remove(i);
				let outliers = count_outliers(&rprcpy);

				if outliers == 0 {
					safe += 1;
					break;
				}
			}
		}
	}

	println!("{safe}");
}

fn part_1(reports: Vec<Vec<i32>>) {

	/* We just count the outliers and safe++ when there are none. */
	let mut safe = 0;
	for rpr in reports {

		let outliers = count_outliers(&rpr);

		if outliers == 0 {
			safe += 1;
		}
	}

	println!("final result is {safe}");
}

fn parse_data() -> Vec<Vec<i32>> {

	let file       = std::fs::read_to_string("inputs/i24day02p1.txt").expect("file");

	let mut reports = Vec::<Vec<i32>>::new();

	for line in file.lines() {
		let mut levels = Vec::new();
		for number in line.split(" ") {
			let num: i32 = number.parse().expect("number");
			levels.push(num);
		}
		reports.push(levels);
	}

	return reports;
}
