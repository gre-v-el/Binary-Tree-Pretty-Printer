mod bstree;

use bstree::Tree;

const FULL_BOX: [[&str; 3]; 3] = [
	["\u{250F}", "\u{2533}", "\u{2513}"],
	["\u{2523}", "\u{254B}", "\u{252B}"],
	["\u{2517}", "\u{253B}", "\u{251B}"],
];
const VERTICAL_LINE: &str = "\u{2503}";
const HORIZONTAL_LINE: &str = "\u{2501}";


fn main() {
	let mut tree: Tree<u16> = Tree::new();

	for _ in 0..30 {
		tree.insert(rand::random());
	}

    println!("{}", tree.as_string());
	println!("{}", "\n".repeat(3));
    println!("{}", tree.as_visual());
	println!("{}", "\n".repeat(3));
    println!("{}", tree.as_inorder_string());
}
