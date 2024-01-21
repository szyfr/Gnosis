

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use gnosis::{raylib::{self, vectors::Vector3}, camera::Camera, world::{World, Coords}, graphics::Graphics};


//= Procedures

fn main() {
    //* Raylib */
	//raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(1280,720,"Gnosis");
	//raylib::set_target_fps(80);

	let graphics = Graphics::new();
	let mut camera = Camera::new();
	let mut world = World::new();
	world.generate_test();

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
		if raylib::button_pressed(raylib_ffi::enums::KeyboardKey::Q as i32) { world.rotation -= 1; }
		if raylib::button_pressed(raylib_ffi::enums::KeyboardKey::E as i32) { world.rotation += 1; }
		if world.rotation < 0 { world.rotation = 3; }
		if world.rotation > 3 { world.rotation = 0; }

		//* Draw */
		camera.begin_drawing();

		raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

		world.draw(&graphics, Coords{x:camera.position.x as i32,y:8,z:camera.position.y as i32});

		raylib::draw_fps(0, 0);

		camera.end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}
