

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::raylib::vectors::{Vector2, Vector3};


//= Structures
#[derive(Clone, Copy)]
pub struct Camera {
	pub position: Vector2,
	pub zoom: f32,

	pub screenSize: Vector2,
}
impl Into<raylib_ffi::Camera2D> for Camera {
	fn into(self) -> raylib_ffi::Camera2D {
		return raylib_ffi::Camera2D {
			offset: Vector2::zero().into(),
			target: self.position.into(),
			rotation: 0.0,
			zoom: self.zoom,
		}
	}
}


//= Procedures

impl Camera {

	/// New Camera
	pub fn new() -> Self {
		unsafe { Self {
			position: Vector2::zero(),
			zoom: 2.0,
			//zoom: 0.5,

			screenSize: Vector2{
				x: raylib_ffi::GetScreenWidth() as f32,
				y: raylib_ffi::GetScreenHeight() as f32,
			}
		}}
	}

	pub fn update(&mut self, position: Vector3) {
		let cameraPosi = Vector2{
			x: (position.x * 16.0) - (position.z * 16.0) + 16.0 - (self.screenSize.x / (self.zoom * 2.0)),
			y: (position.x *  8.0) + (position.z *  8.0) - (position.y * 16.0) + 16.0 - (self.screenSize.y / (self.zoom * 2.0)),
		};

		self.position = cameraPosi;
	}

	/// Begin drawing using raylib_ffi::BeginMode3D
	pub fn begin_drawing(&self) {
		unsafe {
			raylib_ffi::BeginDrawing();
			raylib_ffi::BeginMode2D((*self).into());
		}
	}

	/// End drawing using raylib_ffi::EndMode3D
	pub fn end_mode_2D(&self) {
		unsafe {
			raylib_ffi::EndMode2D();
		}
	}

	//
	pub fn end_drawing(&self) {
		unsafe {
			raylib_ffi::EndDrawing();
		}
	}

}