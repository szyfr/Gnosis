

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::HashMap;

//= Imports
use gnosis::{raylib::{self, structures::{Texture, Vector2}}, camera::Camera, tiles::Tile};


//= Procedures

fn main() {
    //* Raylib */
	//raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(1280,720,"Gnosis");
	raylib::set_target_fps(80);

	let mut camera = Camera::new();

	// TEMP
	let textureTest = Texture::load("data/test.png");

	let mut tile = Tile::new();
	tile.texture = textureTest;

	let mut tiles: HashMap<[i32;2], Tile> = HashMap::new();
	tiles.insert(Vector2{x:0.0,y:0.0}.into(), tile.clone());
	tiles.insert(Vector2{x:1.0,y:0.0}.into(), tile.clone());
	tiles.insert(Vector2{x:0.0,y:1.0}.into(), tile.clone());

	while !raylib::window_should_close() {
		//* Update */
		if raylib::button_down(raylib_ffi::enums::KeyboardKey::D as i32) {
			camera.position.x += 1.0;
		}
		if raylib::button_down(raylib_ffi::enums::KeyboardKey::A as i32) {
			camera.position.x -= 1.0;
		}
		if raylib::button_down(raylib_ffi::enums::KeyboardKey::W as i32) {
			camera.position.y -= 1.0;
		}
		if raylib::button_down(raylib_ffi::enums::KeyboardKey::S as i32) {
			camera.position.y += 1.0;
		}

		//* Draw */
		camera.begin_drawing();

		raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});
		//for y
		//for (pos, tile) in tiles.iter() {
		//	tile.draw(Vector2::from(*pos));
		//}

		camera.end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}
