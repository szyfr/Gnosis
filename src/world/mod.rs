

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod tiles;
pub mod chunks;

use std::{ops::{Add, Div, Rem, Mul}, fmt::Display};

use crate::raylib::vectors::Vector3;


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
impl Div<[usize;3]> for Coords {
	type Output = Coords;

	fn div(self, rhs: [usize;3]) -> Self::Output {
		Coords{
			x: self.x / rhs[0] as i32,
			y: self.y / rhs[1] as i32,
			z: self.z / rhs[2] as i32,
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
impl Rem<[usize;3]> for Coords {
	type Output = Coords;

	fn rem(self, rhs: [usize;3]) -> Self::Output {
		Coords{
			x: self.x % rhs[0] as i32,
			y: self.y % rhs[1] as i32,
			z: self.z % rhs[2] as i32,
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
impl Mul<[usize;3]> for Coords {
	type Output = Coords;

	fn mul(self, rhs: [usize;3]) -> Self::Output {
		Coords{
			x: self.x * rhs[0] as i32,
			y: self.y * rhs[1] as i32,
			z: self.z * rhs[2] as i32,
		}
	}
}
impl Into<[usize;3]> for Coords {
	fn into(self) -> [usize;3] {
		[self.x as usize, self.y as usize, self.z as usize]
	}
}
impl Into<Vector3> for Coords {
	fn into(self) -> Vector3 {
		Vector3{x:self.x as f32, y:self.y as f32, z:self.z as f32}
	}
}
impl From<Vector3> for Coords {
	fn from(value: Vector3) -> Self {
		Self {
			x: value.x as i32,
			y: value.y as i32,
			z: value.z as i32,
		}
	}
}
impl Display for Coords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{},{},{}]", self.x,self.y,self.z)
	}
}


//= Procedures