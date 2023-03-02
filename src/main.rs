use std::fmt::Debug;
use std::num::NonZeroUsize;

use bst::{FULL_BOX, HORIZONTAL_LINE, VERTICAL_LINE};
use bst::binary_search_tree::BSTree;
use bst::print_params::PrintParams;
use bst::tree::Tree;


fn main() {
	
	choice_sim();

	// let mut tree: Tree<u16> = Tree::with_root(0);
	// for i in 0..30 {
	// 	tree.add_child(rand::random::<usize>() % tree.nodes_count(), i).unwrap();
	// }

	// let mut tree: BSTree<u8> = BSTree::new();
	
	// for _ in 0..20 {
	// 	tree.insert(rand::random());
	// }
	// println!("{}", tree.horizontal_string(&PrintParams {
	// 	convert_to_string: Box::new(|v, children| {
	// 		let text = format!("{}", v);
	// 		let len = text.len();

	// 		let children = children.len();
	// 		let lt = if children > 1 {FULL_BOX[1][1]} else {FULL_BOX[0][1]};
	// 		let lb = if children > 0 {FULL_BOX[1][0]} else {FULL_BOX[2][0]};

	// 		format!("{}{}{}\n{} {} {}\n{}{}{}", 
	// 			lt, HORIZONTAL_LINE.to_string().repeat(len+2), FULL_BOX[0][2],
	// 			VERTICAL_LINE, text, VERTICAL_LINE,
	// 			lb, HORIZONTAL_LINE.to_string().repeat(len+2), FULL_BOX[2][2],
	// 		)
	// 	}),
	// 	size: NonZeroUsize::new(5).unwrap(),
	// 	..Default::default()
	// }));
	// println!("{}", tree.vertical_string(&PrintParams {
	// 	convert_to_string: Box::new(|v, children| {
	// 		let text = format!("{}", v);
	// 		let len = text.len();

	// 		let children = children.len();
	// 		let lt = if children > 0 {FULL_BOX[0][1]} else {FULL_BOX[0][0]};
	// 		let rt = if children > 1 {FULL_BOX[0][1]} else {FULL_BOX[0][2]};

	// 		let left_chars = (len+2)/2;

	// 		format!("{}{}{}{}{}\n{} {} {}\n{}{}{}", 
	// 			lt, HORIZONTAL_LINE.to_string().repeat(left_chars), FULL_BOX[2][1], HORIZONTAL_LINE.to_string().repeat(len-left_chars+1), rt,
	// 			VERTICAL_LINE, text, VERTICAL_LINE,
	// 			FULL_BOX[2][0], HORIZONTAL_LINE.to_string().repeat(len+2), FULL_BOX[2][2],
	// 		)
	// 	}),
	// 	size: NonZeroUsize::new(3).unwrap(),
	// 	..Default::default()
	// }));
	// println!("pre:  {:?}", tree.preorder());
	// println!("in:   {:?}", tree.inorder());
	// println!("post: {:?}", tree.postorder());

}

#[derive(PartialEq)]
pub enum Val {
	Start,
	Me,
	NotMe,
}

impl Debug for Val {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(match self {
			Val::Start => "start",
			Val::Me => "me",
			Val::NotMe => "not me",
		}, f)
	}
}

fn choice_sim() {
	let mut tree = Tree::with_root(Val::Start);

	construct(0, 5, &mut tree);

	println!("{}", tree.horizontal_string(&PrintParams{
		convert_to_string: Box::new(|v, children| {
			let bar = if children.len() > 0 {
				VERTICAL_LINE
			} 
			else {
				' '
			};
			// format!("{:?}\n{}", v, bar)
			format!("{:?}", v)
		}),
		..Default::default()
	}));
}

fn construct(index: usize, depth: usize, tree: &mut Tree<Val>) {
	if depth == 0 {return;}

	if *tree.get_node_value(index).unwrap() == Val::Me {
		construct(tree.add_child(index, Val::NotMe).unwrap(), depth-1, tree);
	}
	else {
		construct(tree.add_child(index, Val::Me).unwrap(), depth-1, tree);
		construct(tree.add_child(index, Val::NotMe).unwrap(), depth-1, tree);
	}
}