

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::{textures::Texture, rectangles::Rectangle, images::Image, vectors::Vector2};


//= Structures


/// Font character info
pub struct GlyphInfo {
	pub value: i32,
	pub offsetX: i32,
	pub offsetY: i32,
	pub advanceX: i32,
	pub image: Image,
}
impl From<raylib_ffi::GlyphInfo> for GlyphInfo {
	fn from(value: raylib_ffi::GlyphInfo) -> Self {
		Self {
			value: value.value,
			offsetX: value.offsetX,
			offsetY: value.offsetY,
			advanceX: value.advanceX,
			image: Image::from(value.image),
		}
	}
}
impl Into<raylib_ffi::GlyphInfo> for GlyphInfo {
	fn into(self) -> raylib_ffi::GlyphInfo {
		return raylib_ffi::GlyphInfo {
			value: self.value,
			offsetX: self.offsetX,
			offsetY: self.offsetY,
			advanceX: self.advanceX,
			image: self.image.into(),
		};
	}
}

/// Font type, includes texture and charSet array data
#[derive(Clone)]
pub struct Font {
	pub baseSize: i32,
	pub charsCount: i32,
	pub charsPadding: i32,
	pub texture: Texture,
	pub recs: *mut raylib_ffi::Rectangle,
	pub chars: *mut raylib_ffi::GlyphInfo,
}
impl ToString for Font {
    fn to_string(&self) -> String {
		unsafe {
			let rect = Rectangle::from(*self.recs);
			let str = "[".to_string() + &rect.to_string() + " : ]";
    		return str;
		}
    }
}
impl From<raylib_ffi::Font> for Font {
	fn from(value: raylib_ffi::Font) -> Self {
		Self {
			baseSize: value.baseSize,
			charsCount: value.glyphCount,
			charsPadding: value.glyphPadding,
			texture: Texture::from(value.texture),
			recs: value.recs,
			chars: value.glyphs,
		}
	}
}
impl Into<raylib_ffi::Font> for Font {
	fn into(self) -> raylib_ffi::Font {
		return raylib_ffi::Font {
			baseSize: self.baseSize,
			glyphCount: self.charsCount,
			glyphPadding: self.charsPadding,
			texture: self.texture.into(),
			recs: self.recs,
			glyphs: self.chars,
		};
	}
}


//= Procedures

impl GlyphInfo {

}

impl Font {

	/// Loading Font
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Font::from(raylib_ffi::LoadFont(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Font
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadFont(self.clone().into());
		}
	}

	/// Draw text using raylib_ffi::DrawText
	pub fn draw(&self, text: &str, posX: i32, posY: i32, fontSize: i32, color: raylib_ffi::Color) {
		unsafe {
			raylib_ffi::DrawText(
				raylib_ffi::rl_str!(text),
				posX,
				posY,
				fontSize,
				color,
			);
		}
	}
	/// Draw text using raylib_ffi::DrawTextPro
	pub fn draw_pro(&self, text: &str, position: Vector2, rotation: f32, fontSize: f32, spacing: f32, tint: raylib_ffi::Color) {
		unsafe {
			raylib_ffi::DrawTextPro(
				self.clone().into(),
				raylib_ffi::rl_str!(text),
				position.into(),
				Vector2::zero().into(),
				rotation,
				fontSize,
				spacing,
				tint,
			);
		}
	}

}