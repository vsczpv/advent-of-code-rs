/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

use std::collections::LinkedList;
use std::collections::HashMap;
//use std::cell::RefCell;
//use std::cell::Ref;
use std::rc::Rc;
//use std::rc::Weak;

pub fn main(_part: Part) {

	let file = std::fs::read_to_string("inputs/i20day19p1.txt").unwrap();

	let (first_section, second_section) = sections(file);

	let first_section = filter_terminals(first_section);

	let mut left_stack   = LinkedList::<SkelNode>::new();
	let mut right_stack  = LinkedList::<SkelNode>::new();
	let mut built_map    = HashMap::<i32, Rc<PTNode>>::new();
//	let mut node_storage = LinkedList::<Rc<PTNode>>::new();

	for line in first_section {
		let mut split = line.split(":");
		let digit = split.next().unwrap();
		let rule  = split.next().unwrap();
		if rule.contains("|") {

			let mut rulesplit    = rule.split("|");
			let rule_one: String = rulesplit.next().unwrap().chars().skip(1).collect();
			let rule_two: String = rulesplit.next().unwrap().chars().skip(1).collect();
			let mut one_pieces   = rule_one.split_whitespace();
			let mut two_pieces   = rule_two.split_whitespace();

			let atxt = one_pieces.next().unwrap().to_string();
			let ctxt = two_pieces.next().unwrap().to_string();

			let mut newrule = SkelNode {
				a:  if let Ok(r) = atxt.parse::<i32>() { SkelData::NonTerminal(r) } else { SkelData::Terminal(atxt.chars().nth(0).unwrap()) },
				b:  None,
				c:  if let Ok(r) = ctxt.parse::<i32>() { SkelData::NonTerminal(r) } else { SkelData::Terminal(ctxt.chars().nth(0).unwrap()) },
				d:  None,
				id: digit.parse().unwrap()
			};

			let btxt = one_pieces.next();
			let dtxt = two_pieces.next();

			if let Some(btxt) = btxt {
				newrule.b = if let Ok(r) = btxt.parse() { Some(SkelData::NonTerminal(r)) } else { Some(SkelData::Terminal(btxt.chars().nth(0).unwrap())) }
			}

			if let Some(dtxt) = dtxt {
				newrule.d = if let Ok(r) = dtxt.parse() { Some(SkelData::NonTerminal(r)) } else { Some(SkelData::Terminal(dtxt.chars().nth(0).unwrap())) }
			}

			left_stack.push_back(newrule);

		} else {
			let rule: String = rule.chars().skip(1).collect();
			let mut pieces = rule.split_whitespace();
			let atxt = pieces.next().unwrap().to_string();
			let btxt = pieces.next().unwrap().to_string();
			let a: Result<i32, _> = atxt.parse();
			let b: Result<i32, _> = btxt.parse();

			let a = if let Ok(r) = a { SkelData::NonTerminal(r) } else { SkelData::Terminal(atxt.chars().nth(0).unwrap()) };
			let b = if let Ok(r) = b { SkelData::NonTerminal(r) } else { SkelData::Terminal(btxt.chars().nth(0).unwrap()) };

			let newrule = SkelNode {
				a,
				b:  Some(b),
				c:  a,
				d:  Some(b),
				id: digit.parse().unwrap()
			};

			left_stack.push_back(newrule);
		}
	}

	let terminal_node_a = Rc::new(PTNode::Terminal('a'));
	let terminal_node_b = Rc::new(PTNode::Terminal('b'));

	let mut count = built_map.len();

	loop {
		for item in &left_stack {
			match (item.a, item.b, item.c, item.d) {
				(SkelData::Terminal(a), Some(SkelData::Terminal(b)), SkelData::Terminal(c), Some(SkelData::Terminal(d))) => {
					built_map.insert(item.id, PTNode::NonTerminal(NonTerminalPTNodeData {
						options: [
							(
								if a == 'a' { terminal_node_a.clone() }       else { terminal_node_b.clone() },
								if b == 'a' { Some(terminal_node_a.clone()) } else { Some(terminal_node_b.clone()) }
							),
							(
								if c == 'a' { terminal_node_a.clone() }       else { terminal_node_b.clone() },
								if d == 'a' { Some(terminal_node_a.clone()) } else { Some(terminal_node_b.clone()) }
							)
						]
					}).into());
				},
				(SkelData::Terminal(a), None, SkelData::Terminal(c), None) => {
					built_map.insert(item.id, PTNode::NonTerminal(NonTerminalPTNodeData {
						options: [
							(
								if a == 'a' { terminal_node_a.clone() }       else { terminal_node_b.clone() },
								None,
							),
							(
								if c == 'a' { terminal_node_a.clone() }       else { terminal_node_b.clone() },
								None,
							)
						]
					}).into());
				},
				_ => {
					let a = item.a;
					let b = item.b;
					let c = item.c;
					let d = item.d;

					let a = if let SkelData::NonTerminal(a) = a {
						if !built_map.contains_key(&a) {
							right_stack.push_back(item.clone());
							continue;
						} else {
							built_map.get(&a).unwrap().clone()
						}
					} else {
						let SkelData::Terminal(a) = a else { panic!(); };
						match a {
							'a' => terminal_node_a.clone(),
							'b' => terminal_node_b.clone(),
							_  => panic!()
						}
					};

					let b = if let Some(SkelData::NonTerminal(b)) = b {
						if !built_map.contains_key(&b) {
							right_stack.push_back(item.clone());
							continue;
						} else {
							built_map.get(&b).unwrap().clone()
						}
					} else {
						let Some(SkelData::Terminal(b)) = b else { panic!(); };
						match b {
							'a' => terminal_node_a.clone(),
							'b' => terminal_node_b.clone(),
							_  => panic!()
						}
					};

					let c = if let SkelData::NonTerminal(c) = c {
						if !built_map.contains_key(&c) {
							right_stack.push_back(item.clone());
							continue;
						} else {
							built_map.get(&c).unwrap().clone()
						}
					} else {
						let SkelData::Terminal(c) = c else { panic!(); };
						match c {
							'a' => terminal_node_a.clone(),
							'b' => terminal_node_b.clone(),
							_  => panic!()
						}
					};

					let d = if let Some(SkelData::NonTerminal(d)) = d {
						if !built_map.contains_key(&d) {
							right_stack.push_back(item.clone());
							continue;
						} else {
							built_map.get(&d).unwrap().clone()
						}
					} else {
						let Some(SkelData::Terminal(d)) = d else { panic!(); };
						match d {
							'a' => terminal_node_a.clone(),
							'b' => terminal_node_b.clone(),
							_  => panic!()
						}
					};
					built_map.insert(item.id, PTNode::NonTerminal(NonTerminalPTNodeData {
						options: [(a, Some(b)), (c, Some(d))]
					}).into());
				}
			}
		}
		left_stack.clear();
		std::mem::swap(&mut left_stack, &mut right_stack);

		if built_map.len() == count {
			break;
		} else {
			count = built_map.len();
		}
	}

	let root      = built_map.get(&0).unwrap().clone();
	let ast       = ParserTree::new(root);
	let mut count = 0;

	for line in second_section {
		count += ast.test(line.as_str()) as i32;
	}

	println!("final result is {count}");

	/*
	 * 0: 1 3
	 * 1: ac
	 * 2: bd
	 * 3: 4 5 | 5 4
	 * 4: 1 1 | 2 2
	 * 5: 1 2 | 2 1
	 */

	/*
	let nt_5 = Rc::new(PTNode::NonTerminal(NonTerminalPTNodeData {
		options: [(terminal_node_a.clone(), Some(terminal_node_b.clone())), (terminal_node_b.clone(), Some(terminal_node_a.clone()))]
	}));

	let nt_4 = Rc::new(PTNode::NonTerminal(NonTerminalPTNodeData {
		options: [(terminal_node_a.clone(), Some(terminal_node_a.clone())), (terminal_node_b.clone(), Some(terminal_node_b.clone()))]
	}));

	let nt_3 = Rc::new(PTNode::NonTerminal(NonTerminalPTNodeData {
		options: [(nt_4.clone(), Some(nt_5.clone())), (nt_5.clone(), Some(nt_4.clone()))],
	}));

	let root = PTNode::NonTerminal(NonTerminalPTNodeData {
		options: [(terminal_node_a.clone(), Some(nt_3.clone())), (terminal_node_a.clone(), Some(nt_3.clone()))],
	});

	let ast = ParserTree::new(root);

	println!("{}", ast.test("abaaa"));
	*/
}

