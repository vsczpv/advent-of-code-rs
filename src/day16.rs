use crate::common::Part;

use std::collections::HashMap;

const TICKET_LENGTH: usize = 20usize;

type Ticket = [u32; TICKET_LENGTH];
type FieldRule = (u32,u32,u32,u32);

pub fn main(_part: Part) {

	/* Get data from input file */
	let (fields, _your_ticket, other_tickets) = parse_input();

	let mut invalid_tickets = Vec::new();

	/* Find invalid tickets and the offending field */
	for ticket in other_tickets {
		if let Some(violator) = is_ticket_valid(&fields, ticket) {
			invalid_tickets.push(violator);
		}
	}

	/* Get their sum */
	let sum: u32 = invalid_tickets.iter().sum();
	println!("ticket scanning error rate is {sum}");
}

fn is_ticket_valid(field_rules: &HashMap<String, FieldRule>, ticket: Ticket) -> Option<u32> {

	for field in ticket {
		let mut succeded: bool = false;
		for rule in field_rules.values() {
			let valid: bool = (field >= rule.0 && field <= rule.1) ||
			                  (field >= rule.2 && field <= rule.3);
			succeded |= valid;
		}
		if succeded == false {
			return Some(field);
		}
	}

	return None;
}

fn parse_input() -> (HashMap<String, FieldRule>, Ticket, Vec<Ticket>) {

	let mut fields = HashMap::<String, FieldRule>::new();

	let file_backing = std::fs::read_to_string("inputs/iday16p1.txt").unwrap();
	let file         = file_backing.lines();

	let mut first_section  = Vec::<String>::new();
	let mut second_section = Vec::<String>::new();
	let mut third_section  = Vec::<String>::new();

	let mut current_section_index = 0;
	let mut current_section: &mut Vec<String> = &mut first_section;

	/* Split file in field class section, your_ticket section and tickets section */
	for line in file {
		if line == "" {
			match current_section_index {
				0 => current_section = &mut second_section,
				1 => current_section = &mut third_section,
				_ => panic!("Invalid Input: Too many sections.")
			}
			current_section_index += 1;
		} else {
			current_section.push(line.into());
		}
	}


	/*
	 * Parse first section
	 *
	 * <name>: <num1>-<num2> or <num3>-<num4>
	 *
	 */
	for line in first_section {

		let parts_backing = line.to_string();
		let mut parts     = parts_backing.split(":");

		let first_part = parts.next();
		let last_part  = parts.next();

		assert_eq!(parts.count(), 0, "Invalid Input: Mangled field definition.");

		let clauses_backing  = last_part.unwrap().to_string();
		let mut clauses      = clauses_backing.split("or");

		let left_clause_backing = clauses.next().unwrap().to_string();
		let mut left_clause  = left_clause_backing.split("-");

		let right_clause_backing = clauses.next().unwrap().to_string();
		let mut right_clause = right_clause_backing.split("-");

		assert_eq!(clauses.count(), 0, "Invalid Input: Field definitions must have two ranges.");

		let errmsg = "Invalid Input: Mangled number in field definition.";

		let lc_v1: u32 = left_clause .next().unwrap().to_string().replace(" ", "").parse().expect(errmsg);
		let lc_v2: u32 = left_clause .next().unwrap().to_string().replace(" ", "").parse().expect(errmsg);
		let rc_v1: u32 = right_clause.next().unwrap().to_string().replace(" ", "").parse().expect(errmsg);
		let rc_v2: u32 = right_clause.next().unwrap().to_string().replace(" ", "").parse().expect(errmsg);

		assert_eq!(left_clause .count(), 0, "Invalid Input: Ranges must have two digits.");
		assert_eq!(right_clause.count(), 0, "Invalid Input: Ranges must have two digits.");

		fields.insert(first_part.unwrap().into(), (lc_v1, lc_v2, rc_v1, rc_v2));

	}

	/* Get tickets from second and third section */

	assert_eq!(second_section.len(), 2, "Invalid Input: You must have only one ticket.");
	let your_ticket = parse_ticket(second_section[1].clone());


	let mut tickets: Vec<Ticket> = Vec::new();

	for line in third_section.into_iter().skip(1) {
		tickets.push(parse_ticket(line));
	}

	return (fields, your_ticket, tickets);
}

fn parse_ticket(ticket: String) -> Ticket {

	/*
	 * Parse Tickets
	 *
	 * <num>,<num>,<num>,<num>,<num>,<num>,<num>,<num>,<num>,<num>
	 *
	 */

	let mut res: Ticket = [0u32; TICKET_LENGTH];

	let mut numbers = ticket.split(",");

	for i in 0..TICKET_LENGTH {
		res[i] = numbers.next()
		                .expect("Invalid Input: Ticket must have twenty fields.")
						.parse()
						.expect("Invalid Input: Mangle number in ticket definition.");
	}

	return res;
}

/// TESTS

#[cfg(test)]
mod day16tests {

	use super::*;
	use std::io::Write;

	#[test]
	fn day16_parse_input() {

		let mut buf = Vec::new();

		let (fields, your_ticket, other_tickets) = crate::day16::parse_input();

		let mut field_vec: Vec<(String, (u32, u32, u32, u32))> = fields.into_iter().collect();
		field_vec.sort_unstable_by(|x, y| { x.0.cmp(&y.0) });

		for f in field_vec.iter() {
			let (key, val) = f;
			writeln!(buf, "{}: {}-{} or {}-{}", key, val.0, val.1, val.2, val.3).unwrap();
		}

		writeln!(buf, "Your Ticket: ").unwrap();
		for i in 0..TICKET_LENGTH {
			write!(buf, "{},", your_ticket[i]).unwrap();
		}

		for t in other_tickets {
			writeln!(buf, "").unwrap();
			for i in 0..TICKET_LENGTH {
				write!(buf, "{},", t[i]).unwrap();
			}
		}

		writeln!(buf, "").unwrap();

		let ground_truth = std::fs::read("testdata/iday16.txt").unwrap();

		assert_eq!(ground_truth.len(), buf.len());
		assert_eq!(ground_truth, buf);
	}
}
