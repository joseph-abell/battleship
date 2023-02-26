extern crate rand;
extern crate termion;

use std::{thread, time};

mod ai;
pub mod config;
pub mod game;
mod gui;
mod history;
pub mod movement;
pub mod ships;

use history::History;
use ships::Ship;
use ships::ShipTracker;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use Cell::{Damage, Empty, Placeholder, Shot};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
	Empty,
	Shot,
	Ship,
	ShipTwo([usize; 4]),
	ShipThree([usize; 6]),
	ShipFour([usize; 8]),
	Damage,
	Placeholder,
	Crosshair,
}

#[derive(Copy, Clone, Debug)]
pub enum Rotation {
	Horizontal,
	Vertical,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

fn main() {
	let min_width = config::SIZE_X as u16 * 3 * 2 + 11;
	let min_height = 7 + 2 + 1 + config::SIZE_Y as u16 + 1 + 7 + 4 + 3;
	let size = termion::terminal_size();

	if let Ok((width, height)) = size {
		if width < min_width || height < min_height {
			panic!("\r\n\r\n{}This terminal is not big enough width width:{} height:{}\r\nTo play Battlefield you need at least width:{} height:{}{}\r\n\r\n", termion::color::Fg(termion::color::Red), width, height, min_width, min_height, termion::color::Fg(termion::color::Reset));
		}
	} else {
		panic!("The size of the terminal can't be determined");
	}

	let mut stdout = stdout().into_raw_mode().unwrap();

	// our boards
	let mut board_me = [[Empty; config::SIZE_X]; config::SIZE_Y];
	let mut board_ai = [[Empty; config::SIZE_X]; config::SIZE_Y];
	let mut board_secret = [[Empty; config::SIZE_X]; config::SIZE_Y];

	let mut history = History::new();

	// let the AI generate their own ship placements
	board_secret = ai::set_ships(board_secret);
	let mut ai_attack = ai::Attack::new();

	// the ships to be placed
	let mut ships =
		ShipTracker::new(config::SHIP_TWO_BLOCK_AMOUNT, config::SHIP_THREE_BLOCK_AMOUNT, config::SHIP_FOUR_BLOCK_AMOUNT);
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	let mut ship_size = config::get_entitie_size(&this_ship);

	// rotation of our ship
	let mut rotation = Rotation::Horizontal;

	// our current position on the board
	let mut pos_x: usize = 0;
	let mut pos_y: usize = 0;

	// placing our first ship
	board_me = movement::place_entity(board_me, pos_x, pos_y, ship_size, &rotation, Placeholder);

	write!(stdout, "{}{}", termion::color::Bg(termion::color::Black), termion::clear::All).unwrap();
	stdout.flush().unwrap();

	gui::draw(
		&mut stdout,
		gui::get_score(board_me, board_ai, gui::Round::One),
		gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::One),
		history.get_history(),
		gui::get_round1_instructions(),
	);

	let mut is_round_one_done = false;

