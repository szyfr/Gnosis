

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::raylib::structures::Vector3;


//= Structures
pub struct Camera {
	pub position: Vector3,
	pub rotation: f32,
}


//= Procedures

impl Camera {

	/// Begin drawing using raylib_ffi::BeginMode3D
	pub fn begin_drawing(&self) {
		unsafe { raylib_ffi::BeginMode3D(self.to_ffi()) }
	}
	/// End drawing using raylib_ffi::EndMode3D
	pub fn end_drawing(&self) {
		unsafe { raylib_ffi::EndMode3D() }
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Camera3D {
		return raylib_ffi::Camera3D {
			position:	(self.position + Vector3{x: 2.5, y: 5.0, z: 2.5}).to_ffi(),
			target:		self.position.to_ffi(),
			up:			raylib_ffi::Vector3 { x: 0.0, y: 1.0, z: 0.0 },
			fovy:		70.0,
			projection: raylib_ffi::enums::CameraProjection::Orthographic as i32,
		};
	}
}