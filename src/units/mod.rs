

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::raylib::vectors::Vector3;


//= Structures

/// General unit structure
pub struct Unit {
	pub position: Vector3,
}


//= Procedures

impl Unit {
	
	//
	pub fn new_vec(position: Vector3) -> Self {
		Self {
			position: position,
		}
	}

}