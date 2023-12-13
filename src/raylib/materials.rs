

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::{textures::Texture, shaders::Shader};


//= Structures

/// Material texture map
pub struct MaterialMap {
	texture:	Texture,
	color:		raylib_ffi::Color,
	value:		f32,
}

/// Material type (generic)
pub struct Material {
	shader:	Shader,
	maps:	*mut raylib_ffi::MaterialMap,
	params:	[f32;4],
}


//= Procedures