

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

/// Rectangle type
#[derive(Copy, Clone)]
pub struct Rectangle {
	pub x: f32,			//* Rectangle top-left corner position x
	pub y: f32,			//* Rectangle top-left corner position y
	pub width: f32,		//* Rectangle width
	pub height: f32,	//* Rectangle height
}
impl ToString for Rectangle {
    fn to_string(&self) -> String {
    	return "[".to_string() + &self.x.to_string() + "," + &self.y.to_string() + "," + &self.width.to_string() + "," + &self.height.to_string() + "]"
    }
}
impl From<raylib_ffi::Rectangle> for Rectangle {
	fn from(value: raylib_ffi::Rectangle) -> Self {
		Self {
			x: value.x,
			y: value.y,
			width: value.width,
			height: value.height,
		}
	}
}
impl Into<raylib_ffi::Rectangle> for Rectangle {
	fn into(self) -> raylib_ffi::Rectangle {
		return raylib_ffi::Rectangle {
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
		};
	}
}


//= Procedures

impl Rectangle {

	/// Zeroed out rect
	pub fn zero() -> Self {
		return Rectangle {
			x:		0.0,
			y:		0.0,
			width:	0.0,
			height:	0.0,
		}
	}

	/// Create a rectangle using an index in a spritesheet
	pub fn tex_rect(index: i32, size: [i32;2]) -> Self {
		return Rectangle {
			x:		(index * size[0]) as f32,
			y:		0.0,
			width: 	size[0] as f32,
			height:	size[1] as f32,
		}
	}

}