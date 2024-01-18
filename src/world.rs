

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;
use crate::{tiles::{Tile, Gasses, GasStruct}, graphics::Graphics, raylib::vectors::Vector3};


//= Structures

///
pub struct World {
	pub rotation: i32,
	//pub chunks: HashMap<[i32;3],Chunk>,
	pub chunks: HashMap<[i32;3],Chunk2>,
}

///
#[derive(Clone)]
pub struct Chunk([[[Tile;16];16];16]);
#[derive(Clone)]
pub struct Chunk2(Vec<Vec<Vec<Tile>>>);


//= Procedures

impl World {
	
	/// Create new map
	pub fn new() -> Self {
		Self {
			rotation: 0,
			chunks: HashMap::new(),
		}
	}

	/// Draw map
	// TODO also have player position to only draw nearby chunks
	pub fn draw(&self, graphics: &Graphics, playerPosition: Vector3) {
		let _positionTile: [i32;3] = playerPosition.into();
		for (pos, chunk) in self.chunks.iter() {
			for x in 0..16 {
				for z in 0..16 {
					for y in 0..16 {
						let position = [(pos[0] * 16) + (x as i32), (pos[1] * 16) + (y as i32), (pos[2] * 16) + (z as i32),];
						if !chunk.0[x][y][z].is_empty() && chunk.tile_should_draw([x as i32,y as i32,z as i32]) { graphics.draw_tile(chunk.0[x][y][z], position, self.rotation); }
					}
				}
			}
		}
		// TODO next
		//for chunkX in positionTile[0]-2..positionTile[0]+2 {
		//	for x in 0..16 {
		//		for z in 0..16 {
		//			for y in 0..16 {
		//				let position = [(pos[0] * 16) + (x as i32), (pos[1] * 16) + (y as i32), (pos[2] * 16) + (z as i32),];
		//				if !chunk.0[x][y][z].is_empty() && chunk.tile_should_draw([x as i32,y as i32,z as i32]) { graphics.draw_tile(chunk.0[x][y][z], position, self.rotation); }
		//			}
		//		}
		//	}
		//}
	}

	pub fn generate_test(&mut self) {
		print!("Fuck\n");
		//let mut chunk = [[[Tile::Empty{p:0};16];16];16];
		//let chunkbody = [[[Tile::Test{p:0};16];16];16];
		let layer1 = vec![
			Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},
			Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},
			Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},
			Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},Tile::Empty{p:0},
		];
		let layer2 = vec![
			layer1.clone(),layer1.clone(),layer1.clone(),layer1.clone(),
			layer1.clone(),layer1.clone(),layer1.clone(),layer1.clone(),
			layer1.clone(),layer1.clone(),layer1.clone(),layer1.clone(),
			layer1.clone(),layer1.clone(),layer1.clone(),layer1.clone(),
		];
		let mut chunk = vec![
			layer2.clone(),layer2.clone(),layer2.clone(),layer2.clone(),
			layer2.clone(),layer2.clone(),layer2.clone(),layer2.clone(),
			layer2.clone(),layer2.clone(),layer2.clone(),layer2.clone(),
			layer2.clone(),layer2.clone(),layer2.clone(),layer2.clone(),
		];

		for x in 0..16 {
			for y in 0..8 {
				for z in 0..16 {
					//let noGas = GasStruct{gas:Gasses::None, amount: 1.0};
					//if y == 8 { chunk[x][y][z] = Tile::Air { gasses: [GasStruct{gas:Gasses::Air, amount: 1.0},noGas,noGas,noGas] }; }
					//else { chunk[x][y][z] = Tile::Test; }
					chunk[x][y][z] = Tile::Test{p:0};
				}
			}
		}
		chunk[0][8][0] = Tile::Test{p:0};

		self.chunks.insert([-1, 0, 0], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 0, 0, 0], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 1, 0, 0], Chunk2(chunk.clone()).clone());
		self.chunks.insert([-1, 0,-1], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 0, 0,-1], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 1, 0,-1], Chunk2(chunk.clone()).clone());
		self.chunks.insert([-1, 0, 1], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 0, 0, 1], Chunk2(chunk.clone()).clone());
		self.chunks.insert([ 1, 0, 1], Chunk2(chunk.clone()).clone());
	}

}

impl Chunk {

	pub fn tile_should_draw(&self, position: [i32;3]) -> bool {
		if position[0] == 15 || position[1] == 15 || position[2] == 15 { return true }
		return
			self.0[(position[0]+1) as usize][position[1] as usize][position[2] as usize].is_empty() ||
			self.0[position[0] as usize][(position[1]+1) as usize][position[2] as usize].is_empty() ||
			self.0[position[0] as usize][position[1] as usize][(position[2]+1) as usize].is_empty();
	}

}

impl Chunk2 {

	pub fn tile_should_draw(&self, position: [i32;3]) -> bool {
		if position[0] == 15 || position[1] == 15 || position[2] == 15 { return true }
		return
			self.0[(position[0]+1) as usize][position[1] as usize][position[2] as usize].is_empty() ||
			self.0[position[0] as usize][(position[1]+1) as usize][position[2] as usize].is_empty() ||
			self.0[position[0] as usize][position[1] as usize][(position[2]+1) as usize].is_empty();
	}

}