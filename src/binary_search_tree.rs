use std::fmt::Display;

use crate::{tree::Tree, print_params::PrintParams};

pub struct BSTree<T> where T : Display {
	tree: Tree<T>,
}

impl<T> BSTree<T> where T : PartialOrd + Display {
	pub fn new() -> Self {
		Self{
			tree: Tree::new(),
		}
	}

	pub fn with_root(v: T) -> Self {
		Self {
			tree: Tree::with_root(v)
		}
	}

	pub fn nodes_count(&self) -> usize {
		self.tree.nodes_count()
	}

	pub fn set_root(&mut self, v: T) -> Option<T> {
		self.tree.set_root(v)
	}

	pub fn get_node_value(&self, index: usize) -> Option<&T> {
		self.tree.get_node_value(index)
	}

	pub fn get_node_children(&self, index: usize) -> Option<&Vec<usize>> {
		self.tree.get_node_children(index)
	}

	pub fn insert(&mut self, v: T) {
		if self.nodes_count() == 0 {
			self.set_root(v);
		}
		else {
			self.insert_into_node(0, v);
		}
	}

	// TODO: make a copy of this function in a separate impl block without PartialOrd but with a comparator closure
	fn insert_into_node(&mut self, node: usize, v: T) {
		let value = self.get_node_value(node).unwrap();
		let children = self.get_node_children(node).unwrap();

		if v >= *value {
			if children.len() == 2 {
				self.insert_into_node(children[1], v);
			}
			else if children.len() == 1 && self.get_node_value(children[0]).unwrap() >= value {
				self.insert_into_node(children[0], v);
			}
			else {
				self.tree.add_child(node, v).unwrap();
			}
		}
		else {
			if children.len() == 2 {
				self.insert_into_node(children[0], v);
			}
			else if children.len() == 1 && self.get_node_value(children[0]).unwrap() < value {
				self.insert_into_node(children[0], v);
			}
			else if children.len() == 1 {
				self.tree.add_child_position(node, v, 0).unwrap();
			}
			else {
				self.tree.add_child(node, v).unwrap();
			}
		}
	}

	// TODO: single children are always printed before the parent, regardless of their value
	// Add some params, including a closure to decide the number of children on each side of a parent
	pub fn vertical_string(&self, params: &PrintParams<T>) -> String {
		self.tree.vertical_string(params)
	}

	pub fn horizontal_string(&self, params: &PrintParams<T>) -> String {
		self.tree.horizontal_string(params)
	}

	pub fn preorder(&self) -> Vec<&T> {
		let mut vec = Vec::new();

		if self.nodes_count() > 0 {
			self.preorder_traversal(0, &mut vec);
		}

		vec
	}
	
	fn preorder_traversal<'a>(&'a self, node: usize, vec: &mut Vec<&'a T>) {
		vec.push(self.get_node_value(node).unwrap());

		for c in self.get_node_children(node).unwrap() {
			self.preorder_traversal(*c, vec);
		}
	}

	pub fn inorder(&self) -> Vec<&T> {
		let mut vec = Vec::new();

		if self.nodes_count() > 0 {
			self.inorder_traversal(0, &mut vec);
		}

		vec
	}
	
	fn inorder_traversal<'a>(&'a self, node: usize, vec: &mut Vec<&'a T>) {
		let children = self.get_node_children(node).unwrap();
		let value = self.get_node_value(node).unwrap();

		if children.len() == 2 {
			self.inorder_traversal(children[0], vec);
			vec.push(value);
			self.inorder_traversal(children[1], vec);
		}
		else if children.len() == 1 && self.get_node_value(children[0]).unwrap() >= value {
			vec.push(value);
			self.inorder_traversal(children[0], vec);
		}
		else if children.len() == 1 {
			self.inorder_traversal(children[0], vec);
			vec.push(value);
		}
		else {
			vec.push(value);
		}
	}

	pub fn postorder(&self) -> Vec<&T> {
		let mut vec = Vec::new();

		if self.nodes_count() > 0 {
			self.postorder_traversal(0, &mut vec);
		}

		vec
	}
	
	fn postorder_traversal<'a>(&'a self, node: usize, vec: &mut Vec<&'a T>) {
		for c in self.get_node_children(node).unwrap() {
			self.postorder_traversal(*c, vec);
		}

		vec.push(self.get_node_value(node).unwrap());
	}
}
