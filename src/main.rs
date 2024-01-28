

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use gnosis::{raylib, camera::Camera, world::{chunks::World, Coords, tiles::{Tile, TileType}}, graphics::Graphics, player::Player};


//= Procedures
 
fn main() {
	//* Raylib */
	//raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(1280,720,"Gnosis");
	//raylib::set_target_fps(80);

	let graphics = Graphics::new();
	let mut camera = Camera::new();
	let mut world = World::new();
	let mut player = Player::new();
	world.generate_test();

	while !raylib::window_should_close() {
		//* Update */
		player.update();
		camera.update(player.get_position());

		//* Draw */
		camera.begin_drawing();

		raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

		world.draw(&graphics, Coords::from(player.get_position()), camera);
		graphics.draw_tile_V(Tile{block:TileType::Test, draw:true}, player.get_position());

		camera.end_mode_2D();

		//* UI */
		raylib::draw_fps(0, 0);

		camera.end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}