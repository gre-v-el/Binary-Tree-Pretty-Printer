use std::{fmt::Display, collections::HashSet};

const FULL_BOX: [[&str; 3]; 3] = [
	["\u{250F}", "\u{2533}", "\u{2513}"],
	["\u{2523}", "\u{254B}", "\u{252B}"],
	["\u{2517}", "\u{253B}", "\u{251B}"],
];
const VERTICAL_LINE: &str = "\u{2503}";
const HORIZONTAL_LINE: &str = "\u{2501}";


struct Node<T> {
	left: Option<usize>,
	right: Option<usize>,
	value: T,
}

impl<T> Node<T> where T : PartialOrd {
	pub fn new(v: T) -> Self {
		Self { left: None, right: None, value: v }
	}

}

struct Tree<T> {
	nodes: Vec<Node<T>>,
}

impl<T> Tree<T> where T : PartialOrd + Display {
	pub fn new() -> Self {
		Self{
			nodes: Vec::new(),
		}
	}

	pub fn insert(&mut self, v: T) {
		if self.nodes.len() == 0 {
			self.nodes.push(Node::new(v));
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
				self.nodes.push(Node::new(v));
			}
		}
		else {
			if let Some(left) = self.nodes[node].left {
				self.insert_into_node(left, v);
			}
			else {
				self.nodes[node].left = Some(self.nodes.len());
				self.nodes.push(Node::new(v));
			}
		}
	}

	pub fn as_visual(&self) -> String {
		if self.nodes.len() == 0 { " (empty) ".to_owned() }
		else {
			let mut vis = Vec::new();
			let mut levels = HashSet::new();
			self.append_to_visual(0, &mut vis, 0, 0, None, &mut levels);

			let mut str = String::new();

			let mut r = 0;
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
					r += 1;
				}
				str.push('\n');
			}

			str
		}

	}

	pub fn append_to_visual(&self, node: usize, visual: &mut Vec<String>, index: usize, depth: usize, is_right: Option<bool>, levels: &mut HashSet<usize>) {
		const SIZE: usize = 1;
		
		let symbol = if let Some(side) = is_right {
			if side { FULL_BOX[0][2] }
			else { FULL_BOX[0][0] }
		}
		else {""};

		let mut space = String::new();
		// for i in (0..depth).rev() {
		// 	if levels.contains(&i) {
		// 		space.push_str(HORIZONTAL_LINE);
		// 		space.push_str(" ".repeat(SIZE.max(1)-1).as_str());
		// 	}
		// 	else {
		// 		space.push_str(" ".repeat(SIZE).as_str());
		// 	}
		// }
		for i in 0..(depth.max(1)-1) {
			if levels.contains(&i) {
				space.push_str(HORIZONTAL_LINE);
				space.push_str(" ".repeat(SIZE.max(1)-1).as_str());
			}
			else {
				space.push_str(" ".repeat(SIZE).as_str());
			}
		}
		space = space.chars().rev().collect();

		visual.insert(index, format!("{}{}{}{}", "o", VERTICAL_LINE.repeat(if is_right.is_some() {SIZE-1} else {0}), symbol, space));
		if let Some(right) = self.nodes[node].right {
			if let Some(false) = is_right {
				levels.insert(depth-1);
			}
			self.append_to_visual(right, visual, index+1, depth+1, Some(true), levels);
			if let Some(false) = is_right {
				levels.remove(&(depth-1));
			}
		}
		if let Some(left) = self.nodes[node].left {
			if let Some(true) = is_right {
				levels.insert(depth-1);
			}
			self.append_to_visual(left, visual, index, depth+1, Some(false), levels);
			if let Some(true) = is_right {
				levels.remove(&(depth-1));
			}
		}
	}

	pub fn as_string(&self) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { 
			let mut levels = HashSet::new();
			let mut str = String::new();
			self.append_to_string(0, &mut str, 0, None, &mut levels);
			str
		}
	}

	fn append_to_string(&self, node: usize, str: &mut String, depth: usize, is_right: Option<bool>, levels: &mut HashSet<usize>) {
		const SIZE: usize = 5;
	
		let mut s = String::new();
		for i in 0..(depth.max(1)-1) {
			if levels.contains(&i) {
				s.push_str(VERTICAL_LINE);
				s.push_str(" ".repeat(SIZE.max(1)-1).as_str());
			}
			else {
				s.push_str(" ".repeat(SIZE).as_str());
			}
		}

		if let Some(left) = self.nodes[node].left {
			if let Some(true) = is_right {
				levels.insert(depth-1);
			}
			self.append_to_string(left, str, depth+1, Some(false), levels);
			if let Some(true) = is_right {
				levels.remove(&(depth-1));
			}
		}

		if let Some(is_right) = is_right {
			if is_right {
				s.push_str(FULL_BOX[2][0]);
			}
			else {
				s.push_str(FULL_BOX[0][0]);
			}
			s.push_str(HORIZONTAL_LINE.repeat(SIZE.max(1) - 1).as_str());
		}
		str.push_str(format!("\n{s}<{}>", self.nodes[node].value).as_str());

		if let Some(right) = self.nodes[node].right {
			if let Some(false) = is_right {
				levels.insert(depth-1);
			}
			self.append_to_string(right, str, depth+1, Some(true), levels);
			if let Some(false) = is_right {
				levels.remove(&(depth-1));
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

fn main() {
	let mut tree: Tree<u8> = Tree::new();

	for i in 0..50 {
		tree.insert(rand::random());
	}



    println!("{}", tree.as_string());
    println!("{}", tree.as_visual());
	println!("{}", "\n".repeat(3));
    println!("{}", tree.as_inorder_string());
}
