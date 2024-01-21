

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod tiles;
pub mod chunks;

use std::ops::{Add, Div, Rem, Mul};


//= Structures

/// Coordinates for the tiles
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