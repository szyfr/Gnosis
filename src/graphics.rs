

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{raylib::{textures::Texture, vectors::{Vector3, Vector2}}, world::tiles::Tile, world::{Coords, tiles::TileType}};


//= Structures

/// 
#[derive(Clone)]
pub struct Graphics {
	textures: HashMap<String, Texture>,

	pub blockTest: Texture,
	pub blockAir:  Texture,
}


//= Procedures

impl Graphics {
	
	/// Create a new blank tile
	pub fn new() -> Self {
		let mut textures: HashMap<String, Texture> = HashMap::new();
		textures.insert("tile_test".to_string(), Texture::load("data/test.png"));

		Self {
			textures,
			blockTest: Texture::load("data/test.png"),
			blockAir: Texture::load("data/air.png"),
		}
	}

	/// Draw Tile to screen
	pub fn draw_tile_(&self, tile: Tile, position: Coords) {
		self.textures[&tile.block.to_string()].draw(
			(position.x * 16) - (position.z * 16),
			(position.x *  8) + (position.z *  8) - (position.y * 16),
			raylib_ffi::colors::WHITE,
		);
	}
	pub fn draw_tile(&self, tile: Tile, position: Coords) {
		let posX = (position.x * 16) - (position.z * 16);
		let posY = (position.x *  8) + (position.z *  8) - (position.y * 16);
		
		match tile.block {
			TileType::Test => {
				self.blockTest.draw(posX, posY, raylib_ffi::colors::WHITE);
			}
			TileType::Air { .. } => {
				//self.blockAir.draw(posX, posY, raylib_ffi::Color{r:255,g:0,b:255,a:100});
			}
			_ => {}
		}
	}

	//TEMP
	pub fn draw_tile_V(&self, tile: Tile, position: Vector3) {
		self.textures[&tile.block.to_string()].draw_v(
			Vector2{
				x: (position.x * 16.0) - (position.z * 16.0),
				y: (position.x *  8.0) + (position.z *  8.0) - (position.y * 16.0),
			}
		);
	}

}