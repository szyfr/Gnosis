

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod enums;
pub mod vectors;
pub mod matrixes;
pub mod rectangles;
pub mod images;
pub mod textures;
pub mod fonts;
pub mod shaders;
pub mod materials;
pub mod models;


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

pub fn window_should_close() -> bool {
	unsafe { return raylib_ffi::WindowShouldClose(); }
}

pub fn set_trace_log_level( logLevel : raylib_ffi::enums::TraceLogLevel ) {
	unsafe { raylib_ffi::SetTraceLogLevel(logLevel as i32); }
}

pub fn init_window(width: i32, height: i32, title: &str) {
	unsafe {
		raylib_ffi::InitWindow(
			width,
			height,
			raylib_ffi::rl_str!(title),
		);
	}
}
pub fn set_line_spacing(spacing : i32) {
	unsafe { raylib_ffi::SetTextLineSpacing(spacing); }
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

pub fn set_material_texture( material : *mut raylib_ffi::Material, mapType : raylib_ffi::enums::MaterialMapIndex, texture : raylib_ffi::Texture ) {
	unsafe { raylib_ffi::SetMaterialTexture(material, mapType as i32, texture) }
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
//			position:	camera.camPosition.into(),
//			target:		camera.position.into(),
//			up:			Vector3{x:0.0,y:1.0,z:0.0}.into(),
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
pub fn get_key_pressed() -> String {
	unsafe {
		let keyAsI32 = raylib_ffi::GetKeyPressed();

		match keyAsI32 {
			65 => {
				if !button_down(340) && !button_down(344) { return "a".to_string(); }
				else { return "A".to_string(); }
			},
			66 => {
				if !button_down(340) && !button_down(344) { return "b".to_string(); }
				else { return "B".to_string(); }
			},
			67 => {
				if !button_down(340) && !button_down(344) { return "c".to_string(); }
				else { return "C".to_string(); }
			},
			68 => {
				if !button_down(340) && !button_down(344) { return "d".to_string(); }
				else { return "D".to_string(); }
			},
			69 => {
				if !button_down(340) && !button_down(344) { return "e".to_string(); }
				else { return "E".to_string(); }
			},
			70 => {
				if !button_down(340) && !button_down(344) { return "f".to_string(); }
				else { return "F".to_string(); }
			},
			71 => {
				if !button_down(340) && !button_down(344) { return "g".to_string(); }
				else { return "G".to_string(); }
			},
			72 => {
				if !button_down(340) && !button_down(344) { return "h".to_string(); }
				else { return "H".to_string(); }
			},
			73 => {
				if !button_down(340) && !button_down(344) { return "i".to_string(); }
				else { return "I".to_string(); }
			},
			74 => {
				if !button_down(340) && !button_down(344) { return "j".to_string(); }
				else { return "J".to_string(); }
			},
			75 => {
				if !button_down(340) && !button_down(344) { return "k".to_string(); }
				else { return "K".to_string(); }
			},
			76 => {
				if !button_down(340) && !button_down(344) { return "l".to_string(); }
				else { return "L".to_string(); }
			},
			77 => {
				if !button_down(340) && !button_down(344) { return "m".to_string(); }
				else { return "M".to_string(); }
			},
			78 => {
				if !button_down(340) && !button_down(344) { return "n".to_string(); }
				else { return "N".to_string(); }
			},
			79 => {
				if !button_down(340) && !button_down(344) { return "o".to_string(); }
				else { return "O".to_string(); }
			},
			80 => {
				if !button_down(340) && !button_down(344) { return "p".to_string(); }
				else { return "P".to_string(); }
			},
			81 => {
				if !button_down(340) && !button_down(344) { return "q".to_string(); }
				else { return "Q".to_string(); }
			},
			82 => {
				if !button_down(340) && !button_down(344) { return "r".to_string(); }
				else { return "R".to_string(); }
			},
			83 => {
				if !button_down(340) && !button_down(344) { return "s".to_string(); }
				else { return "S".to_string(); }
			},
			84 => {
				if !button_down(340) && !button_down(344) { return "t".to_string(); }
				else { return "T".to_string(); }
			},
			85 => {
				if !button_down(340) && !button_down(344) { return "u".to_string(); }
				else { return "U".to_string(); }
			},
			86 => {
				if !button_down(340) && !button_down(344) { return "v".to_string(); }
				else { return "V".to_string(); }
			},
			87 => {
				if !button_down(340) && !button_down(344) { return "w".to_string(); }
				else { return "W".to_string(); }
			},
			88 => {
				if !button_down(340) && !button_down(344) { return "x".to_string(); }
				else { return "X".to_string(); }
			},
			89 => {
				if !button_down(340) && !button_down(344) { return "y".to_string(); }
				else { return "Y".to_string(); }
			},
			90 => {
				if !button_down(340) && !button_down(344) { return "z".to_string(); }
				else { return "Z".to_string(); }
			},
			32 => return " ".to_string(),
			259 => return ".".to_string(),
			_ => return "".to_string(),
		}
	}
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

pub fn init_audio_device() {
	unsafe { raylib_ffi::InitAudioDevice() }
}
pub fn close_audio_device() {
	unsafe { raylib_ffi::CloseAudioDevice() }
}

pub fn play_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::PlaySound(sound) }
}
pub fn stop_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::StopSound(sound) }
}
pub fn pause_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::PauseSound(sound) }
}
pub fn resume_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::ResumeSound(sound) }
}
pub fn is_sound_playing( sound: raylib_ffi::Sound ) -> bool {
	unsafe { return raylib_ffi::IsSoundPlaying(sound) }
}
pub fn set_sound_volume( sound: raylib_ffi::Sound, volume: f32 ) {
	unsafe { raylib_ffi::SetSoundVolume(sound, volume) }
}
pub fn load_sound( fileName: &str ) -> raylib_ffi::Sound {
	unsafe { return raylib_ffi::LoadSound(raylib_ffi::rl_str!(fileName)) }
}
pub fn unload_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::UnloadSound(sound) }
}

