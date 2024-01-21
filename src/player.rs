

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{units::Unit, raylib::{vectors::Vector3, self}};


//= Structures

/// Player structure
pub struct Player {
	pub unit: Unit,
}


//= Procedures

impl Player {
	
	/// Create new player
	pub fn new() -> Self {
		Self {
			unit: Unit::new_vec(Vector3{x:17.0,y:8.0,z:17.0}),
		}
	}

	/// Update Player
	pub fn update(&mut self) {
		let up    = raylib::button_down(raylib_ffi::enums::KeyboardKey::W as i32);
		let down  = raylib::button_down(raylib_ffi::enums::KeyboardKey::S as i32);
		let left  = raylib::button_down(raylib_ffi::enums::KeyboardKey::A as i32);
		let right = raylib::button_down(raylib_ffi::enums::KeyboardKey::D as i32);
		if up {
			if right { self.unit.position.z += 0.025; }
			if left  { self.unit.position.x += 0.025; }
			self.unit.position.x -= 0.05;
			self.unit.position.z -= 0.05;
		}
		if down {
			if right { self.unit.position.x -= 0.025; }
			if left  { self.unit.position.z -= 0.025; }
			self.unit.position.x += 0.05;
			self.unit.position.z += 0.05;
		}
		if left {
			self.unit.position.x -= 0.05;
			self.unit.position.z += 0.05;
		}
		if right {
			self.unit.position.x += 0.05;
			self.unit.position.z -= 0.05;
		}
		if raylib::button_pressed(raylib_ffi::enums::KeyboardKey::O as i32) {
			print!("{}\n",self.unit.position);
		}
	}

	/// Get current position
	pub fn get_position(&self) -> Vector3 {
		self.unit.position
	}

}