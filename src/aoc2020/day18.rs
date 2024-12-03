/* SPDX-License-Identifier: 0BSD */

use crate::common::Part;

use std::rc::Rc;
use std::collections::LinkedList;

use regex::Regex;

pub fn main(part: Part) {

	let file = std::fs::read_to_string("inputs/i20day18p1.txt").unwrap();

	match part {
		Part::One => parse_input_pt1(file),
		Part::Two => parse_input_pt2(file)
	}
}

/*
 * The Fortran I compiler would expand each operator with a sequence of parentheses. In a simplified form of the algorithm, it would
 *
 *     replace + and – with ))+(( and ))-((, respectively;
 *     replace * and / with )*( and )/(, respectively;
 *     add (( at the beginning of each expression and after each left parenthesis in the original expression; and
 *     add )) at the end of the expression and before each right parenthesis in the original expression.
 *
 * Although not obvious, the algorithm was correct, and, in the words of Knuth, “The resulting formula is
 * properly parenthesized, believe it or not.”
 *
 * Wikipedia, the free encyclopedia.
 *
 * Obviously, we're goint to invert the precedences.
 */
fn parse_input_pt2(file: String) {

	let mut newexpr = String::from("");
	for line in file.lines() {

		let mut knuth = String::from("((");
		for chr in line.chars() {
			match chr {
				'(' => knuth.push_str("((("),
				')' => knuth.push_str(")))"),
				'+' => knuth.push_str(")+("),
				'*' => knuth.push_str("))*(("),
				' ' => {},
				_   => knuth.push_str(chr.to_string().as_str())
			}
		}
		knuth.push_str("))");

		let mut pruned_knuth = knuth.clone();
		let mut drop_phase = 0;

		/*
		 * Our part one parser doesn't fair well with lone values inside parethesis, so
		 * we prune 'em. Wouldn't be necessary if we modified the parser, but I don't
		 * want to as this is a lot easier.
		 */
		loop {

			knuth = pruned_knuth.clone();
			pruned_knuth.clear();
			knuth.push_str("@@");

			for i in 0..(knuth.len() - 2usize) {

				let a = knuth.chars().nth(i+0).unwrap();
				let b = knuth.chars().nth(i+1).unwrap();
				let c = knuth.chars().nth(i+2).unwrap();

				match drop_phase {
					0 => {
						if a == '(' && b.is_digit(10) && c == ')' {
							drop_phase = 1;
						} else {
							pruned_knuth.push(a);
						}
					},
					1 => {
						pruned_knuth.push(a);
						drop_phase = 2;
					},
					2 => {
						drop_phase = 0;
					}
					_ => panic!()
				};

			}

			knuth.pop();
			knuth.pop();

			if knuth == pruned_knuth { break; }

		};

		newexpr.push_str(knuth.as_str());
		newexpr.push('\n');
	}

	parse_input_pt1(newexpr);
}