fn filter_terminals(section: Vec<String>) -> Vec<String> {

	let mut a_digit = -1;
	let mut b_digit = -1;

	for line in &section {
		if line.contains("\"") {
			let digit = line.split(":").next().unwrap().parse().unwrap();
			match line.chars().nth(line.find("\"").unwrap()+1).unwrap() {
				'a' => a_digit = digit,
				'b' => b_digit = digit,
				 _  => panic!()
			}
		}
	}

	let section: Vec<_> =
		section
			.into_iter()
			.filter(|x| {
				let digit = x.split(":").next().unwrap().parse::<i32>().unwrap();
				digit != a_digit && digit != b_digit
			})
			.collect();

	let section: Vec<_> =
		section
			.into_iter()
			.map(|x| {
				let x = x.replace(a_digit.to_string().as_str(), "a");
				let x = x.replace(b_digit.to_string().as_str(), "b");
				x
			})
			.collect();

	return section;
}

fn sections(file: String) -> (Vec<String>, Vec<String>) {

	let mut first_section  = Vec::<String>::new();
	let mut second_section = Vec::<String>::new();

	let mut current_section_index = 0;
	let mut current_section: &mut Vec<String> = &mut first_section;

	for line in file.lines() {
		if line == "" {
			match current_section_index {
				0 => current_section = &mut second_section,
				_ => panic!("Invalid Input: Too many sections.")
			}
			current_section_index += 1;
		} else {
			current_section.push(line.into());
		}
	}

	(first_section, second_section)
}

