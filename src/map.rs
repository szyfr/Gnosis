

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::HashMap;

//= Imports
use crate::tiles::Tile;


//= Structures

///
pub struct Map {
	pub chunks: HashMap<[i32;3],Chunk>,
}

///
pub struct Chunk([[[Tile;16];16];16]);


//= Procedures

impl Map {
	
	/// Create new map
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
		}
	}
}

impl Chunk {

	//
	pub fn get_tile(&self, position: [i32;3]) -> Tile {
		return self.0[position[0] as usize][position[1] as usize][position[2] as usize].clone();
	}

}