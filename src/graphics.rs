

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{raylib::textures::Texture, world::tiles::Tile, world::Coords};


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

	/// Draw Tile to screen
	pub fn draw_tile(&self, tile: Tile, position: Coords) {
		self.textures[&tile.block.to_string()].draw(
			(position.x * 16) - (position.z * 16),
			(position.x *  8) + (position.z *  8) - (position.y * 16),
		);
	}

}