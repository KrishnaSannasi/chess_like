use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;

pub trait App {
    fn render(&self, args: &RenderArgs, mut gl: GlGraphics) -> GlGraphics;
    fn update(&mut self, args: &UpdateArgs);
}

pub fn start(window: &mut Window, app: &mut App, mut g: GlGraphics) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(r) = e.render_args() {
            g = app.render(&r, g);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
