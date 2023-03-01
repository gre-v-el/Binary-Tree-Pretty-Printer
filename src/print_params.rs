use std::{num::NonZeroUsize, fmt::Debug};

use crate::{VERTICAL_LINE, HORIZONTAL_LINE, FULL_BOX};

pub struct PrintParams<T> {
	pub convert_to_string: Box<dyn Fn(&T, &[&T]) -> String>, // item -> string
	pub split: Box<dyn Fn(&T, &[&T]) -> usize>, // value, children -> nodes_to_the_left
	pub size: NonZeroUsize,
	pub vertical_line: char,
	pub horizontal_line: char,
	pub right_top_corner: char,
	pub left_top_corner: char,
	pub left_bottom_corner: char,
	pub top_junction: char,
	pub left_junction: char,
}

impl<T> Default for PrintParams<T> where T : Debug {
	fn default() -> Self {
		Self {
			convert_to_string: Box::new(|v, _| { format!("<{:?}>", v) }),
			split: Box::new(|_, children| { children.len()/2 }),
			size: NonZeroUsize::new(2).unwrap(),
			vertical_line: VERTICAL_LINE,
			horizontal_line: HORIZONTAL_LINE,
			right_top_corner: FULL_BOX[0][2],
			left_top_corner: FULL_BOX[0][0],
			left_bottom_corner: FULL_BOX[2][0],
			top_junction: FULL_BOX[0][1],
			left_junction: FULL_BOX[1][0],
		}
	}
}