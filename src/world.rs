

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::HashMap;

//= Imports
use crate::{tiles::Tile, raylib::{textures::Texture, vectors::Vector3}};


//= Structures

///
pub struct World {
	pub chunks: HashMap<[i32;3],Chunk>,
}

///
pub struct Chunk([[[Tile;16];16];16]);


//= Procedures

impl World {
	
	/// Create new map
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
		}
	}

	//
	pub fn draw(&self) {
		//for x in -5..5 {
		//	for y in -5..5 {
		//		for z in -5..5 {
		//			if self.contains_key(&[x,y,z]) {
		//				if !(tiles.contains_key(&[x+1,y,z]) && tiles.contains_key(&[x,y+1,z]) && tiles.contains_key(&[x,y,z+1])) {
		//					tile.draw(Vector3::from([x,y,z]));
		//				}
		//			}
		//		}
		//	}
		//}
		for (position, chunk) in self.chunks.iter() {
			for x in 0..16 {
				for y in 0..16 {
					for z in 0..16 {
						chunk.get_tile([x,y,z]).draw(Vector3::from([x + position[0], y + position[1], z + position[2]]));
					}
				}
			}
		}
	}

	//
	pub fn generate_test(&mut self) {
		self.chunks.insert([0,0,0], Chunk::generate_test());
	}

}

impl Chunk {

	pub fn generate_test() -> Self {
		let mut result = Self::new();
		//for x in 0..16 {
		//	for y in 0..8 {
		//		for z in 0..16 {
		//			result.0[x as usize][y as usize][z as usize].empty = false;
		//		}
		//	}
		//}
		return result;
	}

	pub fn new() -> Self {
		let textureTest = Texture::load("data/test.png");

		let mut tile = Tile::new();
		tile.texture = textureTest;

		return Self([[[tile.clone();16];16];16]);
	}

	//
	pub fn get_tile(&self, position: [i32;3]) -> Tile {
		return self.0[position[0] as usize][position[1] as usize][position[2] as usize].clone();
	}

}