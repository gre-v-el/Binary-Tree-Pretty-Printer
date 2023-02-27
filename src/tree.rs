use std::{fmt::Display, error::Error, mem::replace};

use crate::{FULL_BOX, HORIZONTAL_LINE, VERTICAL_LINE};

#[derive(Debug)]
pub struct NodeIndexError {
	given: usize,
	allowed: usize,
}

impl NodeIndexError {
	pub fn new(given: usize, allowed: usize) -> Self {
		Self {
			given, allowed
		}
	}
}

impl Display for NodeIndexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(format!("Node index {} outside bounds [0-{})", self.given, self.allowed).as_str(), f)
	}
}
impl Error for NodeIndexError {}

#[derive(PartialEq, PartialOrd)]
enum NodePosition {
	LeftExtreme,
	Left,
	Right,
	RightExtreme,
	Root,
}

impl NodePosition {
	pub fn left(extreme: bool) -> Self {
		match extreme {
			true => Self::LeftExtreme,
			false => Self::Left
		}
	}

	pub fn right(extreme: bool) -> Self {
		match extreme {
			true => Self::RightExtreme,
			false => Self::Right
		}
	}
}

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

	pub fn with_root(v: T) -> Self {
		Self{
			nodes: vec![Node::new(v)],
		}
	}

	pub fn nodes_count(&self) -> usize {
		self.nodes.len()
	}

	pub fn set_root(&mut self, v: T) -> Option<T> {
		if self.nodes.len() == 0 {
			self.nodes.push(Node::new(v));
			None
		}
		else {
			Some(replace(&mut self.nodes[0], Node::new(v)).value)
		}
	}

	pub fn insert_into_node(&mut self, node: usize, v: T) -> Result<usize, NodeIndexError> {
		let new_index = self.nodes_count();
		
		match self.nodes.get_mut(node) {
			Some(node) => {
				node.children.push(new_index);
				self.nodes.push(Node::new(v));
				Ok(new_index)
			}
			None => {
				Err(NodeIndexError::new(node, self.nodes_count()))
			}
		}

	}

	pub fn vertical_string(&self) -> String {
		if self.nodes.len() == 0 { " (empty) ".to_owned() }
		else {
			let mut vis = Vec::new();
			let mut prefix = "".into();
			self.vertical_recursive(0, &mut vis, 0, NodePosition::Root, &mut prefix);

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

	fn vertical_recursive(&self, node: usize, visual: &mut Vec<String>, index: usize, position: NodePosition, prefix: &mut String) {
		const SIZE: usize = 2;
		
		let symbol =
			match position {
				NodePosition::RightExtreme => FULL_BOX[0][0].to_owned(),
				NodePosition::LeftExtreme => FULL_BOX[0][2].to_owned(),
				NodePosition::Root => " ".to_owned(),
				_ => FULL_BOX[0][1].to_owned(),
			};

		let mut text = format!("<{}>", self.nodes[node].value);
		let len = text.len();
		let left_symbols = len/2;
		let right_symbols = len-1-left_symbols;

		for _ in 0..right_symbols {
			if position != NodePosition::LeftExtreme && position != NodePosition::Root {
				visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(SIZE-1), text.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if position == NodePosition::Root {2} else {1}), " ".repeat(if position != NodePosition::Root {SIZE-1} else {0}), text.pop().unwrap()));
			}
			
		}
		visual.insert(index, format!("{}{}{}{}", prefix, symbol, VERTICAL_LINE.repeat(SIZE-1), text.pop().unwrap()));

		for _ in 0..left_symbols {
			if position != NodePosition::RightExtreme && position != NodePosition::Root {
				visual.insert(index, format!("{}{}{}{}", prefix, HORIZONTAL_LINE, " ".repeat(SIZE-1), text.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if position == NodePosition::Root {2} else {1}), " ".repeat(if position != NodePosition::Root {SIZE-1} else {0}), text.pop().unwrap()));
			}
		}

		let children = self.nodes[node].children.len();
		let left = children/2;
		
		if position != NodePosition::LeftExtreme && position != NodePosition::Root {
			prefix.push_str(HORIZONTAL_LINE);
			prefix.push_str(" ".repeat(SIZE-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(SIZE).as_str());
		}
		for i in 0..left {
			self.vertical_recursive(self.nodes[node].children[i], visual, index+len, NodePosition::left(i==0), prefix);
		}
		for _ in 0..SIZE {
			prefix.pop();
		}

		if position != NodePosition::RightExtreme && position != NodePosition::Root {
			prefix.push_str(HORIZONTAL_LINE);
			prefix.push_str(" ".repeat(SIZE-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(SIZE).as_str());
		}
		for i in left..children {
			self.vertical_recursive(self.nodes[node].children[i], visual, index, NodePosition::right(i==children-1), prefix);
		}
		for _ in 0..SIZE {
			prefix.pop();
		}
	}

	pub fn horizontal_string(&self) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { 
			let mut prefix = "".into();
			let mut str = String::new();
			self.horizontal_recursive(0, &mut str, NodePosition::Root, &mut prefix);
			str
		}
	}

	fn horizontal_recursive(&self, node: usize, str: &mut String, position: NodePosition, prefix: &mut String) {
		const SIZE: usize = 5;

		let before = self.nodes[node].children.len()/2;

		if position != NodePosition::LeftExtreme && position != NodePosition::Root {
			prefix.push_str(VERTICAL_LINE);
			prefix.push_str(" ".repeat(SIZE-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(SIZE).as_str());
		}
		for i in 0..before {
			self.horizontal_recursive(self.nodes[node].children[i], str, NodePosition::left(i==0), prefix);
		}
		for _ in 0..SIZE {
			prefix.pop();
		}

		let mut symbol = 
			match position {
				NodePosition::RightExtreme => FULL_BOX[2][0].to_owned(),
				NodePosition::LeftExtreme => FULL_BOX[0][0].to_owned(),
				NodePosition::Root => " ".to_owned(),
				_ => FULL_BOX[1][0].to_owned(),
			};
		symbol.push_str(HORIZONTAL_LINE.repeat(SIZE - 1).as_str());
		str.push_str(format!("\n{prefix}{symbol}{}", self.nodes[node].value).as_str());

		
		if position != NodePosition::RightExtreme && position != NodePosition::Root {
			prefix.push_str(VERTICAL_LINE);
			prefix.push_str(" ".repeat(SIZE-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(SIZE).as_str());
		}
		for i in before..self.nodes[node].children.len() {
			self.horizontal_recursive(self.nodes[node].children[i], str, NodePosition::right(i == self.nodes[node].children.len() - 1), prefix);
		}
		for _ in 0..SIZE {
			prefix.pop();
		}
	}
}
