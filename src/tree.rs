use std::fmt::Display;

use crate::{FULL_BOX, HORIZONTAL_LINE, VERTICAL_LINE};


struct Node<T> {
	children: Vec<usize>,
	value: T,
}

impl<T> Node<T> where T : PartialOrd {
	pub fn new(v: T) -> Self {
		Self { children: Vec::new(), value: v }
	}

}

pub struct Tree<T> {
	nodes: Vec<Node<T>>,
}

impl<T> Tree<T> where T : PartialOrd + Display {
	pub fn new() -> Self {
		Self{
			nodes: Vec::new(),
		}
	}

	pub fn insert_into_node(&mut self, node: usize, v: T) {
		let count = self.nodes.len();

		if count != 0 { // TODO: rethink it
			self.nodes[node].children.push(count);
		}

		self.nodes.push(Node::new(v));
	}

	// pub fn as_visual(&self) -> String {
	// 	if self.nodes.len() == 0 { " (empty) ".to_owned() }
	// 	else {
	// 		let mut vis = Vec::new();
	// 		let mut prefix = "".into();
	// 		self.append_to_visual(0, &mut vis, 0, 0, None, &mut prefix);

	// 		vis = vis.into_iter().map(|s| // TODO: figure out how to not reverse it, but iterate from the end in the loop
	// 			s.chars().rev().collect()
	// 		).collect();

	// 		let mut str = String::new();

			

	// 		let mut all_nones = false;
	// 		while !all_nones {
	// 			all_nones = true;
	// 			for c in &mut vis {
	// 				if let Some(ch) = c.pop() {
	// 					str.push(ch);
	// 					all_nones = false;
	// 				}
	// 				else {
	// 					str.push(' ');
	// 				}
	// 			}
	// 			str.push('\n');
	// 		}

	// 		str
	// 	}

	// }

	// pub fn append_to_visual(&self, node: usize, visual: &mut Vec<String>, index: usize, depth: usize, is_right: Option<bool>, prefix: &mut String) {
	// 	const SIZE: usize = 2;
		
	// 	let symbol = if let Some(side) = is_right {
	// 		if side { FULL_BOX[0][2] }
	// 		else { FULL_BOX[0][0] }
	// 	}
	// 	else {""};

	// 	let mut text = format!("{}", self.nodes[node].value);
	// 	let len = text.len();
	// 	let left_symbols = len/2;
	// 	let right_symbols = len-1-left_symbols;

	// 	for _ in 0..right_symbols {
	// 		if let Some(false) = is_right {
	// 			visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
	// 		}
	// 		else {
	// 			visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if is_right.is_none() {2} else {1}), " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
	// 		}
			
	// 	}
	// 	visual.insert(index, format!("{}{}{}{}", prefix, symbol, VERTICAL_LINE.repeat(if is_right.is_some() {SIZE-1} else {SIZE}), text.pop().unwrap()));

	// 	for _ in 0..left_symbols {
	// 		if let Some(true) = is_right {
	// 			visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
	// 		}
	// 		else {
	// 			visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if is_right.is_none() {2} else {1}), " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
	// 		}
	// 	}


	// 	if let Some(right) = self.nodes[node].right {
	// 		if let Some(false) = is_right {
	// 			prefix.push_str(HORIZONTAL_LINE);
	// 			prefix.push_str(" ".repeat(SIZE-1).as_str());
	// 		}
	// 		else {
	// 			prefix.push_str(" ".repeat(SIZE).as_str());
	// 		}
	// 		self.append_to_visual(right, visual, index+len, depth+1, Some(true), prefix);
	// 		for _ in 0..SIZE {
	// 			prefix.pop();
	// 		}
	// 	}
	// 	if let Some(left) = self.nodes[node].left {
	// 		if let Some(true) = is_right {
	// 			prefix.push_str(HORIZONTAL_LINE);
	// 			prefix.push_str(" ".repeat(SIZE-1).as_str());
	// 		}
	// 		else {
	// 			prefix.push_str(" ".repeat(SIZE).as_str());
	// 		}
	// 		self.append_to_visual(left, visual, index, depth+1, Some(false), prefix);
	// 		for _ in 0..SIZE {
	// 			prefix.pop();
	// 		}
	// 	}
	// }

	pub fn as_string(&self) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { 
			let mut prefix = "".into();
			let mut str = String::new();
			self.append_to_string(0, &mut str, 0, [true, false], &mut prefix);
			str
		}
	}

	// position: [is_right, is_extreme]
	fn append_to_string(&self, node: usize, str: &mut String, depth: usize, position: [bool; 2], prefix: &mut String) {
		const SIZE: usize = 5;

		let before = self.nodes[node].children.len()/2;

		for i in 0..before {
			if position != [false, true] {
				prefix.push_str(VERTICAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_string(self.nodes[node].children[i], str, depth+1, if i == 0 {[false, true]} else {[false, false]}, prefix);
			
			for _ in 0..SIZE {
				prefix.pop();
			}
		}

		let mut symbol = 
			match position {
				[true, true] => FULL_BOX[2][0].to_owned(),
				[false, true] => FULL_BOX[0][0].to_owned(),
				[_, false] => FULL_BOX[1][0].to_owned(),
			};
		symbol.push_str(HORIZONTAL_LINE.repeat(SIZE - 1).as_str());
		str.push_str(format!("\n{prefix}{symbol}{}", self.nodes[node].value).as_str());


		for i in before..self.nodes[node].children.len() {
			if position != [true, true] {
				prefix.push_str(VERTICAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_string(self.nodes[node].children[i], str, depth+1, if i == self.nodes[node].children.len() - 1 {[true, true]} else {[true, false]}, prefix);
			
			for _ in 0..SIZE {
				prefix.pop();
			}
		}
	}
}
