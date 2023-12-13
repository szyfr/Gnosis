

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{ops::{Sub, Add, Mul}, fmt::Display};


//= Structures

/// Vector2 type
#[derive(Copy, Clone, PartialEq)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}
impl Sub for Vector2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}
impl Add for Vector2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}
impl Mul<f32> for Vector2 {
    type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}
impl Display for Vector2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "[{},{}]",self.x, self.y);
	}
}
impl From<raylib_ffi::Vector2> for Vector2 {
	fn from(value: raylib_ffi::Vector2) -> Self {
		Self {
			x: value.x,
			y: value.y,
		}
	}
}
impl From<[i32;2]> for Vector2 {
	fn from(value: [i32;2]) -> Self {
		Self {
			x: value[0] as f32,
			y: value[1] as f32,
		}
	}
}
impl From<[f32;2]> for Vector2 {
	fn from(value: [f32;2]) -> Self {
		Self {
			x: value[0],
			y: value[1],
		}
	}
}
impl Into<raylib_ffi::Vector2> for Vector2 {
	fn into(self) -> raylib_ffi::Vector2 {
		return raylib_ffi::Vector2 { x: self.x, y: self.y };
	}
}
impl Into<[i32;2]> for Vector2 {
	fn into(self) -> [i32;2] {
		return [self.x as i32, self.y as i32];
	}
}
impl Into<[f32;2]> for Vector2 {
	fn into(self) -> [f32;2] {
		return [self.x, self.y];
	}
}

/// Vector3 type
#[derive(Copy, Clone, PartialEq)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
impl Sub for Vector3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}
impl Add for Vector3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl Mul<f32> for Vector3 {
    type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl Display for Vector3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "[{},{},{}]",self.x, self.y, self.z);
	}
}
impl From<raylib_ffi::Vector3> for Vector3 {
	fn from(value: raylib_ffi::Vector3) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
		}
	}
}
impl From<[i32;3]> for Vector3 {
	fn from(value: [i32;3]) -> Self {
		Self {
			x: value[0] as f32,
			y: value[1] as f32,
			z: value[2] as f32,
		}
	}
}
impl From<[f32;3]> for Vector3 {
	fn from(value: [f32;3]) -> Self {
		Self {
			x: value[0],
			y: value[1],
			z: value[2],
		}
	}
}
impl Into<raylib_ffi::Vector3> for Vector3 {
	fn into(self) -> raylib_ffi::Vector3 {
		return raylib_ffi::Vector3 { x: self.x, y: self.y, z: self.z };
	}
}
impl Into<[i32;3]> for Vector3 {
	fn into(self) -> [i32;3] {
		return [self.x as i32, self.y as i32, self.z as i32];
	}
}
impl Into<[f32;3]> for Vector3 {
	fn into(self) -> [f32;3] {
		return [self.x, self.y, self.z];
	}
}

/// Vector4 type
#[derive(Copy, Clone)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

/// Quaternion type
#[derive(Copy, Clone)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}


//= Procedures

impl Vector2 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector2 {
		return Vector2 { x: 0.0, y: 0.0 }
	}

}

impl Vector3 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector3 {
		return Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	/// Returns true if the inpout Vector is with offset of the original
	pub fn close(&self, v2: Self, offset: f32) -> bool {
		let mut output = true;

		if self.x > v2.x + offset || self.x < v2.x - offset { output = false; }
		if self.y > v2.y + offset || self.y < v2.y - offset { output = false; }
		if self.z > v2.z + offset || self.z < v2.z - offset { output = false; }
		
		return output;
	}

	/// Rounds V3
	pub fn round(&self) -> Self {
		Self {
			x: self.x.round(),
			y: self.y.round(),
			z: self.z.round(),
		}
	}

	/// Returns position of camera rotated around input ``Vector3``.
	pub fn rotate(&self, dist: Self, rot: f32) -> Self {
		let mut position = Vector3{x:0.0,y:0.0,z:0.0};

		position.x = dist.x * (rot / 57.3).cos() - dist.z * (rot / 57.3).sin();
		position.z = dist.x * (rot / 57.3).sin() + dist.z * (rot / 57.3).cos();

		position.x += self.x;
		position.y  = self.y + dist.y;
		position.z += self.z;

		return position;
	}

	/// Creates a binary direction for the difference between two points.
	pub fn direction_to(&self, v2: Self) -> Self {
		let difference = v2 - *self;
		let mut output = Vector3{x:0.0,y:0.0,z:0.0};

		if difference.x  > 0.0 { output.x =  1.0 }
		if difference.x == 0.0 { output.x =  0.0 }
		if difference.x  < 0.0 { output.x = -1.0 }

		if difference.y  > 0.0 { output.y =  1.0 }
		if difference.y == 0.0 { output.y =  0.0 }
		if difference.y  < 0.0 { output.y = -1.0 }

		if difference.z  > 0.0 { output.z =  1.0 }
		if difference.z == 0.0 { output.z =  0.0 }
		if difference.z  < 0.0 { output.z = -1.0 }
		
		return output;
	}

}
