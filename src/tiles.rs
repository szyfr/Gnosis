

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

use crate::raylib::{textures::Texture, vectors::Vector3};

/// Tile structure
#[derive(Clone,Copy)]
pub struct Tile {
	//pub texture: Texture,
	pub empty: bool,
}


//= Procedures

impl Tile {
	
	/// Create a new blank tile
	pub fn new() -> Self {
		Self {
			//texture: Texture::empty(),
			empty: true,
		}
	}

	///
	pub fn draw(&self, position: Vector3) -> &Self {
		self.texture.draw(
			(position.x as i32 * 16) - (position.z as i32 * 16),
			(position.x as i32 *  8) + (position.z as i32 *  8) - (position.y as i32 * 16),
		);

		return self;
	}

}