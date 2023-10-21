

//= Allows


//= Imports
use gnosis::{raylib, units::player};


//= Procedures

fn main() {
    //* Raylib */
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(1280,720,"Gnosis");
	raylib::set_target_fps(80);

	let player = player::create_player();

	while !raylib::window_should_close() {
		//* Update */
		player.update();

		//* Draw */
		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}
