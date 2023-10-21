

//= Allows


//= Imports
use super::{Unit, new, camera::{Camera, create_camera}};


//= Enumerations


//= Structures

/// Player character structure
pub struct Player{
	pub camera: Camera,
	pub unit: Unit,
}


//= Procedures

impl Player {
	pub fn update(&self) {

	}
}

/// Create blank player character.
pub fn create_player() -> Player {
	return Player {
		camera: create_camera(),
		unit: new(),
	};
}