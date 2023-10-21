

//= Allows


//= Imports
pub mod skills;
pub mod player;
pub mod camera;

use std::collections::HashMap;

use raylib_ffi::Vector3;


//= Enumerations

/// Cardinal directions.
pub enum Direction{
	North,
	South,
	East,
	West,
}


//= Structures

/// Struture for all units.
pub struct Unit{
	pub position: Vector3,
	pub direction: Direction,
	
	pub health: [i32;2],
	pub stamina: [i32;2],
	pub mana: [i32;2],

	//TODO Potentially make this into it's own structure to allow copy?
	pub skills: HashMap<skills::Skill, i32>
}


//= Procedures

impl Unit {
	/// Retrieves skill level and whether they have it at all.
	pub fn get_skill(&self, s: skills::Skill ) -> (i32, bool) {
		if self.skills.contains_key(&s) { return (self.skills[&s], true) }
		return (0, false);
	}
}

/// Generate a new blank Unit.
pub fn new() -> Unit {
	return Unit {
		position: Vector3 {x: 0.0, y: 0.0, z: 0.0},
		direction: Direction::South,
		health: [10,10],
		stamina: [10,10],
		mana: [10,10],
		skills: HashMap::new(),
	}
}