fn parse_input_pt1(file: String) {

	#[derive(Copy, Clone, Debug)]
	enum State {
		BuildingExpLhs,
		BuildingExpOp,
		BuildingExpRhs,
		BuildingExpFin
	}

	let regex = Regex::new(r"(?<openpr>\()|(?<digit>\d+)|(?<sum>\+)|(?<prod>\*)|(?<clsepr>\))").unwrap();

	let mut sum = 0i64;

	for line in file.lines() {
		let mut expression_stack = LinkedList::<ExpressionBuilder>::new();
		let mut current_expr     = ExpressionBuilder::new();
		let mut built_expr       = Expression::default();
		let mut state            = State::BuildingExpLhs;

		let matches: Vec<_> = regex.find_iter(line).map(|x| { x.as_str() }).collect();

		/* State machine! */
		for mtch in matches {
			match mtch {
				"(" => {
					state = match state {
						State::BuildingExpLhs | State::BuildingExpRhs => {
							let mut moved_expr = ExpressionBuilder::new();
							std::mem::swap(&mut current_expr, &mut moved_expr);
							expression_stack.push_back(moved_expr);
							State::BuildingExpLhs
						},
						_ => panic!("Invalid Syntax.")
					}
				}
				")" => {
					state = match state {
						State::BuildingExpFin => {
							let mut moved_built = Expression::default();
							std::mem::swap(&mut moved_built, &mut built_expr);

							let mut popped_expr = expression_stack.pop_back().unwrap();

							if !popped_expr.has_lhs() {
								popped_expr = popped_expr
									.with_lhs(SubExpression::Direct(0))
									.with_op (Operation::Sum)
							}

							let mut popped_expr =
								popped_expr
									.with_rhs(SubExpression::Parenthetaized(Rc::new(moved_built)))
									.build();

							std::mem::swap(&mut popped_expr, &mut built_expr);

							State::BuildingExpFin
						},
						_ => panic!("Invalid Syntax.")
					}
				}
				"*" => {
					state = match state {
						State::BuildingExpOp => {
							current_expr = current_expr
								.with_op(Operation::Product);
							State::BuildingExpRhs
						},
						State::BuildingExpFin => {
							let mut moved_built = Expression::default();
							std::mem::swap(&mut moved_built, &mut built_expr);
							current_expr = ExpressionBuilder::new()
								.with_lhs(SubExpression::Parenthetaized(Rc::new(moved_built)))
								.with_op (Operation::Product);
							State::BuildingExpRhs
						}
						_ => panic!("Invalid Syntax.")
					}
				}
				"+" => {
					state = match state {
						State::BuildingExpOp => {
							current_expr = current_expr
								.with_op(Operation::Sum);
							State::BuildingExpRhs
						},
						State::BuildingExpFin => {
							let mut moved_built = Expression::default();
							std::mem::swap(&mut moved_built, &mut built_expr);
							current_expr = ExpressionBuilder::new()
								.with_lhs(SubExpression::Parenthetaized(Rc::new(moved_built)))
								.with_op (Operation::Sum);
							State::BuildingExpRhs
						}
						_ => panic!("Invalid Syntax.")
					}
				}
				_ => {
					let Ok(digit) = mtch.parse::<i64>() else {
						panic!("Invalid Number.");
					};
					state = match state {
						State::BuildingExpLhs => {
							current_expr = current_expr
								.with_lhs(SubExpression::Direct(digit));
							State::BuildingExpOp
						},
						State::BuildingExpRhs => {
							current_expr = current_expr
								.with_rhs(SubExpression::Direct(digit));
							let mut moved_expr = ExpressionBuilder::new();
							std::mem::swap(&mut moved_expr, &mut current_expr);
							built_expr = moved_expr.build();
							State::BuildingExpFin
						}
						_ => panic!("Invalid Syntax.")
					}
				}
			}
		}

		sum += built_expr.execute();
	}

	println!("final result is {sum}");
}


#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Operation {
	#[default]
	Sum,
	Product
}

#[derive(PartialEq, Debug)]
enum SubExpression {
	Direct(i64),
	Parenthetaized(Rc<Expression>)
}

impl Default for SubExpression {
	fn default() -> Self {
		Self::Direct(0)
	}
}

#[derive(PartialEq, Debug, Default)]
struct Expression {
	lhs: SubExpression,
	op:  Operation,
	rhs: SubExpression
}

#[allow(dead_code)]
impl Expression {
	fn do_op(op: Operation, lhs: i64, rhs: i64) -> i64 {
		match op {
			Operation::Sum     => lhs + rhs,
			Operation::Product => lhs * rhs
		}
	}
	fn execute(&self) -> i64 {
		match (&self.lhs, &self.rhs) {
			(SubExpression::Direct(x),         SubExpression::Direct(y))         => Expression::do_op(self.op, *x, *y),
			(SubExpression::Parenthetaized(x), SubExpression::Direct(y))         => Expression::do_op(self.op,  x.execute(), *y),
			(SubExpression::Direct(x),         SubExpression::Parenthetaized(y)) => Expression::do_op(self.op, *x, y.execute()),
			(SubExpression::Parenthetaized(x), SubExpression::Parenthetaized(y)) => Expression::do_op(self.op, x.execute(), y.execute())
		}
	}
}

struct ExpressionBuilder {
	lhs: Option<SubExpression>,
	op:  Option<Operation>,
	rhs: Option<SubExpression>
}

impl ExpressionBuilder {
	fn new() -> Self {
		Self { lhs: None, op: None, rhs: None }
	}
	fn with_lhs(mut self, lhs: SubExpression) -> Self {
		self.lhs = Some(lhs);
		return self;
	}
	fn with_op(mut self, op: Operation) -> Self {
		self.op = Some(op);
		return self;
	}
	fn with_rhs(mut self, rhs: SubExpression) -> Self {
		self.rhs = Some(rhs);
		return self;
	}
	fn build(self) -> Expression {
		Expression {
			lhs: self.lhs.unwrap(),
			op:  self.op .unwrap(),
			rhs: self.rhs.unwrap()
		}
	}
	fn has_lhs(&mut self) -> bool {
		if self.lhs == None {
			return false;
		} else {
			return true;
		}
	}
}
