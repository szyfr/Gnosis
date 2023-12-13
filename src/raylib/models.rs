

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures

use super::{vectors::{Vector3, Quaternion}, matrixes::Matrix, enums, textures::Texture};

/// Transformation properties
pub struct Transform {
	translation:	Vector3,
	rotation:		Quaternion,
	scale:			Vector3,
}

/// Model type
#[derive(Clone)]
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
impl From<raylib_ffi::Model> for Model {
	fn from(value: raylib_ffi::Model) -> Self {
		Self {
			transform: Matrix::from(value.transform),
			meshCount: value.meshCount,
			materialCount: value.materialCount,
			meshes: value.meshes,
			materials: value.materials,
			meshMaterial: value.meshMaterial,

			boneCount: value.boneCount,
			bones: value.bones,
			bindPose: value.bindPose,
		}
	}
}
impl Into<raylib_ffi::Model> for Model {
	fn into(self) -> raylib_ffi::Model {
		return raylib_ffi::Model {
			transform: self.transform.into(),
			meshCount: self.meshCount,
			materialCount: self.materialCount,
			meshes: self.meshes,
			materials: self.materials,
			meshMaterial: self.meshMaterial,

			boneCount: self.boneCount,
			bones: self.bones,
			bindPose: self.bindPose,
		};
	}
}



//= Procedures

impl Model {

	/// Loading Model
	pub fn load(fileName: &str) -> Model {
		unsafe {
			return Model::from(raylib_ffi::LoadModel(raylib_ffi::rl_str!(fileName)));
		}
	}

	/// Set material texture
	pub fn set_material_texture(&mut self, texture: Texture) -> &mut Self {
		unsafe {
			raylib_ffi::SetMaterialTexture(self.materials, enums::MaterialMapIndex::ALBEDO as i32, texture.into());
		}

		return self;
	}

	/// Draw text using raylib_ffi::DrawModel
	pub fn draw(&self, position: Vector3, scale: f32, tint: raylib_ffi::Color) -> &Self {
		unsafe {
			raylib_ffi::DrawModel(
				self.clone().into(),
				position.into(),
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
				self.clone().into(),
				position.into(),
				rotationAxis.into(),
				rotationAngle,
				scale.into(),
				tint,
			);
			return self;
		}
	}

}