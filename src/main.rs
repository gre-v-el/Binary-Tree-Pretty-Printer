use bst::binary_search_tree::BSTree;
use bst::print_params::PrintParams;
use bst::tree::Tree;


fn main() {
	let mut tree: BSTree<u8> = BSTree::new();

	for _ in 0..20 {
		tree.insert(rand::random());
	}
	println!("{}", tree.vertical_string(&PrintParams::default()));
	println!("{}", tree.horizontal_string(&PrintParams::default()));
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
