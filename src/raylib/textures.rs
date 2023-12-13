

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::{enums, vectors::Vector2, rectangles::Rectangle, images::Image};


//= Structures


/// Texture type
#[derive(Copy, Clone)]
pub struct Texture {
	pub id:			u32,				// OpenGL texture id
	pub width:		i32,				// Texture base width
	pub height:		i32,				// Texture base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:		enums::PixelFormat,	// Data format (PixelFormat type)
	pub origin:		Vector2,			// Origin point
	pub tint:		raylib_ffi::Color,	// Color
}
impl From<raylib_ffi::Texture> for Texture {
	fn from(value: raylib_ffi::Texture) -> Self {
		Self {
			id: value.id,
			width: value.width,
			height: value.height,
			mipmaps: value.mipmaps,
			format: enums::PixelFormat::from_i32(value.format),
			origin: Vector2 { x: 0.0, y: 0.0 },
			tint: raylib_ffi::colors::WHITE,
		}
	}
}
impl Into<raylib_ffi::Texture> for Texture {
	fn into(self) -> raylib_ffi::Texture {
		return raylib_ffi::Texture {
			id: self.id,
			width: self.width,
			height: self.height,
			mipmaps: self.mipmaps,
			format: self.format as i32,
		};
	}
}

/// Texture2D type, same as Texture
pub struct Texture2D (Texture);

/// TextureCubemap type, same as Texture
pub struct TextureCubeMap (Texture);

/// RenderTexture type
#[derive(Clone)]
pub struct RenderTexture {
	pub id: u32,
	pub texture: Texture,
	pub depth: Texture,
}
impl From<raylib_ffi::RenderTexture> for RenderTexture {
	fn from(value: raylib_ffi::RenderTexture) -> Self {
		Self {
			id: value.id,
			texture: Texture::from(value.texture),
			depth: Texture::from(value.depth),
		}
	}
}
impl Into<raylib_ffi::RenderTexture> for RenderTexture {
	fn into(self) -> raylib_ffi::RenderTexture {
		return raylib_ffi::RenderTexture {
			id: self.id,
			texture: self.texture.clone().into(),
			depth: self.depth.clone().into(),
		};
	}
}



//= Procedures

impl Texture {

	//= Blank
	/// Create unitialized texture
	pub fn empty() -> Self {
		Self {
			id:			0,
			width:		0,
			height:		0,
			mipmaps:	0,
			format:		enums::PixelFormat::Unknown,
			origin:		Vector2::zero(),
			tint:		raylib_ffi::colors::WHITE,
		}
	}

	//= Loading
	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Texture::from(raylib_ffi::LoadTexture(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Texture
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadTexture(self.clone().into());
		}
	}

	//= Manipulation
	/// Update texture
	pub fn update(&self, image: Image) {
		unsafe { raylib_ffi::UpdateTexture(self.clone().into(), image.data) }
	}

	//= Drawing
	/// Draw texture using raylib_ffi::DrawTexture
	pub fn draw(&self, posX: i32, posY: i32) -> Self {
		unsafe {
			raylib_ffi::DrawTexture(self.clone().into(), posX, posY, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureV
	pub fn draw_v(&self, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureV(self.clone().into(), position.into(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureEX
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTextureEx(self.clone().into(), position.into(), rotation, scale, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureRec
	pub fn draw_rec(&self, source: Rectangle, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureRec(self.clone().into(), source.into(), position.into(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTexturePro
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, rotation: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTexturePro(self.clone().into(), source.into(), dest.into(), self.origin.into(), rotation, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureNPatch
	pub fn draw_npatch(&self, dest: Rectangle, rotation: f32) -> Self {
		unsafe {
			let nPatchInfo = raylib_ffi::NPatchInfo {
				source: raylib_ffi::Rectangle {
					x:		0.0,
					y:		0.0,
					width:	self.width as f32,
					height:	self.height as f32,
				},
				left:	self.width / 3,
				top:	self.height / 3,
				right:	self.width / 3,
				bottom:	self.height / 3,
				layout:	0,
			};
			raylib_ffi::DrawTextureNPatch(self.clone().into(), nPatchInfo, dest.into(), self.origin.into(), rotation, self.tint);

			return self.clone();
		}
	}

}

impl RenderTexture {

	/// Creates a new, blank render texture at input dimensions
	pub fn new(width: i32, height: i32) -> Self {
		unsafe { return RenderTexture::from(raylib_ffi::LoadRenderTexture(width, height)); }
	}
	pub fn unload(&self) {
		unsafe { raylib_ffi::UnloadRenderTexture(self.clone().into()); }
	}

	/// Starts drawing onto texture
	pub fn begin_texture_mode(&self) {
		unsafe { raylib_ffi::BeginTextureMode(self.clone().into()); }
	}
	/// Stops drawing onto texture
	pub fn end_texture_mode(&self) {
		unsafe { raylib_ffi::EndTextureMode(); }
	}

}