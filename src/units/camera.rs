

//= Allows


//= Imports
use raylib_ffi::Vector3;


//= Enumerations


//= Structures


/// Camera structure.
pub struct Camera{
	pub position: Vector3,
	pub rotation: f32,
	pub fovy: f32,
}


//= Procedures

impl Camera {
	pub fn update(&self) {
		
	}
}

/// Create blank player character.
pub fn create_camera() -> Camera {
	return Camera {
		position: Vector3 {x: 0.0, y: 0.0, z: 0.0},
		rotation: 0.0,
		fovy: 0.0,
	}
}