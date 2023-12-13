

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

/// Matrix, 4x4 components, column major, OpenGL style, right handed
#[derive(Clone)]
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32, //* Matrix first row  (4 components)
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32, //* Matrix second row (4 components)
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32, //* Matrix third row  (4 components)
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32, //* Matrix fourth row (4 components)
}
impl From<raylib_ffi::Matrix> for Matrix {
	fn from(value: raylib_ffi::Matrix) -> Self {
		Self {
			m0:  value.m0,
			m1:  value.m1,
			m2:  value.m2,
			m3:  value.m3,
			m4:  value.m4,
			m5:  value.m5,
			m6:  value.m6,
			m7:  value.m7,
			m8:  value.m8,
			m9:  value.m9,
			m10: value.m10,
			m11: value.m11,
			m12: value.m12,
			m13: value.m13,
			m14: value.m14,
			m15: value.m15,
		}
	}
}
impl Into<raylib_ffi::Matrix> for Matrix {
	fn into(self) -> raylib_ffi::Matrix {
		return raylib_ffi::Matrix {
			m0:  self.m0,
			m1:  self.m1,
			m2:  self.m2,
			m3:  self.m3,
			m4:  self.m4,
			m5:  self.m5,
			m6:  self.m6,
			m7:  self.m7,
			m8:  self.m8,
			m9:  self.m9,
			m10: self.m10,
			m11: self.m11,
			m12: self.m12,
			m13: self.m13,
			m14: self.m14,
			m15: self.m15,
		};
	}
}


//= Procedures

impl Matrix {
	
}