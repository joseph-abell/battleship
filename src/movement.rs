use crate::Cell;
use crate::Direction;
use crate::Rotation;

use Cell::{Crosshair, Empty, Placeholder};

pub fn move_ship(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	rotation: &Rotation,
	ship_size: &usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			if is_free_space(&board, pos_x as isize - 1, pos_y as isize, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_x -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Right => {
			if is_free_space(&board, pos_x as isize + 1, pos_y as isize, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_x += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Up => {
			if is_free_space(&board, pos_x as isize, pos_y as isize - 1, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_y -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
		Direction::Down => {
			if is_free_space(&board, pos_x as isize, pos_y as isize + 1, ship_size, rotation) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Empty);
				pos_y += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &rotation, &ship_size, Placeholder);
			}
		}
	};

	(board, pos_x, pos_y)
}

pub fn move_crosshair(
	mut board: [[Cell; 10]; 10],
	mut pos_x: usize,
	mut pos_y: usize,
	direction: Direction,
) -> ([[Cell; 10]; 10], usize, usize) {
	match direction {
		Direction::Left => {
			if is_free_space(&board, pos_x as isize - 1, pos_y as isize, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Empty);
				pos_x -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Crosshair);
			}
		}
		Direction::Right => {
			if is_free_space(&board, pos_x as isize + 1, pos_y as isize, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Empty);
				pos_x += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Crosshair);
			}
		}
		Direction::Up => {
			if is_free_space(&board, pos_x as isize, pos_y as isize - 1, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Empty);
				pos_y -= 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Crosshair);
			}
		}
		Direction::Down => {
			if is_free_space(&board, pos_x as isize, pos_y as isize + 1, &1, &Rotation::Horizontal) {
				// clear previous position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Empty);
				pos_y += 1;
				// set new position
				board = place_entity(board, pos_x, pos_y, &Rotation::Horizontal, &1, Crosshair);
			}
		}
	};

	(board, pos_x, pos_y)
}

pub fn place_entity(
	mut board: [[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
	rotation: &Rotation,
	ship_size: &usize,
	cell: Cell,
) -> [[Cell; 10]; 10] {
	match rotation {
		Rotation::Horizontal => {
			for offset in 0..*ship_size {
				board[pos_y][pos_x + offset] = cell;
			}
		}
		Rotation::Vertical => {
			for offset in 0..*ship_size {
				board[pos_y + offset][pos_x] = cell;
			}
		}
	}

	board
}

#[test]
fn place_entity_works() {
	let mut result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &1, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &2, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Horizontal, &3, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Placeholder,
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &1, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &2, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);

	result = place_entity([[Empty; 10]; 10], 0, 0, &Rotation::Vertical, &3, Placeholder);
	assert_eq!(
		result,
		[
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[
				Placeholder,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty,
				Empty
			],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
			[Empty; 10],
		]
	);
}

pub fn is_free_space(
	board: &[[Cell; 10]; 10],
	temp_pos_x: isize,
	temp_pos_y: isize,
	ship_size: &usize,
	rotation: &Rotation,
) -> bool {
	let mut result = true;

	if temp_pos_x < 0 || temp_pos_y < 0 {
		return false;
	}

	let pos_x: usize = temp_pos_x as usize;
	let pos_y: usize = temp_pos_y as usize;

	match rotation {
		Rotation::Horizontal => {
			for offset in 0..*ship_size {
				let new_pos_x = pos_x + offset;
				if new_pos_x > 9 || pos_y > 9 || board[pos_y][new_pos_x] != Empty && board[pos_y][new_pos_x] != Placeholder {
					result = false;
					break;
				}
			}
		}
		Rotation::Vertical => {
			for offset in 0..*ship_size {
				let new_pos_y = pos_y + offset;
				if pos_x > 9 || new_pos_y > 9 || board[new_pos_y][pos_x] != Empty && board[new_pos_y][pos_x] != Placeholder {
					result = false;
					break;
				}
			}
		}
	};

	result
}

#[test]
fn is_free_space_works() {
	let mut board = [[Empty; 10]; 10];
	board[1][0] = Cell::Ship;
	board[2][1] = Cell::Ship;
	assert_eq!(is_free_space(&board, 0, 0, &2, &Rotation::Vertical), false);
}

pub fn get_next_available_coordinates(
	board: &[[Cell; 10]; 10],
	ship_size: &usize,
	rotation: &Rotation,
) -> (usize, usize) {
	let mut o = 0;
	let mut pos_x = 0;
	let mut pos_y = 0;

	for i in 0..10 {
		if is_free_space(&board, o as isize, o as isize, ship_size, rotation) {
			pos_x = o as usize;
			pos_y = o as usize;
			break;
		}

		for n in 0..i {
			if is_free_space(&board, o as isize, n as isize, ship_size, rotation) {
				pos_x = o as usize;
				pos_y = n as usize;
				break;
			}

			if is_free_space(&board, n as isize, o as isize, ship_size, rotation) {
				pos_x = n as usize;
				pos_y = o as usize;
				break;
			}
		}
		o += 1;
	}

	(pos_x, pos_y)
}

#[test]
fn get_next_available_coordinates_works() {
	let mut board = [[Empty; 10]; 10];
	board[1][0] = Cell::Ship;
	board[2][1] = Cell::Ship;
	assert_eq!(get_next_available_coordinates(&board, &2, &Rotation::Vertical), (2, 2));
}