	// FIRST ROUND setting ships
	for key in stdin().keys() {
		match key.unwrap() {
			Key::Esc | Key::Char('q') => {
				write!(stdout, "{}{}", termion::cursor::Restore, termion::cursor::Show).unwrap();
				stdout.flush().unwrap();
				termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
				std::process::exit(0);
			}
			Key::Char('r') => {
				let new_rotation = match rotation {
					Rotation::Horizontal => Rotation::Vertical,
					Rotation::Vertical => Rotation::Horizontal,
				};

				if movement::is_free_space(&board_me, pos_x as isize, pos_y as isize, ship_size, &new_rotation) {
					// reset previous placement
					board_me = movement::place_entity(board_me, pos_x, pos_y, ship_size, &rotation, Empty);
					rotation = new_rotation;
					// now place new ship in new rotation
					board_me = movement::place_entity(board_me, pos_x, pos_y, ship_size, &rotation, Placeholder);
				}
			}
			// PLACE SHIP
			Key::Char('\n') => {
				board_me = movement::place_entity(board_me, pos_x, pos_y, ship_size, &rotation, Cell::Ship);

				ships.set_ship(&this_ship);
				match ships.get_next_unset_ship() {
					Some(kind) => {
						this_ship = kind;
						ship_size = config::get_entitie_size(&this_ship);
						// collision detection for new pos_x and pos_y
						let (x, y) = movement::get_next_available_coordinates(&board_me, ship_size, &rotation);
						pos_x = x;
						pos_y = y;
						board_me = movement::place_entity(board_me, pos_x, pos_y, ship_size, &rotation, Placeholder);
					}
					None => {
						is_round_one_done = true;
					}
				};
			}
			// MOVEMENT
			Key::Left => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, ship_size, &rotation, Direction::Left);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Right => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, ship_size, &rotation, Direction::Right);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Up => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, ship_size, &rotation, Direction::Up);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Down => {
				let (board_new, pos_x_new, pos_y_new) =
					movement::move_ship(board_me, pos_x, pos_y, ship_size, &rotation, Direction::Down);
				board_me = board_new;
				pos_x = pos_x_new;
				pos_y = pos_y_new;
			}
			Key::Delete | Key::Backspace => {
				ships = ShipTracker::new(
					config::SHIP_ONE_BLOCK_AMOUNT,
					config::SHIP_TWO_BLOCK_AMOUNT,
					config::SHIP_THREE_BLOCK_AMOUNT,
				);
				this_ship = ships.get_next_unset_ship().unwrap();
				ship_size = config::get_entitie_size(&this_ship);
				rotation = Rotation::Horizontal;
				pos_x = 0;
				pos_y = 0;
				board_me = movement::place_entity(
					[[Empty; config::SIZE_X]; config::SIZE_Y],
					pos_x,
					pos_y,
					ship_size,
					&rotation,
					Placeholder,
				);
			}
			_ => {}
		}

		gui::draw(
			&mut stdout,
			gui::get_score(board_me, board_ai, gui::Round::One),
			gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::One),
			history.get_history(),
			gui::get_round1_instructions(),
		);

		if is_round_one_done {
			break;
		}
	}

	pos_x = 0;
	pos_y = 0;
	history.set_history("Placed ships", history::Actor::Me);
	history.set_history("Placed ships", history::Actor::Ai);

	gui::draw(
		&mut stdout,
		gui::get_score(board_me, board_ai, gui::Round::Two),
		gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::Two),
		history.get_history(),
		gui::get_round2_instructions(),
	);

	let mut is_round_two_done = false;

	// SECOND ROUND shooting turns
	for key in stdin().keys() {
		match key.unwrap() {
			Key::Esc | Key::Char('q') => {
				write!(stdout, "{}{}", termion::cursor::Restore, termion::cursor::Show).unwrap();
				stdout.flush().unwrap();
				termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
				std::process::exit(0);
			}
			// SHOOT
			Key::Char('\n') => {
				if movement::is_free_space(&board_ai, pos_x as isize, pos_y as isize, 1, &Rotation::Horizontal) {
					let mut ai_move = false;
					let hit_type = game::get_hit_type(&board_ai, &board_secret, pos_x, pos_y);
					match hit_type {
						game::HitType::Hit => {
							history
								.set_history(&format!("Shoot at {} and hit a ship", gui::get_coord(pos_x, pos_y)), history::Actor::Me);
							board_ai = movement::place_entity(board_ai, pos_x, pos_y, 1, &Rotation::Horizontal, Damage);
						}
						game::HitType::HitNSunk => {
							history.set_history(
								&format!("Shoot at {} and hit and sunk a ship", gui::get_coord(pos_x, pos_y)),
								history::Actor::Me,
							);
							board_ai = movement::place_entity(board_ai, pos_x, pos_y, 1, &Rotation::Horizontal, Damage);
							ai_move = true;
						}
						game::HitType::Miss => {
							history.set_history(&format!("Shoot at {} and missed", gui::get_coord(pos_x, pos_y)), history::Actor::Me);
							board_ai = movement::place_entity(board_ai, pos_x, pos_y, 1, &Rotation::Horizontal, Shot);
							ai_move = true;
						}
					};

					// AI FIRST SHOT
					if ai_move {
						let mut another_turn = false;
						let (ai_pos_x, ai_pos_y) = ai_attack.shoot(&board_me);
						let hit_type = game::get_hit_type(&board_me, &board_me, ai_pos_x, ai_pos_y);

						match hit_type {
							game::HitType::Hit => {
								history.set_history(
									&format!("Shoot at {} and hit a ship", gui::get_coord(ai_pos_x, ai_pos_y)),
									history::Actor::Ai,
								);
								board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Damage);
								another_turn = true;
							}
							game::HitType::HitNSunk => {
								history.set_history(
									&format!("Shoot at {} and hit and sunk a ship", gui::get_coord(ai_pos_x, ai_pos_y)),
									history::Actor::Ai,
								);
								board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Damage);
							}
							game::HitType::Miss => {
								history.set_history(
									&format!("Shoot at {} and missed", gui::get_coord(ai_pos_x, ai_pos_y)),
									history::Actor::Ai,
								);
								board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Shot);
							}
						};

						gui::draw(
							&mut stdout,
							gui::get_score(board_me, board_ai, gui::Round::Two),
							gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::One),
							history.get_history(),
							gui::get_round2_instructions(),
						);

						// AI SHOT AFTER HIT
						while another_turn {
							thread::sleep(time::Duration::from_millis(2000));

							let (ai_pos_x, ai_pos_y) = ai_attack.shoot_after_hit(&board_me);
							let hit_type = game::get_hit_type(&board_me, &board_me, ai_pos_x, ai_pos_y);

							match hit_type {
								game::HitType::Hit => {
									history.set_history(
										&format!("Shoot at {} and hit a ship", gui::get_coord(ai_pos_x, ai_pos_y)),
										history::Actor::Ai,
									);
									board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Damage);
								}
								game::HitType::HitNSunk => {
									history.set_history(
										&format!("Shoot at {} and hit and sunk a ship", gui::get_coord(ai_pos_x, ai_pos_y)),
										history::Actor::Ai,
									);
									board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Damage);
									another_turn = false;
								}
								game::HitType::Miss => {
									history.set_history(
										&format!("Shoot at {} and missed", gui::get_coord(ai_pos_x, ai_pos_y)),
										history::Actor::Ai,
									);
									board_me = movement::place_entity(board_me, ai_pos_x, ai_pos_y, 1, &Rotation::Horizontal, Shot);
									another_turn = false;
								}
							};

							gui::draw(
								&mut stdout,
								gui::get_score(board_me, board_ai, gui::Round::Two),
								gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::One),
								history.get_history(),
								gui::get_round2_instructions(),
							);
						}
					}

					let (x, y) = movement::get_next_available_coordinates(&board_ai, 1, &Rotation::Horizontal);
					pos_x = x;
					pos_y = y;

					let score_me = game::get_score(&board_ai);
					let score_ai = game::get_score(&board_me);

					if score_me == *"10" || score_ai == *"10" {
						is_round_two_done = true;
					}
				}
			}
			// MOVEMENT
			Key::Left => {
				if pos_x > 0 {
					pos_x -= 1;
				}
			}
			Key::Right => {
				if pos_x < (config::SIZE_X - 1) {
					pos_x += 1;
				}
			}
			Key::Up => {
				if pos_y > 0 {
					pos_y -= 1;
				}
			}
			Key::Down => {
				if pos_y < (config::SIZE_Y - 1) {
					pos_y += 1;
				}
			}
			_ => {}
		}

		gui::draw(
			&mut stdout,
			gui::get_score(board_me, board_ai, gui::Round::Two),
			gui::get_board(&board_me, &board_ai, pos_x, pos_y, gui::Round::Two),
			history.get_history(),
			gui::get_round2_instructions(),
		);

		if is_round_two_done {
			write!(stdout, "{}\r\n", gui::get_good_bye_msg(game::get_score(&board_ai) == *"10")).unwrap();
			break;
		}
	}

	write!(stdout, "{}", termion::cursor::Show).unwrap();
	stdout.flush().unwrap();
}
