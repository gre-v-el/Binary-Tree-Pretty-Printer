use std::{fmt::{Display, Debug}, error::Error, mem::replace};

use crate::{print_params::PrintParams, VERTICAL_LINE};

#[derive(Debug)]
pub struct NodeIndexError<'a> {
	object: &'a str,
	given: usize,
	allowed: usize,
}

impl<'a> NodeIndexError<'a> {
	pub fn new(thing: &'a str, given: usize, allowed: usize) -> Self {
		Self {
			object: thing, given, allowed
		}
	}
}

impl<'a> Display for NodeIndexError<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(format!("{} index {} outside bounds [0-{})", self.object, self.given, self.allowed).as_str(), f)
	}
}
impl<'a> Error for NodeIndexError<'a> {}

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

impl<T> Node<T> {
	pub fn new(v: T) -> Self {
		Self { children: Vec::new(), value: v }
	}
}

// TODO do something with the Display trait (add a to_string closure?)
pub struct Tree<T> where T : Debug {
	nodes: Vec<Node<T>>,
}

impl<T> Tree<T> where T : Debug {
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

	pub fn add_child(&mut self, node: usize, v: T) -> Result<usize, NodeIndexError> {
		let new_index = self.nodes_count();
		
		match self.nodes.get_mut(node) {
			Some(node) => {
				node.children.push(new_index);
				self.nodes.push(Node::new(v));
				Ok(new_index)
			}
			None => {
				Err(NodeIndexError::new("Node", node, new_index))
			}
		}
	}

	pub fn add_child_position(&mut self, node: usize, v: T, pos: usize) -> Result<usize, NodeIndexError> {
		let new_index = self.nodes_count();
		
		match self.nodes.get_mut(node) {
			Some(node) => {
				if pos <= node.children.len() {
					node.children.insert(pos, new_index);
					self.nodes.push(Node::new(v));
					Ok(new_index)
				}
				else {
					Err(NodeIndexError::new("Child", pos, node.children.len()))
				}
			}
			None => {
				Err(NodeIndexError::new("Node", node, new_index))
			}
		}
	}

	pub fn get_node_value(&self, index: usize) -> Option<&T> {
		Some(&self.nodes.get(index)?.value)
	}

	pub fn get_node_children(&self, index: usize) -> Option<&Vec<usize>> {
		Some(&self.nodes.get(index)?.children)
	}

	pub fn vertical_string(&self, params: &PrintParams<T>) -> String {
		if self.nodes.len() == 0 { " (empty) ".to_owned() }
		else {
			let mut vis = Vec::new();
			let mut prefix = "".into();
			self.vertical_recursive(0, &mut vis, 0, NodePosition::Root, &mut prefix, params);

			let mut str = String::new();
			
			let mut all_nones = false;
			let mut i = 0;
			while !all_nones {
				all_nones = true;
				for c in &mut vis {
					if i < c.chars().count() {
						str.push(c.chars().nth(i).unwrap());
						all_nones = false;
					}
					else {
						str.push(' ');
					}
				}
				str.push('\n');
				i += 1;
			}

			str
		}

	}

	fn vertical_recursive(&self, node: usize, visual: &mut Vec<String>, index: usize, position: NodePosition, prefix: &mut String, params: &PrintParams<T>) {
		let children: Vec<&T> = self.get_node_children(node).unwrap().iter().map(|i| self.get_node_value(*i).unwrap()).collect();
		
		let symbol =
			match position {
				NodePosition::RightExtreme => params.left_top_corner,
				NodePosition::LeftExtreme => params.right_top_corner,
				NodePosition::Root => ' ',
				_ => params.top_junction,
			};

		let text = (params.convert_to_string)(&self.nodes[node].value, &children);
		let lines = text.split('\n').collect::<Vec<_>>();
		let mut vertical_strings = Vec::new();
		for i in 0..lines[0].chars().count() {
			let mut string = String::new();
			for s in &lines {
				string.push(s.chars().nth(i).unwrap());
			}
			vertical_strings.push(string);
		}
		let len = vertical_strings.len();
		let left_symbols = len/2;
		let right_symbols = len-1-left_symbols;

		for _ in 0..right_symbols {
			if position != NodePosition::LeftExtreme && position != NodePosition::Root {
				visual.insert(index, format!("{}{}{}{}", prefix, params.horizontal_line, " ".repeat(usize::from(params.size)-1), vertical_strings.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if position == NodePosition::Root {params.size.into()} else {1}), " ".repeat(if position != NodePosition::Root {usize::from(params.size)-1} else {0}), vertical_strings.pop().unwrap()));
			}
		}
		let mut lines = String::new();
		for _ in 0..(usize::from(params.size)-1) {
			lines.push(params.vertical_line)
		}
		visual.insert(index, format!("{}{}{}{}", prefix, symbol, lines, vertical_strings.pop().unwrap()));