#[derive(Clone, Copy, Debug)]
enum SkelData {
	NonTerminal(i32),
	Terminal(char)
}

#[derive(Clone, Copy, Debug)]
struct SkelNode {
	a:  SkelData,
	b:  Option<SkelData>,
	c:  SkelData,
	d:  Option<SkelData>,
	id: i32
}

// type PTNodePair <'a> = (&'a PTNode<'a>, Option<&'a PTNode<'a>>);
type PTNodePair = (Rc<PTNode>, Option<Rc<PTNode>>);

struct NonTerminalPTNodeData /*<'a>*/ {
	options: [PTNodePair/*<'a>*/; 2],
}

enum PTNode/*<'a>*/ {
	NonTerminal(NonTerminalPTNodeData/*<'a>*/),
	Terminal   (char)
}

impl/*<'a>*/ PTNode/*<'a>*/ {
	fn execute(&self, input: &str, index: usize) -> bool { // Option<String> {
		match self {
			PTNode::NonTerminal(data) => {

				let tryexec = |input: &str, index: usize, src: usize| -> bool {

					let opt = data.options[src].clone();

					let successful = opt.0.execute(input, index);

					if successful {
//						if opt.1 == Some {
						if let Some(opt1) = opt.1 {
							let successful = opt1.execute(input, index+1);
							if successful {
								return true;
							} else {
								return false;
							}
						} else {
							return true;
						}
					} else {
						return false;
					}
				};

				if tryexec(input, index, 0) { return true; } else {
					return tryexec(input, index, 1);
				}

			},
			PTNode::Terminal(data) => {
				let current_char = input.chars().nth(index).unwrap();
				if current_char == *data { true } else { false }
			}
		}
	}
}

struct ParserTree {
	root: Rc<PTNode>,
}

impl ParserTree {
	fn new(root: Rc<PTNode>) -> Self {
		ParserTree { root }
	}
	fn test(&self, input: &str) -> bool {
		self.root.execute(input, 0)
	}
}
