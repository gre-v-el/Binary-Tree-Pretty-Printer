use std::fmt::Display;

use crate::tree::Tree;

pub struct BSTree<T> where T : PartialOrd + Display {
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

	pub fn vertical_string(&self) -> String {
		self.tree.vertical_string()
	}

	pub fn horizontal_string(&self) -> String {
		self.tree.horizontal_string()
	}
	
	// inorder, postorder, inorder -> Vec<T>
}