pub fn play_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::PlayMusicStream(music) }
}
pub fn stop_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::StopMusicStream(music) }
}
pub fn pause_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::PauseMusicStream(music) }
}
pub fn resume_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::ResumeMusicStream(music) }
}
pub fn is_music_playing( music: raylib_ffi::Music ) -> bool {
	unsafe { return raylib_ffi::IsMusicStreamPlaying(music) }
}
pub fn set_music_volume( music: raylib_ffi::Music, volume: f32 ) {
	unsafe { raylib_ffi::SetMusicVolume(music, volume) }
}
pub fn load_music( fileName: &str ) -> raylib_ffi::Music {
	unsafe { return raylib_ffi::LoadMusicStream(raylib_ffi::rl_str!(fileName)) }
}
pub fn unload_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::UnloadMusicStream(music) }
}
pub fn is_music_ready( music: raylib_ffi::Music ) -> bool {
	unsafe { return raylib_ffi::IsMusicReady(music) }
}
pub fn update_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::UpdateMusicStream(music) }
}

pub fn load_shader(vsFileName: &str, fsFileName: &str) -> raylib_ffi::Shader {
	unsafe { return raylib_ffi::LoadShader(raylib_ffi::rl_str!(vsFileName), raylib_ffi::rl_str!(fsFileName)); }
}
pub fn get_shader_location(shader: raylib_ffi::Shader, uniformName: &str) -> i32 {
	unsafe { return raylib_ffi::GetShaderLocation(shader, raylib_ffi::rl_str!(uniformName)); }
}
pub fn set_shader_value(shader: raylib_ffi::Shader, locIndex: i32, value: *const std::ffi::c_void, uniformType: enums::ShaderUniformDataType ) {
	unsafe { raylib_ffi::SetShaderValue(shader, locIndex, value, uniformType as i32) }
}

pub fn end_texture_mode() {
	unsafe { raylib_ffi::EndTextureMode() }
}
pub fn draw_texture_rec(texture: raylib_ffi::Texture, source: raylib_ffi::Rectangle , position: raylib_ffi::Vector2 , tint: raylib_ffi::Color) {
	unsafe { raylib_ffi::DrawTextureRec(texture, source, position, tint); }
}

pub fn begin_shader_mode(shader: raylib_ffi::Shader) {
	unsafe { raylib_ffi::BeginShaderMode(shader) }
}
pub fn end_shader_mode() {
	unsafe { raylib_ffi::EndShaderMode(); }
}

extern "C" {
	pub fn rlLoadTexture(
		data: i32,
		width: i32,
		height: i32,
		format: i32,
		mipmapCount: i32,
	) -> u32;
}
extern "C" {
	pub fn rlLoadTextureDepth(
		width: i32,
		height: i32,
		useRenderBuffer: bool,
	) -> u32;
}
extern "C" {
	pub fn rlLoadFramebuffer(
		width: i32,
		height: i32,
	) -> u32;
}
extern "C" {
	pub fn rlEnableFramebuffer(
		id: u32,
	);
}
extern "C" {
	pub fn rlFramebufferAttach(
		fboId: u32,
		texId: u32,
		attachType: i32,
		texType: i32,
		mipLevel: i32,
	);
}
extern "C" {
	pub fn rlFramebufferComplete(
		id: u32,
	) -> bool;
}
extern "C" {
	pub fn rlDisableFramebuffer();
}
pub fn load_render_texture_depth_tex(width: i32, height: i32) -> raylib_ffi::RenderTexture2D {
	unsafe {
		let target: raylib_ffi::RenderTexture2D = raylib_ffi::RenderTexture2D{
			id: rlLoadFramebuffer(width, height),
			texture: raylib_ffi::Texture {
				id: rlLoadTexture(0, width, height, enums::PixelFormat::UncompressedR8g8b8 as i32, 1),
				width: width,
				height: height,
				mipmaps: 1,
				format: enums::PixelFormat::UncompressedR8g8b8 as i32,
			},
			depth: raylib_ffi::Texture {
				id: rlLoadTextureDepth(width, height, false),
				width: width,
				height: height,
				mipmaps: 1,
				format: 19,
			}
		};

		rlEnableFramebuffer(target.id);
		rlFramebufferAttach(
			target.id as u32,
			target.texture.id as u32,
			enums::RlFramebufferAttachType::RlAttachmentColorChannel0 as i32,
			enums::RlFramebufferAttachTextureType::RlAttachmentTexture2d as i32,
			0,
		);
		rlFramebufferAttach(
			target.id as u32,
			target.depth.id as u32,
			enums::RlFramebufferAttachType::RlAttachmentDepth as i32,
			enums::RlFramebufferAttachTextureType::RlAttachmentTexture2d as i32,
			0,
		);

		if rlFramebufferComplete(target.id) { print!("Fucking hell this is hard... But it worked?\n") }
		else { print!("It fucking failed. Fuck\n"); }

		rlDisableFramebuffer();

		return target;
	}
}