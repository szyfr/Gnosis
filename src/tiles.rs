

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

use crate::raylib::structures::{Texture, Vector2};

/// Tile structure
#[derive(Clone)]
pub struct Tile {
	pub texture: Texture,
}


//= Procedures

impl Tile {
	
	/// Create a new blank tile
	pub fn new() -> Self {
		Self {
			texture: Texture::empty()
		}
	}

	///
	pub fn draw(&self, position: Vector2) -> &Self {
		self.texture.draw(
			(position.x as i32 * 32) + (position.y as i32 * 16),
			position.y as i32 * 8,
		);

		return self;
	}

}