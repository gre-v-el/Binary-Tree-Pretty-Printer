use std::fmt::Display;

use crate::{FULL_BOX, HORIZONTAL_LINE, VERTICAL_LINE};

struct BSNode<T> {
	left: Option<usize>,
	right: Option<usize>,
	value: T,
}

impl<T> BSNode<T> where T : PartialOrd {
	pub fn new(v: T) -> Self {
		Self { left: None, right: None, value: v }
	}

}

pub struct BSTree<T> {
	nodes: Vec<BSNode<T>>,
}

impl<T> BSTree<T> where T : PartialOrd + Display {
	pub fn new() -> Self {
		Self{
			nodes: Vec::new(),
		}
	}

	pub fn insert(&mut self, v: T) {
		if self.nodes.len() == 0 {
			self.nodes.push(BSNode::new(v));
		}
		else {
			self.insert_into_node(0, v);
		}
	}

	fn insert_into_node(&mut self, node: usize, v: T) {
		if v >= self.nodes[node].value {
			if let Some(right) = self.nodes[node].right {
				self.insert_into_node(right, v);
			}
			else {
				self.nodes[node].right = Some(self.nodes.len());
				self.nodes.push(BSNode::new(v));
			}
		}
		else {
			if let Some(left) = self.nodes[node].left {
				self.insert_into_node(left, v);
			}
			else {
				self.nodes[node].left = Some(self.nodes.len());
				self.nodes.push(BSNode::new(v));
			}
		}
	}

	pub fn as_visual(&self) -> String {
		if self.nodes.len() == 0 { " (empty) ".to_owned() }
		else {
			let mut vis = Vec::new();
			let mut prefix = "".into();
			self.append_to_visual(0, &mut vis, 0, 0, None, &mut prefix);

			vis = vis.into_iter().map(|s| // TODO: figure out how to not reverse it, but iterate from the end in the loop
				s.chars().rev().collect()
			).collect();

			let mut str = String::new();

			

			let mut all_nones = false;
			while !all_nones {
				all_nones = true;
				for c in &mut vis {
					if let Some(ch) = c.pop() {
						str.push(ch);
						all_nones = false;
					}
					else {
						str.push(' ');
					}
				}
				str.push('\n');
			}

			str
		}

	}

	pub fn append_to_visual(&self, node: usize, visual: &mut Vec<String>, index: usize, depth: usize, is_right: Option<bool>, prefix: &mut String) {
		const SIZE: usize = 2;
		
		let symbol = if let Some(side) = is_right {
			if side { FULL_BOX[0][2] }
			else { FULL_BOX[0][0] }
		}
		else {""};

		let mut text = format!("{}", self.nodes[node].value);
		let len = text.len();
		let left_symbols = len/2;
		let right_symbols = len-1-left_symbols;

		for _ in 0..right_symbols {
			if let Some(false) = is_right {
				visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if is_right.is_none() {2} else {1}), " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
			}
			
		}
		visual.insert(index, format!("{}{}{}{}", prefix, symbol, VERTICAL_LINE.repeat(if is_right.is_some() {SIZE-1} else {SIZE}), text.pop().unwrap()));

		for _ in 0..left_symbols {
			if let Some(true) = is_right {
				visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if is_right.is_none() {2} else {1}), " ".repeat(if is_right.is_some() {SIZE-1} else {0}), text.pop().unwrap()));
			}
		}


		if let Some(right) = self.nodes[node].right {
			if let Some(false) = is_right {
				prefix.push_str(HORIZONTAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_visual(right, visual, index+len, depth+1, Some(true), prefix);
			for _ in 0..SIZE {
				prefix.pop();
			}
		}
		if let Some(left) = self.nodes[node].left {
			if let Some(true) = is_right {
				prefix.push_str(HORIZONTAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_visual(left, visual, index, depth+1, Some(false), prefix);
			for _ in 0..SIZE {
				prefix.pop();
			}
		}
	}

	pub fn as_string(&self) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { 
			let mut prefix = "".into();
			let mut str = String::new();
			self.append_to_string(0, &mut str, 0, None, &mut prefix);
			str
		}
	}

	fn append_to_string(&self, node: usize, str: &mut String, depth: usize, is_right: Option<bool>, prefix: &mut String) {
		const SIZE: usize = 5;

		if let Some(left) = self.nodes[node].left {
			if let Some(true) = is_right {
				prefix.push_str(VERTICAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_string(left, str, depth+1, Some(false), prefix);
			
			for _ in 0..SIZE {
				prefix.pop();
			}
		}

		let mut symbol = 
		if let Some(is_right) = is_right {
			if is_right {
				FULL_BOX[2][0].to_owned()
			}
			else {
				FULL_BOX[0][0].to_owned()
			}
		}
		else {
			" ".to_owned()
		};
		symbol.push_str(HORIZONTAL_LINE.repeat(SIZE - 1).as_str());
		str.push_str(format!("\n{prefix}{symbol}{}", self.nodes[node].value).as_str());

		if let Some(right) = self.nodes[node].right {
			if let Some(false) = is_right {
				prefix.push_str(VERTICAL_LINE);
				prefix.push_str(" ".repeat(SIZE-1).as_str());
			}
			else {
				prefix.push_str(" ".repeat(SIZE).as_str());
			}
			self.append_to_string(right, str, depth+1, Some(true), prefix);
			
			for _ in 0..SIZE {
				prefix.pop();
			}
		}
	}

	pub fn as_inorder_string(&self) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { self.inorder(0, String::new()) }
	}
	
	fn inorder(&self, node: usize, str: String) -> String {
		
		let mut string = if let Some(left) = self.nodes[node].left {
			self.inorder(left, str)
		} 
		else {
			str
		};
				
		string = format!("{string} {}", self.nodes[node].value);

		if let Some(right) = self.nodes[node].right {
			string = self.inorder(right, string);
		}

		string
	}
}
