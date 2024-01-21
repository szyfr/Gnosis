

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures / Enumeration

//
#[derive(Clone, Copy)]
pub enum TileType {
	Empty,
	Test,

	Air {

	},
}
impl ToString for TileType {
	fn to_string(&self) -> String {
		match self {
			TileType::Empty { .. } => return "tile_empty".to_string(),
			TileType::Test  { .. } => return "tile_test".to_string(),

			//TileType::Air { gasses } => return format!("tile_air_[{},{},{},{}]",gasses[0],gasses[1],gasses[2],gasses[3]),
			TileType::Air { .. } => return format!("tile_air"),
		}
	}
}

//
#[derive(Clone, Copy)]
pub struct Tile {
	pub block: TileType,
	pub draw: bool,
}
impl ToString for Tile {
	fn to_string(&self) -> String {
		todo!()
	}
}


//= Procedures