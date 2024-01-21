

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{world::{Coords, tiles::{Tile, TileType}}, graphics::Graphics};


//= Structures / Enumeration

///
pub struct World {
	pub chunks: HashMap<Coords,Chunk>,
}

///
#[derive(Clone)]
pub struct Chunk ([[[Tile;16];8];16]);


//= Procedures

impl World {

	/// Create a new world
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
		}
	}

	/// Generate test world
	pub fn generate_test(&mut self) {
		let chunk = Chunk{0:[[[Tile{block: TileType::Test, draw: true};16];8];16]};

		self.chunks.insert(Coords{x:0,y:0,z:0}, chunk.clone());
		self.chunks.insert(Coords{x:1,y:0,z:0}, chunk.clone());
		self.chunks.insert(Coords{x:0,y:0,z:1}, chunk.clone());
	}

	pub fn update(&mut self) {
		for x in 0..16 {
			for z in 0..16 {
				for y in 0..8 {
					
				}
			}
		}
	}

	/// Draw each tile and entity
	pub fn draw(&self, graphics: &Graphics, _position: Coords) {
		for (pos, chunk) in self.chunks.iter() {
			for x in 0..16 {
				for z in 0..16 {
					for y in 0..8 {
						let position = ((*pos).clone() * 16) + [x,y,z];

						let tile = chunk.0[x][y][z].clone();
						if tile.draw { graphics.draw_tile(tile, position); }
					}
				}
			}
		}
	}

}