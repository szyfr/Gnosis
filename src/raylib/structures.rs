

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::enums;
use std::{ffi::c_void, borrow::BorrowMut};


//= Structures

/// Vector2 type
#[derive(Copy, Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

/// Vector3 type
#[derive(Copy, Clone)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
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

/// Matrix, 4x4 components, column major, OpenGL style, right handed
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32, //* Matrix first row  (4 components)
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32, //* Matrix second row (4 components)
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32, //* Matrix third row  (4 components)
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32, //* Matrix fourth row (4 components)
}

/// Rectangle type
#[derive(Copy, Clone)]
pub struct Rectangle {
	pub x: f32,			//* Rectangle top-left corner position x
	pub y: f32,			//* Rectangle top-left corner position y
	pub width: f32,		//* Rectangle width
	pub height: f32,	//* Rectangle height
}

/// Color, 4 components, R8G8B8A8 (32bit)
pub struct Color {
	pub r: u8,		//* Color red value
	pub g: u8,		//* Color green value
	pub b: u8,		//* Color blue value
	pub a: u8,		//* Color alpha value
}

/// Image type, bpp always RGBA (32bit)
pub struct Image {
	pub data:		*mut c_void,		// Image raw data
	pub width:		i32,				// Image base width
	pub height:		i32,				// Image base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:		enums::PixelFormat,	// Data format (PixelFormat type)
}

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
/// Texture2D type, same as Texture
pub struct Texture2D (Texture);
/// TextureCubemap type, same as Texture
pub struct TextureCubeMap (Texture);

/// Font character info
pub struct GlyphInfo {
	pub value: i32,
	pub offsetX: i32,
	pub offsetY: i32,
	pub advanceX: i32,
	pub image: Image,
}

/// Font type, includes texture and charSet array data
pub struct Font {
	pub baseSize: i32,
	pub charsCount: i32,
	pub charsPadding: i32,
	pub texture: Texture,
	pub recs: *mut raylib_ffi::Rectangle,
	pub chars: *mut raylib_ffi::GlyphInfo,
}

/// Shader type (generic)
pub struct Shader {
	id:		u32,
	locs:	*mut i32,
}

/// Material texture map
pub struct MaterialMap {
	texture:	Texture,
	color:		raylib_ffi::Color,
	value:		f32,
}

/// Material type (generic)
pub struct Material {
	shader: Shader,
	maps: *mut raylib_ffi::MaterialMap,
	params: [f32;4],
}

/// Transformation properties
pub struct Transform {
	translation:	Vector3,
	rotation:		Quaternion,
	scale:			Vector3,
}

/// Model type
pub struct Model {
	pub transform: Matrix,

	pub meshCount: i32,
	pub materialCount: i32,
	pub meshes: *mut raylib_ffi::Mesh,
	pub materials: *mut raylib_ffi::Material,
	pub meshMaterial: *mut i32,

	// Animation data
	pub boneCount:	i32,
	pub bones:		*mut raylib_ffi::BoneInfo,
	pub bindPose:	*mut raylib_ffi::Transform,
}


//= Procedures

impl Vector2 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector2 {
		return Vector2 { x: 0.0, y: 0.0 }
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Vector2 {
		return raylib_ffi::Vector2 { x: self.x, y: self.y };
	}

}

impl Vector3 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector3 {
		return Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Vector3 {
		return raylib_ffi::Vector3 { x: self.x, y: self.y, z: self.z };
	}

}

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

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Rectangle {
		return raylib_ffi::Rectangle{
			x: 		self.x,
			y:		self.y,
			width:	self.width,
			height:	self.height,
		};
	}
	/// Converting to Rectangle
	pub fn from_ffi(rectangle: raylib_ffi::Rectangle) -> Self {
		return Rectangle {
			x:		rectangle.x,
			y:		rectangle.y,
			width:	rectangle.width,
			height:	rectangle.height,
		}
	}

}