		for _ in 0..left_symbols {
			if position != NodePosition::RightExtreme && position != NodePosition::Root {
				visual.insert(index, format!("{}{}{}{}", prefix, params.horizontal_line, " ".repeat(usize::from(params.size)-1), vertical_strings.pop().unwrap()));
			}
			else {
				visual.insert(index, format!("{}{}{}{}", prefix, " ".repeat(if position == NodePosition::Root {params.size.into()} else {1}), " ".repeat(if position != NodePosition::Root {usize::from(params.size)-1} else {0}), vertical_strings.pop().unwrap()));
			}
		}

		let children = self.nodes[node].children.len();
		
		let pass: Vec<&T> = self.get_node_children(node).unwrap().iter().map(|i| self.get_node_value(*i).unwrap()).collect();
		let left = (*params.split)(self.get_node_value(node).unwrap(), &pass);
		
		if position != NodePosition::LeftExtreme && position != NodePosition::Root {
			prefix.push(params.horizontal_line);
			prefix.push_str(" ".repeat(usize::from(params.size)-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(usize::from(params.size)).as_str());
		}
		for i in 0..left {
			self.vertical_recursive(self.nodes[node].children[i], visual, index+len, NodePosition::left(i==0), prefix, params);
		}
		for _ in 0..usize::from(params.size) {
			prefix.pop();
		}

		if position != NodePosition::RightExtreme && position != NodePosition::Root {
			prefix.push(params.horizontal_line);
			prefix.push_str(" ".repeat(usize::from(params.size)-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(usize::from(params.size)).as_str());
		}
		for i in left..children {
			self.vertical_recursive(self.nodes[node].children[i], visual, index, NodePosition::right(i==children-1), prefix, params);
		}
		for _ in 0..usize::from(params.size) {
			prefix.pop();
		}
	}

	pub fn horizontal_string(&self, params: &PrintParams<T>) -> String {
		if self.nodes.len() == 0 { "(empty)".into() }
		else { 
			let mut prefix = "".into();
			let mut str = String::new();
			self.horizontal_recursive(0, &mut str, NodePosition::Root, &mut prefix, params);
			str
		}
	}

	fn horizontal_recursive(&self, node: usize, str: &mut String, position: NodePosition, prefix: &mut String, params: &PrintParams<T>) {

		let children: Vec<&T> = self.get_node_children(node).unwrap().iter().map(|i| self.get_node_value(*i).unwrap()).collect();
		let before = (*params.split)(self.get_node_value(node).unwrap(), &children);

		if position != NodePosition::LeftExtreme && position != NodePosition::Root {
			prefix.push(params.vertical_line);
			prefix.push_str(" ".repeat(usize::from(params.size)-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(usize::from(params.size)).as_str());
		}
		for i in 0..before {
			self.horizontal_recursive(self.nodes[node].children[i], str, NodePosition::left(i==0), prefix, params);
		}
		for _ in 0..usize::from(params.size) {
			prefix.pop();
		}

		let text = (params.convert_to_string)(&self.nodes[node].value, &children);
		let mut lines = text.split('\n').rev().collect::<Vec<_>>();

		let mut symbol: String = 
			match position {
				NodePosition::RightExtreme => params.left_bottom_corner.into(),
				NodePosition::LeftExtreme => params.left_top_corner.into(),
				NodePosition::Root => " ".into(),
				_ => params.left_junction.into(),
			};
		for _ in 0..(usize::from(params.size) - 1) {
			symbol.push(params.horizontal_line);
		}
		str.push_str(format!("\n{prefix}{symbol}{}", lines.pop().unwrap()).as_str());
		while let Some(s) = lines.pop() {
			let line = if position != NodePosition::RightExtreme && position != NodePosition::Root {VERTICAL_LINE} else {' '};
			str.push_str(format!("\n{prefix}{}{}{}", line, " ".repeat(usize::from(params.size) - 1), s).as_str());
		}
		// str.push_str(format!("\n{prefix}{}", params.vertical_line).as_str());

		
		if position != NodePosition::RightExtreme && position != NodePosition::Root {
			prefix.push(params.vertical_line);
			prefix.push_str(" ".repeat(usize::from(params.size)-1).as_str());
		}
		else {
			prefix.push_str(" ".repeat(usize::from(params.size)).as_str());
		}
		for i in before..self.nodes[node].children.len() {
			self.horizontal_recursive(self.nodes[node].children[i], str, NodePosition::right(i == self.nodes[node].children.len() - 1), prefix, params);
		}
		for _ in 0..usize::from(params.size) {
			prefix.pop();
		}
	}
}