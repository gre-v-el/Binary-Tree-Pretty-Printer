use std::num::NonZeroUsize;

pub struct PrintParams<T> {
	pub to_string: Box<dyn Fn(&T) -> String>, // item -> string
	pub split: Box<dyn Fn(&T, &[usize], &[T]) -> usize>, // value, children, all_values -> nodes_to_the_left
	pub size: NonZeroUsize,
	pub vertical_line: char,
	pub horizontal_line: char,
	pub right_top_corner: char,
	pub left_top_corner: char,
	pub left_bottom_corner: char,
	pub top_junction: char,
	pub left_junction: char,
}