impl Matrix {
	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Matrix {
		return raylib_ffi::Matrix {
			m0: self.m0, m4: self.m4,  m8: self.m8,  m12: self.m12,
			m1: self.m1, m5: self.m5,  m9: self.m9,  m13: self.m13,
			m2: self.m2, m6: self.m6, m10: self.m10, m14: self.m14,
			m3: self.m3, m7: self.m7, m11: self.m11, m15: self.m15,
		}
	}
	/// Converting to Matrix
	pub fn from_ffi(matrix: raylib_ffi::Matrix) -> Self {
		return Matrix {
			m0: matrix.m0, m4: matrix.m4,  m8: matrix.m8,  m12: matrix.m12,
			m1: matrix.m1, m5: matrix.m5,  m9: matrix.m9,  m13: matrix.m13,
			m2: matrix.m2, m6: matrix.m6, m10: matrix.m10, m14: matrix.m14,
			m3: matrix.m3, m7: matrix.m7, m11: matrix.m11, m15: matrix.m15,
		}
	}
}

impl Image {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::LoadImage(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Image
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadImage(self.to_ffi());
		}
	}

	/// Create duplicate of Image
	pub fn copy(&self) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::ImageCopy(self.to_ffi()));
		}
	}
	/// Create duplicate of portion of Image
	pub fn from_image(&self, rec: Rectangle) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::ImageFromImage(self.to_ffi(), rec.to_ffi()));
		}
	}

	/// Resize image using nerarest neighbor
	pub fn resize_nn(&self, scale: i32) -> Self {
		unsafe {
			let mut ffiImg = self.to_ffi();
			let mutImage: &mut raylib_ffi::Image = ffiImg.borrow_mut();
			raylib_ffi::ImageResizeNN(mutImage, self.width * scale, self.height * scale);

			return Image::from_ffi(*mutImage);
		}
	}

	/// Loading a texture from the image
	pub fn load_texture(&self) -> Texture {
		unsafe {
			return Texture::from_ffi(raylib_ffi::LoadTextureFromImage(self.to_ffi()));
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Image {
		return raylib_ffi::Image{
			data:		self.data,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
	/// Converting to Image
	pub fn from_ffi(image: raylib_ffi::Image) -> Self {
		return Image {
			data:		image.data,
			width:		image.width,
			height:		image.height,
			mipmaps:	image.mipmaps,
			format:		enums::PixelFormat::from_i32(image.format),
		}
	}

}

impl Texture {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Texture::from_ffi(raylib_ffi::LoadTexture(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Texture
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadTexture(self.to_ffi());
		}
	}

	/// Draw texture using raylib_ffi::DrawTexture
	pub fn draw(&self, posX: i32, posY: i32) -> Self {
		unsafe {
			raylib_ffi::DrawTexture(self.to_ffi(), posX, posY, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureV
	pub fn draw_v(&self, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureV(self.to_ffi(), position.to_ffi(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureEX
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTextureEx(self.to_ffi(), position.to_ffi(), rotation, scale, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureRec
	pub fn draw_rec(&self, source: Rectangle, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureRec(self.to_ffi(), source.to_ffi(), position.to_ffi(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTexturePro
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, rotation: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTexturePro(self.to_ffi(), source.to_ffi(), dest.to_ffi(), self.origin.to_ffi(), rotation, self.tint);

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
			raylib_ffi::DrawTextureNPatch(self.to_ffi(), nPatchInfo, dest.to_ffi(), self.origin.to_ffi(), rotation, self.tint);

			return self.clone();
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Texture {
		return raylib_ffi::Texture{
			id:			self.id,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
	/// Converting to Texture
	pub fn from_ffi(texture: raylib_ffi::Texture) -> Self {
		return Texture {
			id:			texture.id,
			width:		texture.width,
			height:		texture.height,
			mipmaps:	texture.mipmaps,
			format:		enums::PixelFormat::from_i32(texture.format),
			origin:		Vector2 { x: 0.0, y: 0.0 },
			tint:		raylib_ffi::colors::WHITE,
		}
	}
}

impl GlyphInfo {
	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::GlyphInfo {
		return raylib_ffi::GlyphInfo {
			value:		self.value,
			offsetX:	self.offsetX,
			offsetY:	self.offsetY,
			advanceX:	self.advanceX,
			image:		self.image.to_ffi(),
		}
	}
	/// Converting to GlyphInfo
	pub fn from_ffi(glyphInfo: raylib_ffi::GlyphInfo) -> Self {
		return GlyphInfo {
			value:		glyphInfo.value,
			offsetX:	glyphInfo.offsetX,
			offsetY:	glyphInfo.offsetY,
			advanceX:	glyphInfo.advanceX,
			image:		Image::from_ffi(glyphInfo.image),
		}
	}
}

impl Font {
	/// Loading Font
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Font::from_ffi(raylib_ffi::LoadFont(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Font
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadFont(self.to_ffi());
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
				self.to_ffi(),
				raylib_ffi::rl_str!(text),
				position.to_ffi(),
				Vector2::zero().to_ffi(),
				rotation,
				fontSize,
				spacing,
				tint,
			);
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Font {
		return raylib_ffi::Font {
			baseSize:		self.baseSize,
			glyphCount:		self.charsCount,
			glyphPadding:	self.charsPadding,
			texture:		self.texture.to_ffi(),
			recs:			self.recs,
			glyphs:			self.chars,
		}
	}
	/// Converting to Texture
	pub fn from_ffi(font: raylib_ffi::Font) -> Self {
		return Font {
			baseSize:		font.baseSize,
			charsCount:		font.glyphCount,
			charsPadding:	font.glyphPadding,
			texture:		Texture::from_ffi(font.texture),
			recs:			font.recs,
			chars:			font.glyphs,
		}
	}
}

impl Model {
	/// Loading Model
	pub fn load(fileName: &str) -> Model {
		unsafe {
			return Model::from_ffi(raylib_ffi::LoadModel(raylib_ffi::rl_str!(fileName)));
		}
	}

	/// Set material texture
	pub fn set_material_texture(&mut self, texture: Texture) -> &mut Self {
		unsafe {
			//(*self.materials).
			raylib_ffi::SetMaterialTexture(self.materials, enums::MaterialMapIndex::ALBEDO as i32, texture.to_ffi());
		}

		return self;
	}

	/// Draw text using raylib_ffi::DrawModel
	pub fn draw(&self, position: Vector3, scale: f32, tint: raylib_ffi::Color) -> &Self {
		unsafe {
			raylib_ffi::DrawModel(
				self.to_ffi(),
				position.to_ffi(),
				scale,
				tint,
			);
			return self;
		}
	}

	/// Draw text using raylib_ffi::DrawModelEx
	pub fn draw_ex(&self, position: Vector3, rotationAxis: Vector3, rotationAngle: f32, scale: Vector3, tint: raylib_ffi::Color) -> &Self {
		unsafe {
			raylib_ffi::DrawModelEx(
				self.to_ffi(),
				position.to_ffi(),
				rotationAxis.to_ffi(),
				rotationAngle,
				scale.to_ffi(),
				tint,
			);
			return self;
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Model {
		return raylib_ffi::Model {
			transform:		self.transform.to_ffi(),
			meshCount:		self.meshCount,
			materialCount:	self.materialCount,
			meshes:			self.meshes,
			materials:		self.materials,
			meshMaterial:	self.meshMaterial,
			boneCount:		self.boneCount,
			bones:			self.bones,
			bindPose:		self.bindPose,
		}
	}
	/// Converting to Model
	pub fn from_ffi(model: raylib_ffi::Model) -> Self {
		return Model {
			transform:		Matrix::from_ffi(model.transform),
			meshCount:		model.meshCount,
			materialCount:	model.materialCount,
			meshes:			model.meshes,
			materials:		model.materials,
			meshMaterial:	model.meshMaterial,
			boneCount:		model.boneCount,
			bones:			model.bones,
			bindPose:		model.bindPose,
		}
	}
}