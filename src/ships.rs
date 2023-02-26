#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ship {
	TwoBlock,
	ThreeBlock,
	FourBlock
}

#[derive(Debug)]
pub struct ShipTracker {
	two_block: usize,
	three_block: usize,
	four_block: usize,
}

impl ShipTracker {
	pub fn new(two_block: usize, three_block: usize, four_block: usize) -> Self {
		Self {
			two_block,
			three_block,
			four_block,
		}
	}

	pub fn get_next_unset_ship(&self) -> Option<Ship> {
		if self.two_block > 0 {
			Some(Ship::TwoBlock)
		} else if self.three_block > 0 {
			Some(Ship::ThreeBlock)
		} else if self.four_block > 0 {
			Some(Ship::FourBlock)
		} else {
			None
		}
	}

	pub fn set_ship(&mut self, kind: &Ship) {
		match kind {
			Ship::TwoBlock => {
				self.two_block -= 1;
			}
			Ship::ThreeBlock => {
				self.three_block -= 1;
			}
			Ship::FourBlock => {
				self.four_block -= 1;
			}
		}
	}

	pub fn get_ships(&self) -> (usize, usize, usize) {
		(self.two_block, self.three_block, self.four_block)
	}
}

#[test]
fn keep_track_of_ships() {
	let mut ships = ShipTracker::new(3, 2, 1);

	assert_eq!(ships.get_ships(), (3, 2, 1));
	let mut this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::TwoBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (2, 2, 1));
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::TwoBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (1, 2, 1));
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::TwoBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (0, 2, 1));
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::ThreeBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (0, 1, 1));
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::ThreeBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (0, 0, 1));
	this_ship = ships.get_next_unset_ship().unwrap();
	assert_eq!(this_ship, Ship::FourBlock);
	ships.set_ship(&this_ship);

	assert_eq!(ships.get_ships(), (0, 0, 0));
	assert!(ships.get_next_unset_ship().is_none())
}
