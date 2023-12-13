

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::{enums, rectangles::Rectangle, textures::Texture};
use std::{ffi::c_void, borrow::BorrowMut};


//= Structures

/// Color, 4 components, R8G8B8A8 (32bit)
pub struct Color {
	pub r: u8,		//* Color red value
	pub g: u8,		//* Color green value
	pub b: u8,		//* Color blue value
	pub a: u8,		//* Color alpha value
}

/// Image type, bpp always RGBA (32bit)
#[derive(Clone)]
pub struct Image {
	pub data:		*mut c_void,		// Image raw data
	pub width:		i32,				// Image base width
	pub height:		i32,				// Image base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:		enums::PixelFormat,	// Data format (PixelFormat type)
}
impl From<raylib_ffi::Image> for Image {
	fn from(value: raylib_ffi::Image) -> Self {
		Self {
			data: value.data,
			width: value.width,
			height: value.height,
			mipmaps: value.mipmaps,
			format: enums::PixelFormat::from_i32(value.format),
		}
	}
}
impl Into<raylib_ffi::Image> for Image {
	fn into(self) -> raylib_ffi::Image {
		return raylib_ffi::Image {
			data: self.data,
			width: self.width,
			height: self.height,
			mipmaps: self.mipmaps,
			format: self.format as i32,
		};
	}
}


//= Procedures

impl Image {

	//= Loading
	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Image::from(raylib_ffi::LoadImage(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Loading a texture from the image
	pub fn load_texture(&self) -> Texture {
		unsafe {
			return Texture::from(raylib_ffi::LoadTextureFromImage(self.clone().into()));
		}
	}
	/// Unloading Image
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadImage(self.clone().into());
		}
	}

	//= Generation
	/// Create new image of size and color
	pub fn gen_color(width: i32, height: i32, color: raylib_ffi::Color) -> Self {
		unsafe { return Image::from(raylib_ffi::GenImageColor(width, height, color)); }
	}

	//= Manipulation
	/// Create duplicate of Image
	pub fn copy(&self) -> Self {
		unsafe {
			return Image::from(raylib_ffi::ImageCopy(self.clone().into()));
		}
	}
	/// Create duplicate of portion of Image
	pub fn from_image(&self, rec: Rectangle) -> Self {
		unsafe {
			return Image::from(raylib_ffi::ImageFromImage(self.clone().into(), rec.into()));
		}
	}
	/// Resize image using nerarest neighbor
	pub fn resize_nn(&self, scale: i32) -> Self {
		unsafe {
			let mut ffiImg: raylib_ffi::Image = self.clone().into();
			let mutImage: &mut raylib_ffi::Image = ffiImg.borrow_mut();
			raylib_ffi::ImageResizeNN(mutImage, self.width * scale, self.height * scale);

			return Image::from(*mutImage);
		}
	}
	//
	pub fn draw_into(&self, src: Image, srcRec: Rectangle, dstRec: Rectangle, tint: raylib_ffi::Color) -> Self {
		unsafe {
			let mut ffiImg: raylib_ffi::Image = self.clone().into();
			let mutImage: *mut raylib_ffi::Image = ffiImg.borrow_mut();
			raylib_ffi::ImageDraw(mutImage, src.into(), srcRec.into(), dstRec.into(), tint);

			return Image::from(*mutImage);
		}
	}

}