

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

use std::fmt::Display;

/// Tile structure
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
	Empty,
	Test,

	Air {
		gas: Gasses,
		amount: f32,
	},
}
impl ToString for Tile {
	fn to_string(&self) -> String {
		match self {
			Tile::Empty { .. } => return "tile_empty".to_string(),
			Tile::Test  { .. } => return "tile_test".to_string(),

			//Tile::Air { gasses } => return format!("tile_air_[{},{},{},{}]",gasses[0],gasses[1],gasses[2],gasses[3]),
			Tile::Air { .. } => return format!("tile_air"),
		}
	}
}

/// Gas definitions
#[derive(Clone, Copy, PartialEq)]
pub enum Gasses {
	None,

	Air,
}
impl Display for Gasses {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Gasses::None => write!(f, "none"),
			Gasses::Air => write!(f, "air"),
		}
	}
}

/// Structure for gasses
#[derive(Clone, Copy, PartialEq)]
pub struct GasStruct {
	pub gas: Gasses,
	pub amount: f32,
}
impl Display for GasStruct {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.gas, self.amount)
	}
}


//= Procedures

impl Tile {

	pub fn is_empty(self) -> bool {
		return self == Tile::Empty;
	}

	pub fn is_clear(self) -> bool {
		match self {
			Tile::Empty => true,
			Tile::Test  => false,
			// TODO Do more with the amount
			Tile::Air { gas, amount } => true,
		}
	}
	
	///// Create a new blank tile
	//pub fn new() -> Self {
	//	Self {
	//		
	//	}
	//}

	/////
	//pub fn draw(&self, position: Vector3) -> &Self {
	//	self.texture.draw(
	//		(position.x as i32 * 16) - (position.z as i32 * 16),
	//		(position.x as i32 *  8) + (position.z as i32 *  8) - (position.y as i32 * 16),
	//	);
//
	//	return self;
	//}

}