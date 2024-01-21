

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{world::{Coords, tiles::{Tile, TileType}}, graphics::{Graphics, self}};


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
		let mut chunk = Chunk{0:[[[Tile{block: TileType::Test, draw: true};16];8];16]};

		for y in 0..8 {
			for x in 0..16 {
				for z in 0..16 {
					if y < 6 { chunk.0[x][y][z].draw = false; }
					if y == 7 {chunk.0[x][y][z].block = TileType::Air { test: 1.0 };}
				}
			}
		}

		for x in -10..11 {
			for z in -10..11 {
				for y in 0..1 {
					self.chunks.insert(Coords{x,y,z}, chunk.clone());
				}
			}
		}
	}

	pub fn update_all(&mut self) {
		//for (pos, tile) in self.chunks.iter_mut() {
		//	for x in 0..16 {
		//		for z in 0..16 {
		//			for y in 0..8 {
		//				let trueCoords = Coords{
		//					x: x + (pos.x * 16),
		//					y: y + (pos.y * 8),
		//					z: z + (pos.z * 16),
		//				};
		//				let tile = self.get_tile(trueCoords);
		//			}
		//		}
		//	}
		//}
	}

	/// Returns a clone of the Tile in the input position
	pub fn get_tile(&self, position: Coords) -> Option<Tile> {
		let chunkPosi = position / [16,8,16];
		let posiInChunk: [usize;3] = (position % [16,8,16]).into();

		if self.chunks.contains_key(&chunkPosi) {
			Some(self.chunks[&chunkPosi].0[posiInChunk[0]][posiInChunk[1]][posiInChunk[2]].clone())
		} else { None }
	}

	/// Sets the tile in the input position to input tile
	pub fn set_tile(&mut self, position: Coords, tile: Tile) -> bool {
		let chunkPosi = position / [16,8,16];
		let posiInChunk: [usize;3] = (position % [16,8,16]).into();

		if self.chunks.contains_key(&chunkPosi) {
			self.chunks.get_mut(&chunkPosi).unwrap().0[posiInChunk[0]][posiInChunk[1]][posiInChunk[2]] = tile;
			true
		} else { false }
	}

	/// Draw each tile and entity
	pub fn draw(&self, graphics: &Graphics, position: Coords, zoom: f32) {
		let currentChunk = position / [16,8,16];

		//* See if chunk exists and if so, draw it */
		for chunkx in (currentChunk.x - 3)..(currentChunk.x + 3) {
			for chunkz in (currentChunk.z - 3)..(currentChunk.z + 3) {
				for chunky in (currentChunk.y - 4)..=(currentChunk.y) {
					let chunkPos = Coords{x:chunkx,y:chunky,z:chunkz};
					if self.chunks.contains_key(&chunkPos) {
						//* Draw chunk */
						self.chunks[&chunkPos].draw(graphics, chunkPos);
					}
				}
			}
		}
	}

}

impl Chunk {
	
	/// Draw a single chunk
	pub fn draw(&self, graphics: &Graphics, chunkPos: Coords) {
		for x in 0..16 {
			for z in 0..16 {
				for y in 0..8 {
					let truePosition = (chunkPos * [16,8,16]) + [x,y,z];
					let tile = self.0[x][y][z];
					if tile.draw { graphics.draw_tile(tile, truePosition); }
				}
			}
		}
	}

}