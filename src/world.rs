

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, ops::{Add, Div, Rem, Mul}};
use crate::{tiles::{Tile, Gasses, GasStruct}, graphics::Graphics, raylib::vectors::Vector3};


//= Structures

///
pub struct World {
	pub rotation: i32,
	//pub chunks: HashMap<[i32;3],Chunk>,
	pub chunks: HashMap<Coords,Chunk2>,
}

///
#[derive(Clone)]
pub struct Chunk([[[Tile;16];16];16]);
#[derive(Clone)]
pub struct Chunk2(Vec<Vec<Vec<Tile>>>);


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}
impl Add<Coords> for Coords {
	type Output = Coords;

	fn add(self, rhs: Self) -> Self::Output {
		Coords{
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl Add<i32> for Coords {
	type Output = Coords;

	fn add(self, rhs: i32) -> Self::Output {
		Coords{
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
		}
	}
}
impl Add<[usize;3]> for Coords {
	type Output = Coords;

	fn add(self, rhs: [usize;3]) -> Self::Output {
		Coords{
			x: self.x + rhs[0] as i32,
			y: self.y + rhs[1] as i32,
			z: self.z + rhs[2] as i32,
		}
	}
}
impl Div<i32> for Coords {
	type Output = Coords;

	fn div(self, rhs: i32) -> Self::Output {
		Coords{
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl Rem<i32> for Coords {
	type Output = Coords;

	fn rem(self, rhs: i32) -> Self::Output {
		Coords{
			x: self.x % rhs,
			y: self.y % rhs,
			z: self.z % rhs,
		}
	}
}
impl Mul<i32> for Coords {
	type Output = Coords;

	fn mul(self, rhs: i32) -> Self::Output {
		Coords{
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}


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
	pub fn draw(&self, graphics: &Graphics, playerPosition: Coords) {
		for (pos, chunk) in self.chunks.iter() {
			for x in 0..16 {
				for z in 0..16 {
					for y in 0..16 {
						//let position = [(pos.x * 16) + (x as i32), (pos.y * 16) + (y as i32), (pos.z * 16) + (z as i32),];
						let position = ((*pos).clone() * 16) + [x,y,z];
						//if !chunk.0[x][y][z].is_empty() && chunk.tile_should_draw([x as i32,y as i32,z as i32]) { graphics.draw_tile(chunk.0[x][y][z], position, self.rotation); }
						//if !chunk.0[x][y][z].is_empty() && self.tile_should_draw(Coords{x: x as i32,y: y as i32,z: z as i32}) { graphics.draw_tile(chunk.0[x][y][z], position, self.rotation); }
						let (tile, res) = self.get_tile(*pos);
						if !res { continue }
						if !chunk.0[x][y][z].is_empty() && self.tile_should_draw(Coords{x: x as i32,y: y as i32,z: z as i32}) { graphics.draw_tile(tile, position, self.rotation); }
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
			Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
			Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
			Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
			Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
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
					chunk[x][y][z] = Tile::Test;
				}
			}
		}
		chunk[0][8][0] = Tile::Test;

		//self.chunks.insert(Coords{x:-1, y:0, z: 0}, Chunk2(chunk.clone()).clone());
		self.chunks.insert(Coords{x: 0, y:0, z: 0}, Chunk2(chunk.clone()).clone());
		self.chunks.insert(Coords{x: 1, y:0, z: 0}, Chunk2(chunk.clone()).clone());
		//self.chunks.insert(Coords{x:-1, y:0, z:-1}, Chunk2(chunk.clone()).clone());
		//self.chunks.insert(Coords{x: 0, y:0, z:-1}, Chunk2(chunk.clone()).clone());
		//self.chunks.insert(Coords{x: 1, y:0, z:-1}, Chunk2(chunk.clone()).clone());
		//self.chunks.insert(Coords{x:-1, y:0, z: 1}, Chunk2(chunk.clone()).clone());
		self.chunks.insert(Coords{x: 0, y:0, z: 1}, Chunk2(chunk.clone()).clone());
		self.chunks.insert(Coords{x: 1, y:0, z: 1}, Chunk2(chunk.clone()).clone());
	}

	pub fn tile_should_draw(&self, position: Coords) -> bool {
		//* Load the tile or return false if it doesn't exist */
		let (tile, result) = self.get_tile(position);
		if !result { return false }
		match tile {
			Tile::Empty => return false,
			_ => {}
		}

		let (north, ..) = self.get_tile(position.clone() + Coords{x:0,y:1,z:0});
		let (south, ..) = self.get_tile(position.clone() + Coords{x:1,y:0,z:0});
		let (east , ..) = self.get_tile(position.clone() + Coords{x:0,y:0,z:1});

		north.is_clear() || south.is_clear() || east.is_clear()
	}

	pub fn get_tile(&self, position: Coords) -> (Tile, bool) {
		let chunkPosi = position / 16;
		let posiInChunk = position % 16;

		if self.chunks.contains_key(&chunkPosi) {
			(self.chunks[&chunkPosi].0[posiInChunk.x as usize][posiInChunk.y as usize][posiInChunk.z as usize], true)
		} else {
			(Tile::Empty, true)
		}
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

impl Coords {

}