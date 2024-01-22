

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib::{textures::Texture, vectors::{Vector3, Vector2}}, world::tiles::Tile, world::{Coords, tiles::TileType}};


//= Structures

/// 
#[derive(Clone, Copy)]
pub struct Graphics {
	pub blockTest: Texture,
	pub blockAir:  Texture,
}


//= Procedures

impl Graphics {
	
	/// Create a new blank tile
	pub fn new() -> Self {
		Self {
			blockTest: Texture::load("data/test.png"),
			blockAir:  Texture::load("data/air.png"),
		}
	}

	/// Draw Tile to screen
	pub fn draw_tile(&self, tile: Tile, position: Coords) {
		let posX = (position.x * 16) - (position.z * 16);
		let posY = (position.x *  8) + (position.z *  8) - (position.y * 16);
		
		match tile.block {
			TileType::Test => {
				self.blockTest.draw(posX, posY, raylib_ffi::colors::WHITE);
			}
			TileType::Air { .. } => {
				//self.blockAir.draw(posX, posY, raylib_ffi::Color{r:255,g:0,b:255,a:255});
			}
			_ => {}
		}
	}

	//TEMP
	pub fn draw_tile_V(&self, tile: Tile, position: Vector3) {
		let posX = (position.x * 16.0) - (position.z * 16.0);
		let posY = (position.x *  8.0) + (position.z *  8.0) - (position.y * 16.0);
		
		match tile.block {
			TileType::Test => {
				self.blockTest.draw_v(Vector2 { x: posX, y: posY });
			}
			TileType::Air { .. } => {
				//self.blockAir.draw(posX, posY, raylib_ffi::Color{r:255,g:0,b:255,a:255});
			}
			_ => {}
		}
		//self.textures[&tile.block.to_string()].draw_v(
		//	Vector2{
		//		x: (position.x * 16.0) - (position.z * 16.0),
		//		y: (position.x *  8.0) + (position.z *  8.0) - (position.y * 16.0),
		//	}
		//);
	}

}