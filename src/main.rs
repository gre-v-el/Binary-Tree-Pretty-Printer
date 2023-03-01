use std::num::NonZeroUsize;

use bst::{FULL_BOX, HORIZONTAL_LINE, VERTICAL_LINE};
use bst::binary_search_tree::BSTree;
use bst::print_params::PrintParams;
use bst::tree::Tree;


fn main() {
	let mut tree: BSTree<u8> = BSTree::new();
	
	for _ in 0..20 {
		tree.insert(rand::random());
	}
	let params = PrintParams {
		convert_to_string: Box::new(|v, children| {
			let text = format!("{}", v);
			let len = text.len();

			let children = children.len();
			let lt = if children > 1 {FULL_BOX[1][1]} else {FULL_BOX[0][1]};
			let lb = if children > 0 {FULL_BOX[1][0]} else {FULL_BOX[2][0]};

			format!("{}{}{}\n{}{}{}\n{}{}{}", 
				lt, HORIZONTAL_LINE.to_string().repeat(len), FULL_BOX[0][2],
				VERTICAL_LINE, text, VERTICAL_LINE,
				lb, HORIZONTAL_LINE.to_string().repeat(len), FULL_BOX[2][2],
			)
		}),
		size: NonZeroUsize::new(5).unwrap(),
		..Default::default()
	};
	println!("{}", tree.horizontal_string(&params));
	println!("{}", tree.vertical_string(&params));
	println!("pre:  {:?}", tree.preorder());
	println!("in:   {:?}", tree.inorder());
	println!("post: {:?}", tree.postorder());


	// let mut tree: Tree<u16> = Tree::with_root(0);
	// for i in 0..30 {
	// 	tree.insert_into_node(rand::random::<usize>() % tree.nodes_count(), i).unwrap();
	// }

	// println!("{}", tree.horizontal_string());
	// println!("{}", tree.vertical_string());

}
