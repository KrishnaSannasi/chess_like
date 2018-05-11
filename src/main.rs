extern crate piston;
extern crate piston_window;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate find_folder;

use piston::window::WindowSettings;
use opengl_graphics::OpenGL;
use piston_window::PistonWindow;

mod gui;
mod game;

use game::Game;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create an Glutin window.
    let window: PistonWindow = WindowSettings::new(
            "chess-like",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    gui::start(window, Game::new(10 ,10));
}
