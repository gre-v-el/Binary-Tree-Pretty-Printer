use bst::binary_search_tree::BSTree;
use bst::tree::Tree;


fn main() {
	let mut tree: BSTree<u16> = BSTree::new();

	for _ in 0..20 {
		tree.insert(rand::random());
	}
	println!("{}", tree.horizontal_string());
	println!("{}", tree.vertical_string());


	// let mut tree: Tree<u16> = Tree::with_root(0);
	// for i in 0..30 {
	// 	tree.insert_into_node(rand::random::<usize>() % tree.nodes_count(), i).unwrap();
	// }

	// println!("{}", tree.horizontal_string());
	// println!("{}", tree.vertical_string());

}
