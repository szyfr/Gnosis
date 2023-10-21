

//= Allows


//= Imports


//= Enumerations

/// Enumeration describing each type of tile.
pub enum Tile{
	Null,
	Dust{
		value: i32,
	},
}


//= Structures


//= Procedures

impl Tile {
	///
	pub fn update(&mut self) {
		match self {
			Tile::Null => {},
			Tile::Dust{..} => {},
		}
	}
}