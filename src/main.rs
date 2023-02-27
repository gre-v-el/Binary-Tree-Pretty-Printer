mod binary_search_tree;
mod tree;

use binary_search_tree::BSTree;
use tree::Tree;

const FULL_BOX: [[&str; 3]; 3] = [
	["\u{250F}", "\u{2533}", "\u{2513}"],
	["\u{2523}", "\u{254B}", "\u{252B}"],
	["\u{2517}", "\u{253B}", "\u{251B}"],
];
const VERTICAL_LINE: &str = "\u{2503}";
const HORIZONTAL_LINE: &str = "\u{2501}";


fn main() {
	// let mut tree: BSTree<u16> = BSTree::new();

	// for _ in 0..30 {
	// 	tree.insert(rand::random());
	// }

    // println!("{}", tree.as_string());
	// println!("{}", "\n".repeat(3));
    // println!("{}", tree.as_visual());
	// println!("{}", "\n".repeat(3));
    // println!("{}", tree.as_inorder_string());


	let mut tree: Tree<u16> = Tree::with_root(0);
	tree.insert_into_node(0, 1);
	tree.insert_into_node(0, 2);
	tree.insert_into_node(0, 3);
	tree.insert_into_node(0, 4);
	tree.insert_into_node(1, 5);
	tree.insert_into_node(1, 6);
	tree.insert_into_node(1, 7);
	tree.insert_into_node(1, 8);
	tree.insert_into_node(1, 9);
	tree.insert_into_node(2, 10);
	tree.insert_into_node(2, 11);
	tree.insert_into_node(2, 12);
	tree.insert_into_node(2, 13);
	tree.insert_into_node(2, 14);
	tree.insert_into_node(2, 15);
	tree.insert_into_node(2, 16);
	tree.insert_into_node(3, 17);
	tree.insert_into_node(3, 18);
	tree.insert_into_node(3, 19);
	tree.insert_into_node(3, 20);
	tree.insert_into_node(4, 21);
	tree.insert_into_node(4, 22);
	tree.insert_into_node(4, 23);
	tree.insert_into_node(5, 24);
	tree.insert_into_node(6, 25);
	tree.insert_into_node(7, 26);
	tree.insert_into_node(8, 27);
	tree.insert_into_node(9, 28);

	println!("{}", tree.horizontal_string());
	println!("{}", tree.vertical_string());

}
