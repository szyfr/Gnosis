

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Structures


//= Procedures
pub fn begin_drawing() {
	unsafe { raylib_ffi::BeginDrawing(); }
}
pub fn end_drawing() {
	unsafe { raylib_ffi::EndDrawing(); }
}

pub fn clear_background( color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::ClearBackground(color); }
}

pub fn draw_text( text : *const std::os::raw::c_char, posX : i32, posY : i32, fontSize : i32, color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawText(text, posX, posY, fontSize, color); }
}
pub fn draw_text_pro( font : raylib_ffi::Font, text : &str, position : raylib_ffi::Vector2, origin : raylib_ffi::Vector2, rotation : f32, fontSize : f32, spacing : f32, tint : raylib_ffi::Color ) {
	unsafe {
		raylib_ffi::DrawTextPro(
			font,
			raylib_ffi::rl_str!(text),
			position,
			origin,
			rotation,
			fontSize,
			spacing,
			tint,
		);
	}
}

pub fn window_should_close() -> bool {
	unsafe { return raylib_ffi::WindowShouldClose(); }
}

pub fn set_trace_log_level( logLevel : raylib_ffi::enums::TraceLogLevel ) {
	unsafe { raylib_ffi::SetTraceLogLevel(logLevel as i32); }
}

pub fn init_window( width : i32, height : i32, title : &str ) {
	unsafe {
		raylib_ffi::InitWindow(
			width,
			height,
			raylib_ffi::rl_str!(title),
		);
	}
}
pub fn close_window() {
	unsafe { raylib_ffi::CloseWindow(); }
}
pub fn is_window_ready() -> bool {
	unsafe { return raylib_ffi::IsWindowReady(); }
}

pub fn set_target_fps( fps : i32 ) {
	unsafe { raylib_ffi::SetTargetFPS(fps); }
}
pub fn draw_fps( x : i32, y : i32 ) {
	unsafe { raylib_ffi::DrawFPS(x, y); }
}

pub fn set_exit_key( key : raylib_ffi::enums::KeyboardKey ) {
	unsafe { raylib_ffi::SetExitKey(key as i32); }
}

pub fn load_font( filename : &str ) -> raylib_ffi::Font {
	unsafe { return raylib_ffi::LoadFont(raylib_ffi::rl_str!(filename)); }
}
pub fn load_image( filename : &str ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::LoadImage(raylib_ffi::rl_str!(filename)); }
}
pub fn image_from_image( image : raylib_ffi::Image, rec : raylib_ffi::Rectangle ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::ImageFromImage(image, rec); }
}
pub fn image_copy( image : raylib_ffi::Image ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::ImageCopy(image); }
}
pub fn unload_image( image : raylib_ffi::Image ) {
	unsafe { raylib_ffi::UnloadImage(image) }
}
pub fn load_texture( filename : &str ) -> raylib_ffi::Texture {
	unsafe { return raylib_ffi::LoadTexture(raylib_ffi::rl_str!(filename)) }
}
pub fn load_texture_from_image( img : raylib_ffi::Image ) -> raylib_ffi::Texture {
	unsafe { return raylib_ffi::LoadTextureFromImage(img); }
}
pub fn draw_texture( texture : raylib_ffi::Texture, posX : i32, posY : i32, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawTexture(texture, posX, posY, tint) }
}
pub fn draw_texture_npatch( texture : raylib_ffi::Texture, dest : raylib_ffi::Rectangle, origin : raylib_ffi::Vector2, rotation : f32, tint : raylib_ffi::Color ) {
	let nPatchInfo = raylib_ffi::NPatchInfo {
		source: raylib_ffi::Rectangle {
			x: 0.0,
			y: 0.0,
			width: texture.width as f32,
			height: texture.height as f32,
		},
		left: texture.width / 3,
		top: texture.height / 3,
		right: texture.width / 3,
		bottom: texture.height / 3,
		layout: raylib_ffi::enums::NPatchLayout::NinePatch as i32,
	};
	unsafe { raylib_ffi::DrawTextureNPatch(texture, nPatchInfo, dest, origin, rotation, tint); }
}
pub fn image_resize_nn( image : &mut raylib_ffi::Image, scale : i32 ) {
	unsafe { raylib_ffi::ImageResizeNN(image, image.width * scale, image.height * scale); }
}

pub fn set_material_texture( material : &mut raylib_ffi::Material, mapType : raylib_ffi::enums::MaterialMapIndex, texture : raylib_ffi::Texture ) {
	unsafe { raylib_ffi::SetMaterialTexture(material, mapType as i32, texture) }
}

pub fn load_model( filename : &str ) -> raylib_ffi::Model {
	unsafe { return raylib_ffi::LoadModel(raylib_ffi::rl_str!(filename)) }
}

pub fn draw_mesh( mesh : *mut raylib_ffi::Mesh, material : &raylib_ffi::Material, transform : raylib_ffi::Matrix ) {
	unsafe {
		raylib_ffi::DrawMesh(*mesh, *material, transform);
	}
}
pub fn draw_model( model : raylib_ffi::Model, position : raylib_ffi::Vector3, scale : f32, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawModel(model, position, scale, tint); }
}
pub fn draw_model_ex( model : raylib_ffi::Model, position : raylib_ffi::Vector3, rotationAxis : raylib_ffi::Vector3, rotationAngle : f32, scale : raylib_ffi::Vector3, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawModelEx(model, position, rotationAxis, rotationAngle, scale, tint); }
}

pub fn load_default_material() -> raylib_ffi::Material {
	unsafe { return raylib_ffi::LoadMaterialDefault(); }
}
pub fn unload_material( material : raylib_ffi::Material ) {
	unsafe { raylib_ffi::UnloadMaterial(material); }
}

pub fn get_frame_time() -> f32 {
	unsafe { return raylib_ffi::GetFrameTime(); }
}

//pub fn begin_3d_mode( camera : &Camera ) {
//	unsafe {
//		let rlCamera = raylib_ffi::Camera3D{
//			position:	camera.camPosition,
//			target:		camera.position,
//			up:			Vector3{x:0.0,y:1.0,z:0.0},
//			fovy:		camera.fovy,
//			projection:	raylib_ffi::enums::CameraProjection::Perspective as i32,
//		};
//
//		raylib_ffi::BeginMode3D(rlCamera);
//	}
//}
pub fn end_3d_mode() {
	unsafe { raylib_ffi::EndMode3D(); }
}

pub fn draw_grid( slices : i32, spacing : f32 ) {
	unsafe { raylib_ffi::DrawGrid(slices, spacing); }
}

pub fn button_pressed( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyPressed(key ); }
}
pub fn button_down( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyDown(key ); }
}
pub fn button_released( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyReleased(key); }
}
pub fn button_up( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyUp(key); }
}

pub fn mouse_button_pressed( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonPressed(key); }
}
pub fn mouse_button_down( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonDown(key); }
}
pub fn mouse_button_released( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonReleased(key); }
}
pub fn mouse_button_up( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonUp(key); }
}

pub fn gamepad_available( gamepad : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadAvailable(gamepad); }
}
pub fn gamepad_button_pressed( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonPressed(gamepad, key); }
}
pub fn gamepad_button_down( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonDown(gamepad, key); }
}
pub fn gamepad_button_released( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonReleased(gamepad, key); }
}
pub fn gamepad_button_up( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonUp(gamepad, key); }
}

