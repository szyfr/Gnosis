

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{raylib::textures::Texture, tiles::Tile, world::Coords};


//= Structures

/// 
#[derive(Clone)]
pub struct Graphics {
	textures: HashMap<String, Texture>
}


//= Procedures

impl Graphics {
	
	/// Create a new blank tile
	pub fn new() -> Self {
		let mut textures: HashMap<String, Texture> = HashMap::new();
		textures.insert("tile_test".to_string(), Texture::load("data/test.png"));

		Self {
			textures,
		}
	}

	//
	pub fn draw_tile(&self, tile: Tile, position: Coords, _rotation: i32) {
		self.textures[&tile.to_string()].draw(
			(position.x * 16) - (position.z * 16),
			(position.x *  8) + (position.z *  8) - (position.y * 16),
		);
		//*! All of this is testing rotating the camera
		//*! It doesn't work yet
		//match rotation {
		//	1 => {
		//		self.textures[&tile.to_string()].draw(
		//			(position[2] * 16) - (position[0] * 16),
		//			(position[2] *  8) + (position[0] *  8) - (position[1] * 16),
		//		);
		//	}
		//	2 => {}
		//	3 => {}
		//	_ => {
		//		self.textures[&tile.to_string()].draw(
		//			(position[0] * 16) - (position[2] * 16),
		//			(position[0] *  8) + (position[2] *  8) - (position[1] * 16),
		//		);
		//	}
		//}
	}

}