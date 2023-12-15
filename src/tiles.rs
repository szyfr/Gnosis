

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

/// Tile structure
#[derive(Clone,Copy, PartialEq)]
pub enum Tile {
	Empty,
	Test,
}
impl ToString for Tile {
	fn to_string(&self) -> String {
		match self {
			Tile::Empty => return "tile_empty".to_string(),
			Tile::Test  => return "tile_test".to_string(),
		}
	}
}


//= Procedures

impl Tile {

	pub fn is_empty(self) -> bool {
		return self == Tile::Empty;
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