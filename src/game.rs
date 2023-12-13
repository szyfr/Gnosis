

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::camera::Camera;


//= Structures

///
pub struct Game {
	camera: Camera,
	

}


//= Procedures

impl Game {
	
	/// Create new game
	pub fn new() -> Self {
		Self {
			camera: Camera::new(),
		}
	}
}