/* SPDX-License-Identifier: 0BSD */

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Part {
	One,
	Two
}

pub fn section_file(file: String) -> Vec<Vec<String>> {

	let mut section_count = 1;
	for line in file.lines() { if line == "" { section_count += 1; } }
	let section_count = section_count;

	let mut sections = vec![Vec::<String>::new(); section_count];

	let mut current_section_index = 0;
	let mut current_section: &mut Vec<String> = &mut sections[current_section_index];

	/* Split file in field class section, your_ticket section and tickets section */
	for line in file.lines() {
		if line == "" {
			current_section_index += 1;
			current_section = &mut sections[current_section_index];
		} else {
			current_section.push(line.into());
		}
	}

	return sections;
}
